use anyhow::Result;
use csv::Writer;
use itertools::Itertools;
use radix_trie::Trie;
use reqwest::blocking::get;
use serde_json::Value;

const URL: &str = "https://bestofjs-static-api.vercel.app/projects.json";
type CmuTrie = Trie<String, String>;

fn special_word(name: &str) -> Option<&str> {
    Some(match name {
        "dva" => "DVA",
        "Qwik" => "quick",
        "tinyhttp" => "tiny http",
        "Strapi" => "strap e",
        "Actionhero" => "action hero",
        "htmx" => "HTMX",
        "Stylify" => "style ify",
        "$mol" => "mole",
        _ => return None,
    })
}
fn special_pronounce(name: &str) -> Option<String> {
    Some(
        match name {
            "NUXT" => "N AH K S T",
            "BULMA" => "B AH L M AH",
            "KOA" => "K OW AA",
            "WINDI" => "W IH N D IY",
            "TAILWIND" => "T EY L W IH N D",
            "VITE" => "V IY T",
            _ => return None,
        }
        .to_owned(),
    )
}

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
        .join(" ")
}

const SUFFIX: &[(&str, &str)] = &[
    ("IFY", "IH F AY"),
    ("KIT", "K IH T"),
    ("DOM", "D AA M"),
    ("IZE", "AY Z"),
    ("APP", "AE P"),
    ("STACK", "S T AE K"),
];

fn get_suffix(s: &str) -> Option<(&str, &str)> {
    for (suffix, pronounce) in SUFFIX.iter() {
        if let Some(stripped) = s.strip_suffix(suffix) {
            return Some((stripped, pronounce));
        }
    }
    None
}
fn get_pronounce_rec(word: &str, trie: &CmuTrie, suffix: bool) -> String {
    special_pronounce(word)
        .or_else(|| trie.get(word).map(|s| s.to_owned()))
        .or_else(|| {
            if !suffix {
                return None;
            }
            get_suffix(word).map(|(word, pronounce)| {
                format!("{} {}", get_pronounce_rec(word, trie, false), pronounce)
            })
        })
        .unwrap_or_else(|| guess_pronounce(word))
}
fn get_pronounce(word: &str, trie: &CmuTrie) -> String {
    get_pronounce_rec(word, trie, true)
}
fn split_into_words(name: &str) -> Vec<String> {
    // detect if its just a word or has camel case, snake case, kebab case, has spaces, has punctuation
    // split by whitespace and punctuation handles:
    // - snake case
    // - kebab case
    // - punctuation
    // - spaces
    special_word(name)
        .unwrap_or(name)
        .split(|c: char| c.is_ascii_whitespace() || c.is_ascii_punctuation())
        .flat_map(|part| {
            // check if camel case
            let uppercase = part.chars().filter(char::is_ascii_uppercase).count();
            let is_camel_case = (part
                .chars()
                .next()
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
                    s == "framework"
                        || s == "nodejs-framework"
                        || s == "fullstack"
                        || s == "css-lib"
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
    let ret: Vec<(&u64, String, String)> = sorted
        .iter()
        .map(|(stars, name)| {
            (
                stars,
                name.split(|c: char| c.is_ascii_whitespace())
                    // remove version number from name
                    .filter(|s| !s.chars().all(char::is_numeric))
                    // remove .js/JS/.mjs
                    .map(|s| s.trim_end_matches(".js"))
                    .map(|s| s.trim_end_matches(".mjs"))
                    .map(|s| s.trim_end_matches("JS"))
                    .map(|s| s.trim_end_matches("js"))
                    // remove .css/CSS
                    .map(|s| s.trim_end_matches(".css"))
                    .map(|s| s.trim_end_matches("CSS"))
                    // remove extra whitespace
                    .map(|s| s.trim())
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        })
        .map(|(stars, name)| (stars, name.clone(), get_framework_pronounce(&name, &trie)))
        .collect();
    // write to csv
    let mut wtr = Writer::from_path(out_path)?;
    for (stars, name, pronounce) in ret {
        wtr.write_record(&[name, pronounce, stars.to_string()])?;
    }
    Ok(())
}
