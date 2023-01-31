use crate::pronounce::Pronounce;
use crate::pronounce::PronounceBuilder;
use itertools::Itertools;

pub fn replace_words(s: &str, replace_fn: impl Fn(&str) -> String) -> String {
    s.chars()
        .group_by(|c| !(c.is_ascii_punctuation() || c.is_whitespace()))
        .into_iter()
        .map(|(is_word, group)| {
            let part = group.collect::<String>();
            match (is_word, part.is_ascii()) {
                (true, true) => replace_fn(&part),
                _ => part,
            }
        })
        .collect::<String>()
}

pub struct Replacer {
    words: Vec<(String, Pronounce)>,
    builder: PronounceBuilder,
}
impl Replacer {
    pub fn new(words: Vec<(String, String)>) -> Self {
        let builder = PronounceBuilder::new();
        let words = words
            .into_iter()
            .map(|(word, arpabet)| {
                let pronounce = builder.pronounce_from_arpabet(&arpabet);
                (word, pronounce)
            })
            .collect();
        Replacer { words, builder }
    }

    pub fn replace(&self, s: &str, threshold: f32) -> String {
        replace_words(s, |word| {
            let pronounce = self.builder.pronounce(word);
            self.words
                .iter()
                // map to (difference, word)
                .map(|other| (other.1.distance(&pronounce), other.0.clone()))
                // find the smallest difference
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                // filter out words that are too different
                .filter(|a| a.0 < threshold)
                .map(|a| a.1)
                .unwrap_or_else(|| word.to_owned())
        })
    }
}
