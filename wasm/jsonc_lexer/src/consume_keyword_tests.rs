//! Tests for consume_keyword.rs

use pretty_assertions::assert_eq;
use crate::consume_keyword::consume_keyword;

// true keyword.
#[test]
fn true_value() {
    let result = consume_keyword(b"true", 0, 4, 1, 0).unwrap();
    assert_eq!(result.token.value, "true");
}

#[test]
fn true_pos() {
    let result = consume_keyword(b"true", 0, 4, 1, 0).unwrap();
    assert_eq!(result.pos, 4);
}

#[test]
fn true_category() {
    let result = consume_keyword(b"true", 0, 4, 1, 0).unwrap();
    assert_eq!(result.token.category, "keyword");
}

// false keyword.
#[test]
fn false_value() {
    let result = consume_keyword(b"false", 0, 5, 1, 0).unwrap();
    assert_eq!(result.token.value, "false");
}

#[test]
fn false_pos() {
    let result = consume_keyword(b"false", 0, 5, 1, 0).unwrap();
    assert_eq!(result.pos, 5);
}

#[test]
fn false_category() {
    let result = consume_keyword(b"false", 0, 5, 1, 0).unwrap();
    assert_eq!(result.token.category, "keyword");
}

// null keyword.
#[test]
fn null_value() {
    let result = consume_keyword(b"null", 0, 4, 1, 0).unwrap();
    assert_eq!(result.token.value, "null");
}

#[test]
fn null_pos() {
    let result = consume_keyword(b"null", 0, 4, 1, 0).unwrap();
    assert_eq!(result.pos, 4);
}

#[test]
fn null_category() {
    let result = consume_keyword(b"null", 0, 4, 1, 0).unwrap();
    assert_eq!(result.token.category, "keyword");
}

// Keywords followed by non-identifier chars.
#[test]
fn true_with_comma_value() {
    let result = consume_keyword(b"true,", 0, 5, 1, 0).unwrap();
    assert_eq!(result.token.value, "true");
}

#[test]
fn true_with_comma_pos() {
    let result = consume_keyword(b"true,", 0, 5, 1, 0).unwrap();
    assert_eq!(result.pos, 4);
}

#[test]
fn false_with_brace_value() {
    let result = consume_keyword(b"false}", 0, 6, 1, 0).unwrap();
    assert_eq!(result.token.value, "false");
}

#[test]
fn false_with_brace_pos() {
    let result = consume_keyword(b"false}", 0, 6, 1, 0).unwrap();
    assert_eq!(result.pos, 5);
}

#[test]
fn null_with_bracket_value() {
    let result = consume_keyword(b"null]", 0, 5, 1, 0).unwrap();
    assert_eq!(result.token.value, "null");
}

#[test]
fn null_with_bracket_pos() {
    let result = consume_keyword(b"null]", 0, 5, 1, 0).unwrap();
    assert_eq!(result.pos, 4);
}

// Keywords mid-input.
#[test]
fn mid_input_value() {
    let result = consume_keyword(b"xtrue", 1, 5, 1, 5).unwrap();
    assert_eq!(result.token.value, "true");
}

#[test]
fn mid_input_pos() {
    let result = consume_keyword(b"xtrue", 1, 5, 1, 5).unwrap();
    assert_eq!(result.pos, 5);
}

#[test]
fn mid_input_column() {
    let result = consume_keyword(b"xtrue", 1, 5, 1, 5).unwrap();
    assert_eq!(result.column, 9);
}

// Token metadata.
#[test]
fn token_line() {
    let result = consume_keyword(b"true", 0, 4, 5, 10).unwrap();
    assert_eq!(result.token.line, 5);
}

#[test]
fn token_column() {
    let result = consume_keyword(b"true", 0, 4, 5, 10).unwrap();
    assert_eq!(result.token.column, 10);
}

// Not keywords - too short.
#[test]
fn tru_returns_none() {
    assert!(consume_keyword(b"tru", 0, 3, 1, 0).is_none());
}

#[test]
fn fals_returns_none() {
    assert!(consume_keyword(b"fals", 0, 4, 1, 0).is_none());
}

#[test]
fn nul_returns_none() {
    assert!(consume_keyword(b"nul", 0, 3, 1, 0).is_none());
}

// Keywords with letters after (still matches the keyword part).
#[test]
fn truex_value() {
    let result = consume_keyword(b"truex", 0, 5, 1, 0).unwrap();
    assert_eq!(result.token.value, "true");
}

#[test]
fn truex_pos() {
    let result = consume_keyword(b"truex", 0, 5, 1, 0).unwrap();
    assert_eq!(result.pos, 4);
}

#[test]
fn falsex_value() {
    let result = consume_keyword(b"falsex", 0, 6, 1, 0).unwrap();
    assert_eq!(result.token.value, "false");
}

#[test]
fn falsex_pos() {
    let result = consume_keyword(b"falsex", 0, 6, 1, 0).unwrap();
    assert_eq!(result.pos, 5);
}

#[test]
fn nullx_value() {
    let result = consume_keyword(b"nullx", 0, 5, 1, 0).unwrap();
    assert_eq!(result.token.value, "null");
}

#[test]
fn nullx_pos() {
    let result = consume_keyword(b"nullx", 0, 5, 1, 0).unwrap();
    assert_eq!(result.pos, 4);
}

// Not keywords - completely different.
#[test]
fn invalid_returns_none() {
    assert!(consume_keyword(b"invalid", 0, 7, 1, 0).is_none());
}

#[test]
fn number_returns_none() {
    assert!(consume_keyword(b"123", 0, 3, 1, 0).is_none());
}

#[test]
fn xyz_returns_none() {
    assert!(consume_keyword(b"xyz", 0, 3, 1, 0).is_none());
}

// Keywords at end of input.
#[test]
fn true_at_end_pos() {
    let result = consume_keyword(b"true", 0, 4, 1, 0).unwrap();
    assert_eq!(result.pos, 4);
}

#[test]
fn false_at_end_pos() {
    let result = consume_keyword(b"false", 0, 5, 1, 0).unwrap();
    assert_eq!(result.pos, 5);
}

#[test]
fn null_at_end_pos() {
    let result = consume_keyword(b"null", 0, 4, 1, 0).unwrap();
    assert_eq!(result.pos, 4);
}

// Column tracking.
#[test]
fn true_column() {
    let result = consume_keyword(b"true", 0, 4, 1, 0).unwrap();
    assert_eq!(result.column, 4);
}

#[test]
fn false_column() {
    let result = consume_keyword(b"false", 0, 5, 1, 0).unwrap();
    assert_eq!(result.column, 5);
}

#[test]
fn null_column() {
    let result = consume_keyword(b"null", 0, 4, 1, 0).unwrap();
    assert_eq!(result.column, 4);
}
