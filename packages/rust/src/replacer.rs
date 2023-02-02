use crate::pronounce::Pronounce;
use crate::pronounce::PronounceBuilder;
use itertools::Itertools;

pub enum Replacement {
    Replace(String),
    Keep(String),
}

pub fn replace_words(s: &str, replace_fn: impl Fn(&str) -> Option<String>) -> Vec<Replacement> {
    s.chars()
        .group_by(|c| !(c.is_ascii_punctuation() || c.is_whitespace()))
        .into_iter()
        .map(|(is_word, group)| {
            let part = group.collect::<String>();
            match (is_word, part.is_ascii()) {
                (true, true) => replace_fn(&part)
                    .map(|word| Replacement::Replace(word))
                    .unwrap_or(Replacement::Keep(part)),
                _ => Replacement::Keep(part),
            }
        })
        .collect()
}

pub struct Replacer {
    words: Vec<(String, Pronounce, u64)>,
    builder: PronounceBuilder,
}
impl Replacer {
    pub fn new(words: Vec<(String, String, u64)>) -> Self {
        let builder = PronounceBuilder::new();
        let words = words
            .into_iter()
            .map(|(word, arpabet, stars)| {
                let pronounce = builder.pronounce_from_arpabet(&arpabet);
                (word, pronounce, stars)
            })
            .collect();
        Replacer { words, builder }
    }

    pub fn replace(&self, s: &str, threshold: f32, star_threshold: u32) -> Vec<Replacement> {
        replace_words(s, |word| {
            let pronounce = self.builder.pronounce(word);
            self.words
                .iter()
                // filter out words that don't have enough stars
                .filter(|other| other.2 >= star_threshold as u64)
                // map to (difference, word)
                .map(|other| (other.1.distance(&pronounce), other.0.clone()))
                // find the smallest difference
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                // filter out words that are too different
                .filter(|a| a.0 < threshold)
                .map(|a| a.1)
        })
    }
}
