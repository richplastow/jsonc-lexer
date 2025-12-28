//! Tokenizes JSONC (JSON with Comments) into an array of token objects.
//!
//! Uses simple while loops over bytes for easy porting to/from JavaScript.

use wasm_bindgen::prelude::*;

pub mod types;
pub mod is_digit;
pub mod is_hex_digit;
pub mod consume_whitespace;
pub mod consume_string;
pub mod consume_line_comment;
pub mod consume_block_comment;
pub mod consume_number;
pub mod consume_keyword;

use types::{Token, TokenizeError, TokenizeResult};
use is_digit::is_digit;
use consume_whitespace::consume_whitespace;
use consume_string::consume_string;
use consume_line_comment::consume_line_comment;
use consume_block_comment::consume_block_comment;
use consume_number::consume_number;
use consume_keyword::consume_keyword;

#[cfg(test)]
mod is_digit_tests;
#[cfg(test)]
mod is_hex_digit_tests;
#[cfg(test)]
mod consume_whitespace_tests;
#[cfg(test)]
mod consume_string_tests;
#[cfg(test)]
mod consume_line_comment_tests;
#[cfg(test)]
mod consume_block_comment_tests;
#[cfg(test)]
mod consume_number_tests;
#[cfg(test)]
mod consume_keyword_tests;
#[cfg(test)]
mod tokenize_jsonc_tests;

/// Tokenizes a JSONC string into tokens and errors (native Rust version).
pub fn jsonc_lexer_native(input: &str) -> TokenizeResult {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut tokens: Vec<Token> = Vec::new();
    let mut errors: Vec<TokenizeError> = Vec::new();

    let mut pos: usize = 0; // current position in the string
    let mut line: u32 = 1;  // one-based line number
    let mut column: u32 = 0; // zero-based column position

    // Main tokenization loop.
    while pos < len {
        let ch = bytes[pos];

        // Whitespace: space, tab, carriage return, or newline.
        if ch == b' ' || ch == b'\t' || ch == b'\r' || ch == b'\n' {
            let result = consume_whitespace(bytes, pos, len, line, column);
            tokens.push(result.token);
            pos = result.pos;
            line = result.line;
            column = result.column;
            continue;
        }

        // Single-character structural tokens.
        if ch == b'{' || ch == b'}' {
            tokens.push(Token {
                category: "brace".to_string(),
                column,
                line,
                value: (ch as char).to_string(),
            });
            pos += 1;
            column += 1;
            continue;
        }
        if ch == b'[' || ch == b']' {
            tokens.push(Token {
                category: "bracket".to_string(),
                column,
                line,
                value: (ch as char).to_string(),
            });
            pos += 1;
            column += 1;
            continue;
        }
        if ch == b':' {
            tokens.push(Token {
                category: "colon".to_string(),
                column,
                line,
                value: ":".to_string(),
            });
            pos += 1;
            column += 1;
            continue;
        }
        if ch == b',' {
            tokens.push(Token {
                category: "comma".to_string(),
                column,
                line,
                value: ",".to_string(),
            });
            pos += 1;
            column += 1;
            continue;
        }

        // String: starts with double quote.
        if ch == b'"' {
            let result = consume_string(bytes, pos, len, line, column, &mut errors);
            tokens.push(result.token);
            pos = result.pos;
            column = result.column;
            continue;
        }

        // Comment: starts with '/'.
        if ch == b'/' {
            let next = if pos + 1 < len { bytes[pos + 1] } else { 0 };
            if next == b'/' {
                let result = consume_line_comment(bytes, pos, len, line, column);
                tokens.push(result.token);
                pos = result.pos;
                column = result.column;
                continue;
            }
            if next == b'*' {
                let result = consume_block_comment(bytes, pos, len, line, column, &mut errors);
                tokens.push(result.token);
                pos = result.pos;
                line = result.line;
                column = result.column;
                continue;
            }
            // Lone '/' is invalid in JSONC.
            errors.push(TokenizeError {
                category: "syntax".to_string(),
                column,
                line,
                message: "Unexpected character '/'".to_string(),
            });
            pos += 1;
            column += 1;
            continue;
        }

        // Number: starts with digit or minus sign.
        if is_digit(ch) || ch == b'-' {
            let result = consume_number(bytes, pos, len, line, column, &mut errors);
            tokens.push(result.token);
            pos = result.pos;
            column = result.column;
            continue;
        }

        // Keywords: true, false, null.
        if ch == b't' || ch == b'f' || ch == b'n' {
            if let Some(result) = consume_keyword(bytes, pos, len, line, column) {
                tokens.push(result.token);
                pos = result.pos;
                column = result.column;
                continue;
            }
            // Fall through to unexpected character if keyword didn't match.
        }

        // Unexpected character.
        errors.push(TokenizeError {
            category: "syntax".to_string(),
            column,
            line,
            message: format!("Unexpected character '{}'", ch as char),
        });
        pos += 1;
        column += 1;
    }

    TokenizeResult { errors, tokens }
}

/// Tokenizes a JSONC string and returns the result as a JS object.
/// Uses serde-wasm-bindgen for efficient object conversion.
#[wasm_bindgen]
pub fn jsonc_lexer(input: &str) -> JsValue {
    let result = jsonc_lexer_native(input);
    serde_wasm_bindgen::to_value(&result).expect("failed to serialize TokenizeResult")
}
