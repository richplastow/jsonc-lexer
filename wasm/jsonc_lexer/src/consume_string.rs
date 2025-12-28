//! Consumes a double-quoted string from JSONC input.

use crate::is_hex_digit::is_hex_digit;
use crate::types::{ConsumeResult, Token, TokenizeError};

/// Consumes a double-quoted string.
pub fn consume_string(
    bytes: &[u8],
    start_pos: usize,
    len: usize,
    line: u32,
    start_column: u32,
    errors: &mut Vec<TokenizeError>,
) -> ConsumeResult {
    let mut pos = start_pos + 1; // skip opening quote
    let mut column = start_column + 1;
    let mut value = vec![b'"'];

    while pos < len {
        let ch = bytes[pos];

        // Closing quote found.
        if ch == b'"' {
            value.push(b'"');
            pos += 1;
            column += 1;
            return ConsumeResult {
                column,
                line,
                pos,
                token: Token {
                    category: "string".to_string(),
                    column: start_column,
                    line,
                    value: String::from_utf8_lossy(&value).to_string(),
                },
            };
        }

        // Escape sequence.
        if ch == b'\\' {
            if pos + 1 >= len {
                errors.push(TokenizeError {
                    category: "syntax".to_string(),
                    column,
                    line,
                    message: "Unterminated escape sequence".to_string(),
                });
                value.push(ch);
                pos += 1;
                column += 1;
                break;
            }
            let next = bytes[pos + 1];
            // Valid escape characters: " \ / b f n r t u
            if next == b'"' || next == b'\\' || next == b'/'
                || next == b'b' || next == b'f' || next == b'n'
                || next == b'r' || next == b't'
            {
                value.push(ch);
                value.push(next);
                pos += 2;
                column += 2;
                continue;
            }
            // Unicode escape: \uXXXX
            if next == b'u' {
                if pos + 5 >= len {
                    errors.push(TokenizeError {
                        category: "syntax".to_string(),
                        column,
                        line,
                        message: "Incomplete unicode escape sequence".to_string(),
                    });
                    value.push(ch);
                    value.push(next);
                    pos += 2;
                    column += 2;
                    continue;
                }
                // Check for 4 hex digits.
                let mut valid = true;
                for i in 0..4 {
                    if !is_hex_digit(bytes[pos + 2 + i]) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    errors.push(TokenizeError {
                        category: "syntax".to_string(),
                        column,
                        line,
                        message: "Invalid unicode escape sequence".to_string(),
                    });
                }
                // Consume \uXXXX regardless.
                for i in 0..6 {
                    value.push(bytes[pos + i]);
                }
                pos += 6;
                column += 6;
                continue;
            }
            // Invalid escape character.
            errors.push(TokenizeError {
                category: "syntax".to_string(),
                column,
                line,
                message: format!("Invalid escape character '\\{}'", next as char),
            });
            value.push(ch);
            value.push(next);
            pos += 2;
            column += 2;
            continue;
        }

        // Unescaped newline in string is an error.
        if ch == b'\n' || ch == b'\r' {
            errors.push(TokenizeError {
                category: "syntax".to_string(),
                column: start_column,
                line,
                message: "Unterminated string literal".to_string(),
            });
            break;
        }

        // Control characters (U+0000 to U+001F) should be escaped.
        if ch < 32 {
            errors.push(TokenizeError {
                category: "syntax".to_string(),
                column,
                line,
                message: "Unescaped control character in string".to_string(),
            });
        }

        // Regular character.
        value.push(ch);
        pos += 1;
        column += 1;
    }

    // Reached end without closing quote.
    if value.is_empty() || value[value.len() - 1] != b'"' {
        errors.push(TokenizeError {
            category: "syntax".to_string(),
            column: start_column,
            line,
            message: "Unterminated string literal".to_string(),
        });
    }

    ConsumeResult {
        column,
        line,
        pos,
        token: Token {
            category: "string".to_string(),
            column: start_column,
            line,
            value: String::from_utf8_lossy(&value).to_string(),
        },
    }
}
