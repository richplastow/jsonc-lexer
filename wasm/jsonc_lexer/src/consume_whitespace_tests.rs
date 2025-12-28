//! Tests for consume_whitespace.rs

use pretty_assertions::assert_eq;
use crate::consume_whitespace::consume_whitespace;
use crate::types::Token;

// Single space.
#[test]
fn single_space_value() {
    let result = consume_whitespace(b" ", 0, 1, 1, 0);
    assert_eq!(result.token.value, " ");
}

#[test]
fn single_space_pos() {
    let result = consume_whitespace(b" ", 0, 1, 1, 0);
    assert_eq!(result.pos, 1);
}

#[test]
fn single_space_column() {
    let result = consume_whitespace(b" ", 0, 1, 1, 0);
    assert_eq!(result.column, 1);
}

#[test]
fn single_space_line() {
    let result = consume_whitespace(b" ", 0, 1, 1, 0);
    assert_eq!(result.line, 1);
}

// Multiple spaces.
#[test]
fn multiple_spaces_value() {
    let result = consume_whitespace(b"    ", 0, 4, 1, 0);
    assert_eq!(result.token.value, "    ");
}

#[test]
fn multiple_spaces_pos() {
    let result = consume_whitespace(b"    ", 0, 4, 1, 0);
    assert_eq!(result.pos, 4);
}

#[test]
fn multiple_spaces_column() {
    let result = consume_whitespace(b"    ", 0, 4, 1, 0);
    assert_eq!(result.column, 4);
}

// Single tab.
#[test]
fn single_tab_value() {
    let result = consume_whitespace(b"\t", 0, 1, 1, 0);
    assert_eq!(result.token.value, "\t");
}

#[test]
fn single_tab_pos() {
    let result = consume_whitespace(b"\t", 0, 1, 1, 0);
    assert_eq!(result.pos, 1);
}

#[test]
fn single_tab_column() {
    let result = consume_whitespace(b"\t", 0, 1, 1, 0);
    assert_eq!(result.column, 1);
}

// Mixed spaces and tabs.
#[test]
fn mixed_spaces_tabs_value() {
    let result = consume_whitespace(b"  \t \t", 0, 5, 1, 0);
    assert_eq!(result.token.value, "  \t \t");
}

#[test]
fn mixed_spaces_tabs_pos() {
    let result = consume_whitespace(b"  \t \t", 0, 5, 1, 0);
    assert_eq!(result.pos, 5);
}

#[test]
fn mixed_spaces_tabs_column() {
    let result = consume_whitespace(b"  \t \t", 0, 5, 1, 0);
    assert_eq!(result.column, 5);
}

// Single newline.
#[test]
fn single_newline_value() {
    let result = consume_whitespace(b"\n", 0, 1, 1, 0);
    assert_eq!(result.token.value, "\n");
}

#[test]
fn single_newline_pos() {
    let result = consume_whitespace(b"\n", 0, 1, 1, 0);
    assert_eq!(result.pos, 1);
}

#[test]
fn single_newline_column() {
    let result = consume_whitespace(b"\n", 0, 1, 1, 0);
    assert_eq!(result.column, 0);
}

#[test]
fn single_newline_line() {
    let result = consume_whitespace(b"\n", 0, 1, 1, 0);
    assert_eq!(result.line, 2);
}

// CRLF sequence.
#[test]
fn crlf_value() {
    let result = consume_whitespace(b"\r\n", 0, 2, 1, 0);
    assert_eq!(result.token.value, "\r\n");
}

#[test]
fn crlf_pos() {
    let result = consume_whitespace(b"\r\n", 0, 2, 1, 0);
    assert_eq!(result.pos, 2);
}

#[test]
fn crlf_column() {
    let result = consume_whitespace(b"\r\n", 0, 2, 1, 0);
    assert_eq!(result.column, 0);
}

#[test]
fn crlf_line() {
    let result = consume_whitespace(b"\r\n", 0, 2, 1, 0);
    assert_eq!(result.line, 2);
}

// Lone CR.
#[test]
fn lone_cr_value() {
    let result = consume_whitespace(b"\r", 0, 1, 1, 0);
    assert_eq!(result.token.value, "\r");
}

#[test]
fn lone_cr_pos() {
    let result = consume_whitespace(b"\r", 0, 1, 1, 0);
    assert_eq!(result.pos, 1);
}

#[test]
fn lone_cr_column() {
    let result = consume_whitespace(b"\r", 0, 1, 1, 0);
    assert_eq!(result.column, 0);
}

#[test]
fn lone_cr_line() {
    let result = consume_whitespace(b"\r", 0, 1, 1, 0);
    assert_eq!(result.line, 2);
}

// Spaces before newline (should stop at newline).
#[test]
fn spaces_before_newline_value() {
    let result = consume_whitespace(b"  \n  ", 0, 5, 1, 0);
    assert_eq!(result.token.value, "  ");
}

#[test]
fn spaces_before_newline_pos() {
    let result = consume_whitespace(b"  \n  ", 0, 5, 1, 0);
    assert_eq!(result.pos, 2);
}

#[test]
fn spaces_before_newline_column() {
    let result = consume_whitespace(b"  \n  ", 0, 5, 1, 0);
    assert_eq!(result.column, 2);
}

#[test]
fn spaces_before_newline_line() {
    let result = consume_whitespace(b"  \n  ", 0, 5, 1, 0);
    assert_eq!(result.line, 1);
}

// Newline followed by spaces (consume just newline).
#[test]
fn newline_then_spaces_value() {
    let result = consume_whitespace(b"\n  ", 0, 3, 1, 0);
    assert_eq!(result.token.value, "\n");
}

#[test]
fn newline_then_spaces_pos() {
    let result = consume_whitespace(b"\n  ", 0, 3, 1, 0);
    assert_eq!(result.pos, 1);
}

#[test]
fn newline_then_spaces_column() {
    let result = consume_whitespace(b"\n  ", 0, 3, 1, 0);
    assert_eq!(result.column, 0);
}

#[test]
fn newline_then_spaces_line() {
    let result = consume_whitespace(b"\n  ", 0, 3, 1, 0);
    assert_eq!(result.line, 2);
}

// Whitespace followed by non-whitespace.
#[test]
fn whitespace_then_nonws_value() {
    let result = consume_whitespace(b"  x", 0, 3, 1, 0);
    assert_eq!(result.token.value, "  ");
}

#[test]
fn whitespace_then_nonws_pos() {
    let result = consume_whitespace(b"  x", 0, 3, 1, 0);
    assert_eq!(result.pos, 2);
}

#[test]
fn whitespace_then_nonws_column() {
    let result = consume_whitespace(b"  x", 0, 3, 1, 0);
    assert_eq!(result.column, 2);
}

// Starting from middle position.
#[test]
fn mid_input_value() {
    let result = consume_whitespace(b"x   y", 1, 5, 1, 10);
    assert_eq!(result.token.value, "   ");
}

#[test]
fn mid_input_pos() {
    let result = consume_whitespace(b"x   y", 1, 5, 1, 10);
    assert_eq!(result.pos, 4);
}

#[test]
fn mid_input_column() {
    let result = consume_whitespace(b"x   y", 1, 5, 1, 10);
    assert_eq!(result.column, 13);
}

// Token metadata.
#[test]
fn token_metadata() {
    let result = consume_whitespace(b"  ", 0, 2, 5, 10);
    assert_eq!(result.token, Token {
        category: "whitespace".to_string(),
        column: 10,
        line: 5,
        value: "  ".to_string(),
    });
}
