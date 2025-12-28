//! Consumes a block comment from JSONC input.

use crate::types::{ConsumeResult, Token, TokenizeError};

/// Consumes a block comment (/* ... */).
pub fn consume_block_comment(
    bytes: &[u8],
    start_pos: usize,
    len: usize,
    start_line: u32,
    start_column: u32,
    errors: &mut Vec<TokenizeError>,
) -> ConsumeResult {
    let mut pos = start_pos + 2; // skip '/*'
    let mut line = start_line;
    let mut column = start_column + 2;
    let mut value = vec![b'/', b'*'];

    // Consume until '*/' or end of input.
    while pos < len {
        let ch = bytes[pos];

        // Check for closing '*/'.
        if ch == b'*' && pos + 1 < len && bytes[pos + 1] == b'/' {
            value.push(b'*');
            value.push(b'/');
            pos += 2;
            column += 2;
            return ConsumeResult {
                column,
                line,
                pos,
                token: Token {
                    category: "comment".to_string(),
                    column: start_column,
                    line: start_line,
                    value: String::from_utf8_lossy(&value).to_string(),
                },
            };
        }

        // Handle newlines inside block comment.
        if ch == b'\n' {
            value.push(ch);
            pos += 1;
            line += 1;
            column = 0;
            continue;
        }
        if ch == b'\r' {
            value.push(ch);
            pos += 1;
            // Handle \r\n.
            if pos < len && bytes[pos] == b'\n' {
                value.push(b'\n');
                pos += 1;
            }
            line += 1;
            column = 0;
            continue;
        }

        // Regular character.
        value.push(ch);
        pos += 1;
        column += 1;
    }

    // Reached end without closing '*/'.
    errors.push(TokenizeError {
        category: "syntax".to_string(),
        column: start_column,
        line: start_line,
        message: "Unterminated block comment".to_string(),
    });

    ConsumeResult {
        column,
        line,
        pos,
        token: Token {
            category: "comment".to_string(),
            column: start_column,
            line: start_line,
            value: String::from_utf8_lossy(&value).to_string(),
        },
    }
}
