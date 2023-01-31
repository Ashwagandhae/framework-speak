use anyhow::Result;
use csv::Writer;
use itertools::Itertools;
use radix_trie::Trie;
use reqwest::blocking::get;
use serde_json::Value;

const URL: &str = "https://bestofjs-static-api.vercel.app/projects.json";
type CmuTrie = Trie<String, String>;

fn guess_pronounce(word: &str) -> String {
    word.chars()
        .map(|c| match c {
            'A' => "AE",
            'E' => "IY",
            'I' => "AY",
            'O' => "OW",
            'U' => "UW",
            'B' => "B",
            'C' => "K",
            'D' => "D",
            'F' => "F",
            'G' => "G",
            'H' => "HH",
            'J' => "JH",
            'K' => "K",
            'L' => "L",
            'M' => "M",
            'N' => "N",
            'P' => "P",
            'Q' => "K",
            'R' => "R",
            'S' => "S",
            'T' => "T",
            'V' => "V",
            'W' => "W",
            'X' => "K S",
            'Y' => "Y",
            'Z' => "Z",
            _ => "",
        })
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .flatten()
        .join(" ")
}

fn get_pronounce(word: &str, trie: &CmuTrie) -> String {
    trie.get(word)
        .map(|word| word.to_owned())
        .unwrap_or_else(|| guess_pronounce(word))
        .to_owned()
}
fn split_into_words(name: &str) -> Vec<String> {
    // detect if its just a word or has camel case, snake case, kebab case, has spaces, has punctuation
    // split by whitespace and punctuation handles:
    // - snake case
    // - kebab case
    // - punctuation
    // - spaces
    name.split(|c: char| c.is_ascii_whitespace() || c.is_ascii_punctuation())
        .map(|part| {
            // check if camel case
            let uppercase = part.chars().filter(char::is_ascii_uppercase).count();
            let is_camel_case = (part
                .chars()
                .nth(0)
                .map(|c| c.is_ascii_uppercase())
                .unwrap_or(false)
                && uppercase > 2)
                || uppercase > 1;
            if is_camel_case {
                // split by uppercase, while keeping the uppercase
                let mut ret = Vec::new();
                let mut current = String::new();
                for c in part.chars() {
                    if c.is_ascii_uppercase() {
                        if !current.is_empty() {
                            ret.push(current);
                        }
                        current = String::new();
                    }
                    current.push(c);
                }
                if !current.is_empty() {
                    ret.push(current);
                }
                ret
            } else {
                vec![part.to_owned()]
            }
        })
        .flatten()
        .filter(|part| !part.is_empty())
        .collect()
}

fn get_framework_pronounce(name: &str, trie: &CmuTrie) -> String {
    split_into_words(name)
        .iter()
        .map(|word| word.to_ascii_uppercase())
        .map(|word| get_pronounce(&word, trie))
        .join(" ")
}

pub fn get_frameworks(out_path: &str, trie: CmuTrie) -> Result<()> {
    let resp = get(URL)?;
    let frameworks: Value = serde_json::from_str(&resp.text()?)?;
    let mut sorted = frameworks["projects"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|project| {
            project
                .get("tags")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .any(|value| {
                    let s = value.as_str().unwrap();
                    s == "framework" || s == "nodejs-framework" || s == "fullstack"
                })
        })
        .map(|project| {
            (
                project["stars"].as_u64().unwrap(),
                project["name"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();

    // remove duplicates (such as Vue.js 3 and Vue.js 2 -> Vue and Vue)
    sorted.sort_by(|(_, name), (_, other_name)| name.cmp(other_name));
    sorted = sorted
        .into_iter()
        .coalesce(|(stars, name), (other_stars, other_name)| {
            if name == other_name {
                Ok((stars + other_stars, name))
            } else {
                Err(((stars, name), (other_stars, other_name)))
            }
        })
        .collect();
    // sort by number of stars
    sorted.sort_by(|(stars, _), (other_stars, _)| other_stars.cmp(stars));
    let ret: Vec<(String, String)> = sorted
        .iter()
        .map(|(_, name)| name.to_owned())
        .map(|name| {
            name.split(|c: char| c.is_ascii_whitespace())
                // remove version number from name
                .filter(|s| !s.chars().all(char::is_numeric))
                // remove .js extension
                .map(|s| s.trim_end_matches(".js"))
                // remove .mjs extension
                .map(|s| s.trim_end_matches(".mjs"))
                // remove JS suffix
                .map(|s| s.trim_end_matches("JS"))
                .map(|s| s.trim_end_matches("js"))
                // remove extra whitespace
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .map(|name| (name.clone(), get_framework_pronounce(&name, &trie)))
        .collect();
    // write to csv
    let mut wtr = Writer::from_path(out_path)?;
    for (name, pronounce) in ret {
        wtr.write_record(&[name, pronounce])?;
    }
    Ok(())
}
