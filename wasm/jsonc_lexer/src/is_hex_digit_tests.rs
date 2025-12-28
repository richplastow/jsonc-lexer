//! Tests for is_hex_digit.rs

use pretty_assertions::assert_eq;
use crate::is_hex_digit::is_hex_digit;

#[test]
fn hex_digit_zero() {
    assert_eq!(is_hex_digit(b'0'), true);
}

#[test]
fn hex_digit_nine() {
    assert_eq!(is_hex_digit(b'9'), true);
}

#[test]
fn hex_digit_lowercase_a() {
    assert_eq!(is_hex_digit(b'a'), true);
}

#[test]
fn hex_digit_lowercase_f() {
    assert_eq!(is_hex_digit(b'f'), true);
}

#[test]
fn hex_digit_uppercase_a() {
    assert_eq!(is_hex_digit(b'A'), true);
}

#[test]
fn hex_digit_uppercase_f() {
    assert_eq!(is_hex_digit(b'F'), true);
}

#[test]
fn not_hex_lowercase_g() {
    assert_eq!(is_hex_digit(b'g'), false);
}

#[test]
fn not_hex_uppercase_g() {
    assert_eq!(is_hex_digit(b'G'), false);
}

#[test]
fn not_hex_space() {
    assert_eq!(is_hex_digit(b' '), false);
}

#[test]
fn not_hex_x() {
    assert_eq!(is_hex_digit(b'x'), false);
}
