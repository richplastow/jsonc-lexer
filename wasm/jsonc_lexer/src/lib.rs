use wasm_bindgen::prelude::*;

pub fn jsonc_lexer_native(_input: &str) -> String {
    String::from("[123]")
}

#[wasm_bindgen]
pub fn jsonc_lexer(input: &str) -> String {
    jsonc_lexer_native(input)
}
