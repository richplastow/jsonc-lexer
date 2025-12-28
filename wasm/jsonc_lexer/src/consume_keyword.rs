//! Consumes a JSON keyword from JSONC input.

use crate::types::{ConsumeResult, Token};

/// Attempts to consume a keyword (true, false, null).
/// Returns None if no keyword matches.
pub fn consume_keyword(
    bytes: &[u8],
    start_pos: usize,
    len: usize,
    line: u32,
    start_column: u32,
) -> Option<ConsumeResult> {
    // Check for 'true'.
    if start_pos + 4 <= len && &bytes[start_pos..start_pos + 4] == b"true" {
        return Some(ConsumeResult {
            column: start_column + 4,
            line,
            pos: start_pos + 4,
            token: Token {
                category: "keyword".to_string(),
                column: start_column,
                line,
                value: "true".to_string(),
            },
        });
    }

    // Check for 'false'.
    if start_pos + 5 <= len && &bytes[start_pos..start_pos + 5] == b"false" {
        return Some(ConsumeResult {
            column: start_column + 5,
            line,
            pos: start_pos + 5,
            token: Token {
                category: "keyword".to_string(),
                column: start_column,
                line,
                value: "false".to_string(),
            },
        });
    }

    // Check for 'null'.
    if start_pos + 4 <= len && &bytes[start_pos..start_pos + 4] == b"null" {
        return Some(ConsumeResult {
            column: start_column + 4,
            line,
            pos: start_pos + 4,
            token: Token {
                category: "keyword".to_string(),
                column: start_column,
                line,
                value: "null".to_string(),
            },
        });
    }

    // No keyword matched.
    None
}
