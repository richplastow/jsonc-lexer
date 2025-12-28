//! Tests for consume_line_comment.rs

use pretty_assertions::assert_eq;
use crate::consume_line_comment::consume_line_comment;

// Empty comment.
#[test]
fn empty_comment_value() {
    let result = consume_line_comment(b"//", 0, 2, 1, 0);
    assert_eq!(result.token.value, "//");
}

#[test]
fn empty_comment_pos() {
    let result = consume_line_comment(b"//", 0, 2, 1, 0);
    assert_eq!(result.pos, 2);
}

#[test]
fn empty_comment_category() {
    let result = consume_line_comment(b"//", 0, 2, 1, 0);
    assert_eq!(result.token.category, "comment");
}

// Simple comment.
#[test]
fn simple_comment_value() {
    let result = consume_line_comment(b"// hello", 0, 8, 1, 0);
    assert_eq!(result.token.value, "// hello");
}

#[test]
fn simple_comment_pos() {
    let result = consume_line_comment(b"// hello", 0, 8, 1, 0);
    assert_eq!(result.pos, 8);
}

// Comment with special characters.
#[test]
fn special_chars_value() {
    let result = consume_line_comment(b"// @TODO: fix this!", 0, 19, 1, 0);
    assert_eq!(result.token.value, "// @TODO: fix this!");
}

#[test]
fn special_chars_pos() {
    let result = consume_line_comment(b"// @TODO: fix this!", 0, 19, 1, 0);
    assert_eq!(result.pos, 19);
}

// Comment stops at newline.
#[test]
fn stops_at_newline_value() {
    let result = consume_line_comment(b"// test\nmore", 0, 12, 1, 0);
    assert_eq!(result.token.value, "// test");
}

#[test]
fn stops_at_newline_pos() {
    let result = consume_line_comment(b"// test\nmore", 0, 12, 1, 0);
    assert_eq!(result.pos, 7);
}

#[test]
fn stops_at_newline_line() {
    let result = consume_line_comment(b"// test\nmore", 0, 12, 1, 0);
    assert_eq!(result.line, 1);
}

// Comment stops at CR.
#[test]
fn stops_at_cr_value() {
    let result = consume_line_comment(b"// test\rmore", 0, 12, 1, 0);
    assert_eq!(result.token.value, "// test");
}

#[test]
fn stops_at_cr_pos() {
    let result = consume_line_comment(b"// test\rmore", 0, 12, 1, 0);
    assert_eq!(result.pos, 7);
}

// Comment with trailing spaces.
#[test]
fn trailing_spaces_value() {
    let result = consume_line_comment(b"// test   ", 0, 10, 1, 0);
    assert_eq!(result.token.value, "// test   ");
}

#[test]
fn trailing_spaces_pos() {
    let result = consume_line_comment(b"// test   ", 0, 10, 1, 0);
    assert_eq!(result.pos, 10);
}

// Comment at end of file.
#[test]
fn eof_value() {
    let result = consume_line_comment(b"// EOF", 0, 6, 1, 0);
    assert_eq!(result.token.value, "// EOF");
}

#[test]
fn eof_pos() {
    let result = consume_line_comment(b"// EOF", 0, 6, 1, 0);
    assert_eq!(result.pos, 6);
}

// Comment starting mid-input.
#[test]
fn mid_input_value() {
    let result = consume_line_comment(b"x// test", 1, 8, 1, 5);
    assert_eq!(result.token.value, "// test");
}

#[test]
fn mid_input_pos() {
    let result = consume_line_comment(b"x// test", 1, 8, 1, 5);
    assert_eq!(result.pos, 8);
}

#[test]
fn mid_input_column() {
    let result = consume_line_comment(b"x// test", 1, 8, 1, 5);
    assert_eq!(result.column, 12);
}

// Token metadata.
#[test]
fn token_line() {
    let result = consume_line_comment(b"// x", 0, 4, 5, 10);
    assert_eq!(result.token.line, 5);
}

#[test]
fn token_column() {
    let result = consume_line_comment(b"// x", 0, 4, 5, 10);
    assert_eq!(result.token.column, 10);
}

// Comment with slashes inside.
#[test]
fn slashes_inside() {
    let result = consume_line_comment(b"// path/to/file", 0, 15, 1, 0);
    assert_eq!(result.token.value, "// path/to/file");
}

// Comment with unicode.
#[test]
fn unicode_comment() {
    let input = "// emoji 🎉".as_bytes();
    let result = consume_line_comment(input, 0, input.len(), 1, 0);
    assert_eq!(result.token.value, "// emoji 🎉");
}

// Very long comment.
#[test]
fn long_comment_length() {
    let long_text = format!("// {}", "a".repeat(1000));
    let bytes = long_text.as_bytes();
    let result = consume_line_comment(bytes, 0, bytes.len(), 1, 0);
    assert_eq!(result.token.value.len(), 1003);
}

#[test]
fn long_comment_pos() {
    let long_text = format!("// {}", "a".repeat(1000));
    let bytes = long_text.as_bytes();
    let result = consume_line_comment(bytes, 0, bytes.len(), 1, 0);
    assert_eq!(result.pos, 1003);
}

// Column tracking.
#[test]
fn column_tracking() {
    let result = consume_line_comment(b"// test", 0, 7, 1, 0);
    assert_eq!(result.column, 7);
}
