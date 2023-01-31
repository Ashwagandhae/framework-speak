mod framework_data;
mod pronounce;
mod replacer;
use framework_data::read_frameworks;
use replacer::Replacer;
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

#[wasm_bindgen]
impl FrameworkReplacer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let rep = Replacer::new(read_frameworks());
        FrameworkReplacer { rep }
    }

    pub fn replace(&self, s: &str, threshold: f32) -> String {
        self.rep.replace(s, threshold)
    }
}
