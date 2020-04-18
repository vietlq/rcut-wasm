mod utils;

use wasm_bindgen::prelude::*;

extern crate rcut;
use rcut::{extract_ranged_pairs, merge_ranged_pairs, process_line_by_char_utf8};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn rcut_char(line: &str, char_ranges: &str) -> String {
    let ranged_pairs = extract_ranged_pairs(char_ranges);
    let bytes = process_line_by_char_utf8(line, &ranged_pairs);
    String::from_utf8(bytes).unwrap()
}
