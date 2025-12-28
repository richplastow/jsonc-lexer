//! Tests for consume_block_comment.rs

use pretty_assertions::assert_eq;
use crate::consume_block_comment::consume_block_comment;

// Empty comment.
#[test]
fn empty_comment_value() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/**/", 0, 4, 1, 0, &mut errors);
    assert_eq!(result.token.value, "/**/");
}

#[test]
fn empty_comment_pos() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/**/", 0, 4, 1, 0, &mut errors);
    assert_eq!(result.pos, 4);
}

#[test]
fn empty_comment_category() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/**/", 0, 4, 1, 0, &mut errors);
    assert_eq!(result.token.category, "comment");
}

// Simple comment.
#[test]
fn simple_comment_value() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/* hello */", 0, 11, 1, 0, &mut errors);
    assert_eq!(result.token.value, "/* hello */");
}

#[test]
fn simple_comment_pos() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/* hello */", 0, 11, 1, 0, &mut errors);
    assert_eq!(result.pos, 11);
}

// Comment with special characters.
#[test]
fn special_chars() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/* @TODO: fix! */", 0, 17, 1, 0, &mut errors);
    assert_eq!(result.token.value, "/* @TODO: fix! */");
}

// Comment with asterisks inside.
#[test]
fn asterisks_inside() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/* * */", 0, 7, 1, 0, &mut errors);
    assert_eq!(result.token.value, "/* * */");
}

// Multi-line comment.
#[test]
fn multiline_value() {
    let mut errors = Vec::new();
    let input = b"/* line1\nline2 */";
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.token.value, "/* line1\nline2 */");
}

#[test]
fn multiline_line() {
    let mut errors = Vec::new();
    let input = b"/* line1\nline2 */";
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.line, 2);
}

#[test]
fn multiline_column() {
    let mut errors = Vec::new();
    let input = b"/* line1\nline2 */";
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.column, 8);
}

// Comment with CRLF.
#[test]
fn crlf_value() {
    let mut errors = Vec::new();
    let input = b"/* line1\r\nline2 */";
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.token.value, "/* line1\r\nline2 */");
}

#[test]
fn crlf_line() {
    let mut errors = Vec::new();
    let input = b"/* line1\r\nline2 */";
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.line, 2);
}

// Comment with CR.
#[test]
fn cr_value() {
    let mut errors = Vec::new();
    let input = b"/* line1\rline2 */";
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.token.value, "/* line1\rline2 */");
}

#[test]
fn cr_line() {
    let mut errors = Vec::new();
    let input = b"/* line1\rline2 */";
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.line, 2);
}

// Comment starting mid-input.
#[test]
fn mid_input_value() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"x/* test */", 1, 11, 1, 5, &mut errors);
    assert_eq!(result.token.value, "/* test */");
}

#[test]
fn mid_input_pos() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"x/* test */", 1, 11, 1, 5, &mut errors);
    assert_eq!(result.pos, 11);
}

#[test]
fn mid_input_column() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"x/* test */", 1, 11, 1, 5, &mut errors);
    assert_eq!(result.column, 15);
}

// Token metadata.
#[test]
fn token_line() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/* x */", 0, 7, 5, 10, &mut errors);
    assert_eq!(result.token.line, 5);
}

#[test]
fn token_column() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/* x */", 0, 7, 5, 10, &mut errors);
    assert_eq!(result.token.column, 10);
}

// Comment with slashes.
#[test]
fn slashes_inside() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/* // */", 0, 9, 1, 0, &mut errors);
    assert_eq!(result.token.value, "/* // */");
}

// Comment with unicode.
#[test]
fn unicode_comment() {
    let mut errors = Vec::new();
    let input = "/* emoji 🎉 */".as_bytes();
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.token.value, "/* emoji 🎉 */");
}

// Unterminated comment - error case.
#[test]
fn unterminated_has_error() {
    let mut errors = Vec::new();
    consume_block_comment(b"/* incomplete", 0, 13, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

#[test]
fn unterminated_message() {
    let mut errors = Vec::new();
    consume_block_comment(b"/* incomplete", 0, 13, 1, 0, &mut errors);
    assert!(errors[0].message.contains("Unterminated"));
}

// Unterminated with newline.
#[test]
fn unterminated_with_newline() {
    let mut errors = Vec::new();
    let input = b"/* test\n";
    consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

// Comment ending with just *.
#[test]
fn unterminated_with_asterisk() {
    let mut errors = Vec::new();
    consume_block_comment(b"/* test *", 0, 9, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

// Very long comment.
#[test]
fn long_comment_length() {
    let mut errors = Vec::new();
    let long_text = format!("/* {} */", "a".repeat(1000));
    let bytes = long_text.as_bytes();
    let result = consume_block_comment(bytes, 0, bytes.len(), 1, 0, &mut errors);
    assert_eq!(result.token.value.len(), 1006);
}

#[test]
fn long_comment_pos() {
    let mut errors = Vec::new();
    let long_text = format!("/* {} */", "a".repeat(1000));
    let bytes = long_text.as_bytes();
    let result = consume_block_comment(bytes, 0, bytes.len(), 1, 0, &mut errors);
    assert_eq!(result.pos, 1006);
}

// Multi-line with multiple newlines.
#[test]
fn multiple_newlines_line() {
    let mut errors = Vec::new();
    let input = b"/* line1\nline2\nline3 */";
    let result = consume_block_comment(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.line, 3);
}

// Column tracking.
#[test]
fn column_tracking() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"/* test */", 0, 10, 1, 0, &mut errors);
    assert_eq!(result.column, 10);
}

// Starting column with offset.
#[test]
fn starting_column_offset() {
    let mut errors = Vec::new();
    let result = consume_block_comment(b"  /* test */", 2, 12, 1, 2, &mut errors);
    assert_eq!(result.column, 12);
}
