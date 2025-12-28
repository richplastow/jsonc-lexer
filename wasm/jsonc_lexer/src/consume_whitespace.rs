//! Consumes whitespace characters from JSONC input.

use crate::types::{ConsumeResult, Token};

/// Consumes contiguous whitespace characters.
pub fn consume_whitespace(
    bytes: &[u8],
    start_pos: usize,
    len: usize,
    start_line: u32,
    start_column: u32,
) -> ConsumeResult {
    let mut pos = start_pos;
    let mut line = start_line;
    let mut column = start_column;
    let mut value = Vec::new();

    // Consume until non-whitespace or newline boundary.
    while pos < len {
        let ch = bytes[pos];

        // Newline ends this whitespace token (newline is its own token).
        if ch == b'\n' {
            if value.is_empty() {
                // Newline is the first char, include it and stop.
                value.push(b'\n');
                pos += 1;
                line += 1;
                column = 0;
            }
            // If we already have content, stop before the newline.
            break;
        }

        // Carriage return: handle \r\n as single newline.
        if ch == b'\r' {
            if value.is_empty() {
                // Check for \r\n sequence.
                if pos + 1 < len && bytes[pos + 1] == b'\n' {
                    value.push(b'\r');
                    value.push(b'\n');
                    pos += 2;
                } else {
                    value.push(b'\r');
                    pos += 1;
                }
                line += 1;
                column = 0;
            }
            break;
        }

        // Space or tab: accumulate.
        if ch == b' ' || ch == b'\t' {
            value.push(ch);
            pos += 1;
            column += 1;
            continue;
        }

        // Non-whitespace: stop.
        break;
    }

    let value_str = String::from_utf8(value).expect("whitespace should be valid UTF-8");
    ConsumeResult {
        column,
        line,
        pos,
        token: Token {
            category: "whitespace".to_string(),
            column: start_column,
            line: start_line,
            value: value_str,
        },
    }
}
