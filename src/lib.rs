mod utils;

extern crate console_error_panic_hook;
use std::panic;

use std::io::prelude::*;
use std::io::BufReader;
use std::slice;
use wasm_bindgen::prelude::*;

extern crate rcut;
use rcut::{prepare_ranged_pairs, process_line_by_byte, process_line_by_char_utf8};

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
pub fn rcut_chars(line: &str, char_ranges: &str, merge_ranges: bool) -> String {
    let ranged_pairs = prepare_ranged_pairs(!merge_ranges, char_ranges);
    let bytes = process_line_by_char_utf8(line, &ranged_pairs);
    String::from_utf8(bytes).unwrap()
}

#[wasm_bindgen]
pub fn rcut_bytes(line: &str, byte_ranges: &str, merge_ranges: bool) -> Vec<u8> {
    let ranged_pairs = prepare_ranged_pairs(!merge_ranges, byte_ranges);
    let bytes = process_line_by_byte(line, &ranged_pairs);
    bytes
}

// https://github.com/emscripten-core/emscripten/issues/8441
fn rcut_chars_from_raw_impl(raw_chars: &Vec<u8>, char_ranges: &str) -> String {
    let input_str = String::from_utf8(raw_chars.clone()).unwrap();
    let cursor = std::io::Cursor::new(&input_str);
    let buf_reader = BufReader::new(cursor);
    let merge_ranges = true;
    let ranged_pairs = prepare_ranged_pairs(!merge_ranges, char_ranges);
    let mut result = Vec::<u8>::new();

    for line in buf_reader.lines() {
        result.extend(process_line_by_char_utf8(&line.unwrap(), &ranged_pairs));
    }

    String::from_utf8(result).unwrap()
}

#[wasm_bindgen]
#[no_mangle]
pub fn rcut_chars_from_raw(raw_chars: *const u8, input_size: usize, char_ranges: &str) -> String {
    console_error_panic_hook::set_once();

    unsafe {
        rcut_chars_from_raw_impl(
            &slice::from_raw_parts(raw_chars, input_size).to_vec(),
            char_ranges,
        )
    }
}
