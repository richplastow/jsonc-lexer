//! Tests for consume_string.rs

use pretty_assertions::assert_eq;
use crate::consume_string::consume_string;

// Empty string.
#[test]
fn empty_string_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"\"", 0, 2, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"\"");
}

#[test]
fn empty_string_pos() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"\"", 0, 2, 1, 0, &mut errors);
    assert_eq!(result.pos, 2);
}

#[test]
fn empty_string_no_errors() {
    let mut errors = Vec::new();
    consume_string(b"\"\"", 0, 2, 1, 0, &mut errors);
    assert_eq!(errors.len(), 0);
}

// Simple string.
#[test]
fn simple_string_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"hello\"", 0, 7, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"hello\"");
}

#[test]
fn simple_string_pos() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"hello\"", 0, 7, 1, 0, &mut errors);
    assert_eq!(result.pos, 7);
}

#[test]
fn simple_string_no_errors() {
    let mut errors = Vec::new();
    consume_string(b"\"hello\"", 0, 7, 1, 0, &mut errors);
    assert_eq!(errors.len(), 0);
}

// String with spaces.
#[test]
fn string_with_spaces_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"hello world\"", 0, 13, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"hello world\"");
}

#[test]
fn string_with_spaces_pos() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"hello world\"", 0, 13, 1, 0, &mut errors);
    assert_eq!(result.pos, 13);
}

// Escaped quote.
#[test]
fn escaped_quote_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"say \\\"hi\\\"\"", 0, 12, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"say \\\"hi\\\"\"");
}

#[test]
fn escaped_quote_no_errors() {
    let mut errors = Vec::new();
    consume_string(b"\"say \\\"hi\\\"\"", 0, 12, 1, 0, &mut errors);
    assert_eq!(errors.len(), 0);
}

// Escaped backslash.
#[test]
fn escaped_backslash_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"back\\\\slash\"", 0, 13, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"back\\\\slash\"");
}

#[test]
fn escaped_backslash_no_errors() {
    let mut errors = Vec::new();
    consume_string(b"\"back\\\\slash\"", 0, 13, 1, 0, &mut errors);
    assert_eq!(errors.len(), 0);
}

// All valid escapes.
#[test]
fn all_valid_escapes() {
    let mut errors = Vec::new();
    let input = b"\"\\\"\\\\\\/\\b\\f\\n\\r\\t\"";
    let result = consume_string(input, 0, input.len(), 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"\\\"\\\\\\/\\b\\f\\n\\r\\t\"");
    assert_eq!(errors.len(), 0);
}

// Unicode escape.
#[test]
fn unicode_escape_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"\\u0041\"", 0, 8, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"\\u0041\"");
}

#[test]
fn unicode_escape_no_errors() {
    let mut errors = Vec::new();
    consume_string(b"\"\\u0041\"", 0, 8, 1, 0, &mut errors);
    assert_eq!(errors.len(), 0);
}

// Multiple unicode escapes.
#[test]
fn multiple_unicode_escapes() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"\\u0041\\u0042\\u0043\"", 0, 20, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"\\u0041\\u0042\\u0043\"");
    assert_eq!(errors.len(), 0);
}

// Invalid escape character.
#[test]
fn invalid_escape_has_error() {
    let mut errors = Vec::new();
    consume_string(b"\"bad\\x\"", 0, 7, 1, 0, &mut errors);
    assert_eq!(errors.len(), 1);
}

#[test]
fn invalid_escape_message() {
    let mut errors = Vec::new();
    consume_string(b"\"bad\\x\"", 0, 7, 1, 0, &mut errors);
    assert!(errors[0].message.contains("Invalid escape"));
}

#[test]
fn invalid_escape_value_preserved() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"bad\\x\"", 0, 7, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"bad\\x\"");
}

// Incomplete unicode (too short).
#[test]
fn incomplete_unicode_error() {
    let mut errors = Vec::new();
    consume_string(b"\"\\u00\"", 0, 6, 1, 0, &mut errors);
    assert_eq!(errors.len(), 1);
}

#[test]
fn incomplete_unicode_message() {
    let mut errors = Vec::new();
    consume_string(b"\"\\u00\"", 0, 6, 1, 0, &mut errors);
    assert!(errors[0].message.contains("Incomplete unicode"));
}

// Invalid unicode (non-hex chars).
#[test]
fn invalid_unicode_error() {
    let mut errors = Vec::new();
    consume_string(b"\"\\u00XY\"", 0, 8, 1, 0, &mut errors);
    assert_eq!(errors.len(), 1);
}

#[test]
fn invalid_unicode_message() {
    let mut errors = Vec::new();
    consume_string(b"\"\\u00XY\"", 0, 8, 1, 0, &mut errors);
    assert!(errors[0].message.contains("Invalid unicode"));
}

// Unterminated string.
#[test]
fn unterminated_string_has_error() {
    let mut errors = Vec::new();
    consume_string(b"\"hello", 0, 6, 1, 0, &mut errors);
    assert_eq!(errors.len(), 1);
}

#[test]
fn unterminated_string_message() {
    let mut errors = Vec::new();
    consume_string(b"\"hello", 0, 6, 1, 0, &mut errors);
    assert!(errors[0].message.contains("Unterminated"));
}

#[test]
fn unterminated_string_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"hello", 0, 6, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"hello");
}

// Unterminated escape at end.
#[test]
fn unterminated_escape_at_end() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"test\\", 0, 6, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
    assert!(result.token.value.contains("\\"));
}

// String with newline (error).
#[test]
fn string_with_newline_error() {
    let mut errors = Vec::new();
    consume_string(b"\"hello\nworld\"", 0, 13, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

#[test]
fn string_with_newline_message() {
    let mut errors = Vec::new();
    consume_string(b"\"hello\nworld\"", 0, 13, 1, 0, &mut errors);
    assert!(errors[0].message.contains("Unterminated"));
}

#[test]
fn string_with_newline_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"hello\nworld\"", 0, 13, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"hello");
}

// String with CR (error).
#[test]
fn string_with_cr_error() {
    let mut errors = Vec::new();
    consume_string(b"\"hello\rworld\"", 0, 13, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

// Control character in string.
#[test]
fn control_char_error() {
    let mut errors = Vec::new();
    consume_string(b"\"\x01\"", 0, 3, 1, 0, &mut errors);
    assert_eq!(errors.len(), 1);
}

#[test]
fn control_char_message() {
    let mut errors = Vec::new();
    consume_string(b"\"\x01\"", 0, 3, 1, 0, &mut errors);
    assert!(errors[0].message.contains("control character"));
}

// String starting mid-input.
#[test]
fn mid_input_value() {
    let mut errors = Vec::new();
    let result = consume_string(b"x\"test\"", 1, 7, 1, 5, &mut errors);
    assert_eq!(result.token.value, "\"test\"");
}

#[test]
fn mid_input_pos() {
    let mut errors = Vec::new();
    let result = consume_string(b"x\"test\"", 1, 7, 1, 5, &mut errors);
    assert_eq!(result.pos, 7);
}

#[test]
fn mid_input_column() {
    let mut errors = Vec::new();
    let result = consume_string(b"x\"test\"", 1, 7, 1, 5, &mut errors);
    assert_eq!(result.column, 11);
}

// Token metadata.
#[test]
fn token_category() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"x\"", 0, 3, 5, 10, &mut errors);
    assert_eq!(result.token.category, "string");
}

#[test]
fn token_line() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"x\"", 0, 3, 5, 10, &mut errors);
    assert_eq!(result.token.line, 5);
}

#[test]
fn token_column() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"x\"", 0, 3, 5, 10, &mut errors);
    assert_eq!(result.token.column, 10);
}

// Mixed escapes and normal chars.
#[test]
fn mixed_escapes_and_chars() {
    let mut errors = Vec::new();
    let result = consume_string(b"\"a\\nb\\tc\\rd\"", 0, 12, 1, 0, &mut errors);
    assert_eq!(result.token.value, "\"a\\nb\\tc\\rd\"");
    assert_eq!(errors.len(), 0);
}
