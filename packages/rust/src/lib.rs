mod framework_data;
mod pronounce;
mod replacer;
use framework_data::read_frameworks;
use itertools::Itertools;
use js_sys::Array;
use replacer::Replacement;
use replacer::Replacer;
use serde_derive::Serialize;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct FrameworkReplacer {
    rep: Replacer,
}

#[derive(Serialize)]
pub struct JsReplacement {
    pub word: String,
    pub replace: bool,
}

#[wasm_bindgen]
impl FrameworkReplacer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let rep = Replacer::new(read_frameworks());
        FrameworkReplacer { rep }
    }

    pub fn replace(&self, s: &str, threshold: f32, star_threshold: u32) -> Array {
        self.rep
            .replace(s, threshold, star_threshold)
            .iter()
            .map(|replacement| match replacement {
                Replacement::Keep(word) => JsReplacement {
                    word: word.clone(),
                    replace: false,
                },
                Replacement::Replace(word) => JsReplacement {
                    word: word.clone(),
                    replace: true,
                },
            })
            .coalesce(|mut part, other_part| {
                if part.replace == other_part.replace {
                    part.word.push_str(&other_part.word);
                    Ok(part)
                } else {
                    Err((part, other_part))
                }
            })
            .map(|replacement| serde_wasm_bindgen::to_value(&replacement).unwrap())
            .collect()
    }
}

impl Default for FrameworkReplacer {
    fn default() -> Self {
        Self::new()
    }
}
