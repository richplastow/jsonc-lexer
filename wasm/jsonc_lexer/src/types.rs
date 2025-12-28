//! Type definitions for JSONC tokenization.

use serde::{Deserialize, Serialize};

/// A single token from the JSONC input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    /// The type of token: brace, bracket, colon, comma, comment, keyword, number, string, whitespace
    pub category: String,
    /// Zero-based column position
    pub column: u32,
    /// One-based line number
    pub line: u32,
    /// The raw token value
    pub value: String,
}

/// A syntax error encountered during tokenization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenizeError {
    /// Always "syntax" for tokenization errors
    pub category: String,
    /// Zero-based column where the error occurred
    pub column: u32,
    /// One-based line number where the error occurred
    pub line: u32,
    /// Human-readable error description
    pub message: String,
}

/// The result of tokenizing a JSONC string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenizeResult {
    /// Any syntax errors encountered
    pub errors: Vec<TokenizeError>,
    /// The parsed tokens
    pub tokens: Vec<Token>,
}

/// Internal result from consume functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsumeResult {
    /// Updated column position
    pub column: u32,
    /// Updated line number
    pub line: u32,
    /// Updated position in the string
    pub pos: usize,
    /// The consumed token
    pub token: Token,
}
