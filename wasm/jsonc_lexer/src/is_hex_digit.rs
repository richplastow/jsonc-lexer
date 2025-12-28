//! Helper function to check if a byte is a hexadecimal digit.

/// Checks if a byte is a hex digit (0-9, a-f, A-F).
pub fn is_hex_digit(ch: u8) -> bool {
    (ch >= 48 && ch <= 57)  // 0-9
        || (ch >= 65 && ch <= 70)  // A-F
        || (ch >= 97 && ch <= 102) // a-f
}
