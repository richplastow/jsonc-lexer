//! Tests for consume_number.rs

use pretty_assertions::assert_eq;
use crate::consume_number::consume_number;

// Simple integers.
#[test]
fn zero_value() {
    let mut errors = Vec::new();
    let result = consume_number(b"0", 0, 1, 1, 0, &mut errors);
    assert_eq!(result.token.value, "0");
}

#[test]
fn zero_pos() {
    let mut errors = Vec::new();
    let result = consume_number(b"0", 0, 1, 1, 0, &mut errors);
    assert_eq!(result.pos, 1);
}

#[test]
fn zero_category() {
    let mut errors = Vec::new();
    let result = consume_number(b"0", 0, 1, 1, 0, &mut errors);
    assert_eq!(result.token.category, "number");
}

#[test]
fn digit_one() {
    let mut errors = Vec::new();
    let result = consume_number(b"1", 0, 1, 1, 0, &mut errors);
    assert_eq!(result.token.value, "1");
}

#[test]
fn multi_digit() {
    let mut errors = Vec::new();
    let result = consume_number(b"123", 0, 3, 1, 0, &mut errors);
    assert_eq!(result.token.value, "123");
}

#[test]
fn long_integer() {
    let mut errors = Vec::new();
    let result = consume_number(b"9876543210", 0, 10, 1, 0, &mut errors);
    assert_eq!(result.token.value, "9876543210");
}

// Negative integers.
#[test]
fn negative_zero() {
    let mut errors = Vec::new();
    let result = consume_number(b"-0", 0, 2, 1, 0, &mut errors);
    assert_eq!(result.token.value, "-0");
}

#[test]
fn negative_one() {
    let mut errors = Vec::new();
    let result = consume_number(b"-1", 0, 2, 1, 0, &mut errors);
    assert_eq!(result.token.value, "-1");
}

#[test]
fn negative_multi_digit() {
    let mut errors = Vec::new();
    let result = consume_number(b"-123", 0, 4, 1, 0, &mut errors);
    assert_eq!(result.token.value, "-123");
}

// Decimals.
#[test]
fn decimal_zero() {
    let mut errors = Vec::new();
    let result = consume_number(b"0.0", 0, 3, 1, 0, &mut errors);
    assert_eq!(result.token.value, "0.0");
}

#[test]
fn decimal_simple() {
    let mut errors = Vec::new();
    let result = consume_number(b"1.5", 0, 3, 1, 0, &mut errors);
    assert_eq!(result.token.value, "1.5");
}

#[test]
fn decimal_multi() {
    let mut errors = Vec::new();
    let result = consume_number(b"123.456", 0, 7, 1, 0, &mut errors);
    assert_eq!(result.token.value, "123.456");
}

#[test]
fn negative_decimal() {
    let mut errors = Vec::new();
    let result = consume_number(b"-123.456", 0, 8, 1, 0, &mut errors);
    assert_eq!(result.token.value, "-123.456");
}

// Scientific notation.
#[test]
fn scientific_lowercase() {
    let mut errors = Vec::new();
    let result = consume_number(b"1e10", 0, 4, 1, 0, &mut errors);
    assert_eq!(result.token.value, "1e10");
}

#[test]
fn scientific_uppercase() {
    let mut errors = Vec::new();
    let result = consume_number(b"1E10", 0, 4, 1, 0, &mut errors);
    assert_eq!(result.token.value, "1E10");
}

#[test]
fn scientific_positive() {
    let mut errors = Vec::new();
    let result = consume_number(b"1e+10", 0, 5, 1, 0, &mut errors);
    assert_eq!(result.token.value, "1e+10");
}

#[test]
fn scientific_negative() {
    let mut errors = Vec::new();
    let result = consume_number(b"1e-10", 0, 5, 1, 0, &mut errors);
    assert_eq!(result.token.value, "1e-10");
}

#[test]
fn scientific_with_decimal() {
    let mut errors = Vec::new();
    let result = consume_number(b"1.23e45", 0, 7, 1, 0, &mut errors);
    assert_eq!(result.token.value, "1.23e45");
}

#[test]
fn full_scientific() {
    let mut errors = Vec::new();
    let result = consume_number(b"-1.23e-45", 0, 9, 1, 0, &mut errors);
    assert_eq!(result.token.value, "-1.23e-45");
}

// Number stops at non-digit.
#[test]
fn stops_at_letter_value() {
    let mut errors = Vec::new();
    let result = consume_number(b"123abc", 0, 6, 1, 0, &mut errors);
    assert_eq!(result.token.value, "123");
}

#[test]
fn stops_at_letter_pos() {
    let mut errors = Vec::new();
    let result = consume_number(b"123abc", 0, 6, 1, 0, &mut errors);
    assert_eq!(result.pos, 3);
}

// Number mid-input.
#[test]
fn mid_input_value() {
    let mut errors = Vec::new();
    let result = consume_number(b"x123", 1, 4, 1, 5, &mut errors);
    assert_eq!(result.token.value, "123");
}

#[test]
fn mid_input_pos() {
    let mut errors = Vec::new();
    let result = consume_number(b"x123", 1, 4, 1, 5, &mut errors);
    assert_eq!(result.pos, 4);
}

#[test]
fn mid_input_column() {
    let mut errors = Vec::new();
    let result = consume_number(b"x123", 1, 4, 1, 5, &mut errors);
    assert_eq!(result.column, 8);
}

// Token metadata.
#[test]
fn token_line() {
    let mut errors = Vec::new();
    let result = consume_number(b"123", 0, 3, 5, 10, &mut errors);
    assert_eq!(result.token.line, 5);
}

#[test]
fn token_column() {
    let mut errors = Vec::new();
    let result = consume_number(b"123", 0, 3, 5, 10, &mut errors);
    assert_eq!(result.token.column, 10);
}

// Leading zero is allowed for 0.
#[test]
fn leading_zero_allowed() {
    let mut errors = Vec::new();
    let result = consume_number(b"0", 0, 1, 1, 0, &mut errors);
    assert_eq!(result.token.value, "0");
}

// Invalid: just minus.
#[test]
fn just_minus_error() {
    let mut errors = Vec::new();
    consume_number(b"-", 0, 1, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

// Leading zero - just consumes the 0.
#[test]
fn leading_zero_value() {
    let mut errors = Vec::new();
    let result = consume_number(b"01", 0, 2, 1, 0, &mut errors);
    assert_eq!(result.token.value, "0");
}

#[test]
fn leading_zero_pos() {
    let mut errors = Vec::new();
    let result = consume_number(b"01", 0, 2, 1, 0, &mut errors);
    assert_eq!(result.pos, 1);
}

// Invalid: decimal without fraction.
#[test]
fn decimal_no_fraction_error() {
    let mut errors = Vec::new();
    consume_number(b"1.", 0, 2, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

// Invalid: exponent without digits.
#[test]
fn exponent_no_digits_error() {
    let mut errors = Vec::new();
    consume_number(b"1e", 0, 2, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

// Invalid: exponent sign without digits.
#[test]
fn exponent_sign_no_digits_error() {
    let mut errors = Vec::new();
    consume_number(b"1e+", 0, 3, 1, 0, &mut errors);
    assert!(errors.len() >= 1);
}

// Complex valid number.
#[test]
fn complex_number() {
    let mut errors = Vec::new();
    let result = consume_number(b"-123.456e-789", 0, 13, 1, 0, &mut errors);
    assert_eq!(result.token.value, "-123.456e-789");
}

// Column tracking.
#[test]
fn column_tracking() {
    let mut errors = Vec::new();
    let result = consume_number(b"123", 0, 3, 1, 0, &mut errors);
    assert_eq!(result.column, 3);
}
