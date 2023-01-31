use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use radix_trie::Trie;
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;

type CmuTrie = Trie<String, String>;
const URL: &str = "https://raw.githubusercontent.com/Alexir/CMUdict/master/cmudict-0.7b";
pub fn get_cmudict(out_path: &str) -> Result<CmuTrie> {
    let resp = get(URL)?;
    let mut lines: Vec<(String, String)> = resp
        .text()?
        .lines()
        // remove comments
        .filter(|line| !line.starts_with(";;;"))
        // remove punctuation
        .filter(|line| line.starts_with(|c: char| c.is_ascii_alphanumeric()))
        .map(|line| {
            line.chars()
                // we don't care about the stress markers or other pronounce details
                .filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace())
                .collect::<String>()
                .to_ascii_uppercase()
        })
        .map(|line| {
            let mut parts = line.split("  ");
            (
                parts.next().unwrap().to_owned(),
                parts.next().unwrap().to_owned(),
            )
        })
        .collect();
    // remove multiple pronounciations of the same word
    lines.dedup_by(|(word, _), (other_word, _)| word == other_word);

    let mut encoder = GzEncoder::new(File::create(out_path)?, Compression::default());
    encoder.write_all(
        lines
            .iter()
            .map(|(word, pronounce)| format!("{}  {}", word, pronounce))
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
    )?;
    encoder.finish()?;
    // we return a Trie so we can get framework pronounciations
    Ok(lines
        .iter()
        .map(|(word, pronounce)| (word.to_owned(), pronounce.to_owned()))
        .collect())
}
