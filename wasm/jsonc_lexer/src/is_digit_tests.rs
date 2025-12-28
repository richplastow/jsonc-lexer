//! Tests for is_digit.rs

use pretty_assertions::assert_eq;
use crate::is_digit::is_digit;

#[test]
fn digit_zero() {
    assert_eq!(is_digit(b'0'), true);
}

#[test]
fn digit_one() {
    assert_eq!(is_digit(b'1'), true);
}

#[test]
fn digit_nine() {
    assert_eq!(is_digit(b'9'), true);
}

#[test]
fn not_digit_letter_a() {
    assert_eq!(is_digit(b'a'), false);
}

#[test]
fn not_digit_letter_z() {
    assert_eq!(is_digit(b'z'), false);
}

#[test]
fn not_digit_space() {
    assert_eq!(is_digit(b' '), false);
}

#[test]
fn not_digit_minus() {
    assert_eq!(is_digit(b'-'), false);
}

#[test]
fn not_digit_dot() {
    assert_eq!(is_digit(b'.'), false);
}
