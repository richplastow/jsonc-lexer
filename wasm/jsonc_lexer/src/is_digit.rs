//! Helper function to check if a byte is a digit.

/// Checks if a byte is a digit (0-9).
pub fn is_digit(ch: u8) -> bool {
    // '0' = 48, '9' = 57
    ch >= 48 && ch <= 57
}
