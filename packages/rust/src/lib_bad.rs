mod pronounce;
mod framework_data;
use itertools::Itertools;
use pronounce::Pronounce;
use pronounce::PronounceBuilder;
use framework_data::read_framework_data;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
#[wasm_bindgen]
ReplacePronounce(pub String, pub String);

#[wasm_bindgen]
pub struct Replacer {
    words: Vec<ReplacePronounce>,
    builder: PronounceBuilder,
}
#[wasm_bindgen]
impl Replacer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let words = read_framework_data();
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
