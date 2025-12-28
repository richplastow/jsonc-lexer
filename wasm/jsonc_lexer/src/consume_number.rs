//! Consumes a JSON number from JSONC input.

use crate::is_digit::is_digit;
use crate::types::{ConsumeResult, Token, TokenizeError};

/// Consumes a JSON number.
pub fn consume_number(
    bytes: &[u8],
    start_pos: usize,
    len: usize,
    line: u32,
    start_column: u32,
    errors: &mut Vec<TokenizeError>,
) -> ConsumeResult {
    let mut pos = start_pos;
    let mut column = start_column;
    let mut value = Vec::new();

    // Optional leading minus.
    if pos < len && bytes[pos] == b'-' {
        value.push(b'-');
        pos += 1;
        column += 1;
    }

    // Integer part.
    if pos < len && bytes[pos] == b'0' {
        // Leading zero must not be followed by another digit.
        value.push(b'0');
        pos += 1;
        column += 1;
    } else if pos < len && is_digit(bytes[pos]) {
        // Non-zero digit followed by optional digits.
        while pos < len && is_digit(bytes[pos]) {
            value.push(bytes[pos]);
            pos += 1;
            column += 1;
        }
    } else {
        // No digit after minus.
        errors.push(TokenizeError {
            category: "syntax".to_string(),
            column: start_column,
            line,
            message: "Invalid number: expected digit".to_string(),
        });
        return ConsumeResult {
            column,
            line,
            pos,
            token: Token {
                category: "number".to_string(),
                column: start_column,
                line,
                value: String::from_utf8(value).unwrap_or_default(),
            },
        };
    }

    // Optional fractional part.
    if pos < len && bytes[pos] == b'.' {
        value.push(b'.');
        pos += 1;
        column += 1;
        // Must have at least one digit after decimal point.
        if pos >= len || !is_digit(bytes[pos]) {
            errors.push(TokenizeError {
                category: "syntax".to_string(),
                column,
                line,
                message: "Invalid number: expected digit after decimal point".to_string(),
            });
        } else {
            while pos < len && is_digit(bytes[pos]) {
                value.push(bytes[pos]);
                pos += 1;
                column += 1;
            }
        }
    }

    // Optional exponent part.
    if pos < len && (bytes[pos] == b'e' || bytes[pos] == b'E') {
        value.push(bytes[pos]);
        pos += 1;
        column += 1;
        // Optional sign.
        if pos < len && (bytes[pos] == b'+' || bytes[pos] == b'-') {
            value.push(bytes[pos]);
            pos += 1;
            column += 1;
        }
        // Must have at least one digit.
        if pos >= len || !is_digit(bytes[pos]) {
            errors.push(TokenizeError {
                category: "syntax".to_string(),
                column,
                line,
                message: "Invalid number: expected digit in exponent".to_string(),
            });
        } else {
            while pos < len && is_digit(bytes[pos]) {
                value.push(bytes[pos]);
                pos += 1;
                column += 1;
            }
        }
    }

    ConsumeResult {
        column,
        line,
        pos,
        token: Token {
            category: "number".to_string(),
            column: start_column,
            line,
            value: String::from_utf8(value).expect("number should be valid UTF-8"),
        },
    }
}
