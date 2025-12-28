//! Consumes a single-line comment from JSONC input.

use crate::types::{ConsumeResult, Token};

/// Consumes a single-line comment (// ...).
pub fn consume_line_comment(
    bytes: &[u8],
    start_pos: usize,
    len: usize,
    line: u32,
    start_column: u32,
) -> ConsumeResult {
    let mut pos = start_pos + 2; // skip '//'
    let mut column = start_column + 2;
    let mut value = vec![b'/', b'/'];

    // Consume until newline or end of input.
    while pos < len {
        let ch = bytes[pos];
        if ch == b'\n' || ch == b'\r' {
            break;
        }
        value.push(ch);
        pos += 1;
        column += 1;
    }

    ConsumeResult {
        column,
        line,
        pos,
        token: Token {
            category: "comment".to_string(),
            column: start_column,
            line,
            value: String::from_utf8_lossy(&value).to_string(),
        },
    }
}
