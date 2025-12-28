//! Tests for jsonc_lexer_native in lib.rs

use pretty_assertions::assert_eq;
use crate::jsonc_lexer_native;
use crate::types::Token;

// Empty Input.
#[test]
fn empty_string_no_tokens() {
    let result = jsonc_lexer_native("");
    assert_eq!(result.tokens.len(), 0);
}

#[test]
fn empty_string_no_errors() {
    let result = jsonc_lexer_native("");
    assert_eq!(result.errors.len(), 0);
}

// Whitespace.
#[test]
fn single_space() {
    let result = jsonc_lexer_native(" ");
    assert_eq!(result.tokens, vec![
        Token { category: "whitespace".to_string(), column: 0, line: 1, value: " ".to_string() }
    ]);
}

#[test]
fn four_spaces() {
    let result = jsonc_lexer_native("    ");
    assert_eq!(result.tokens, vec![
        Token { category: "whitespace".to_string(), column: 0, line: 1, value: "    ".to_string() }
    ]);
}

#[test]
fn single_tab() {
    let result = jsonc_lexer_native("\t");
    assert_eq!(result.tokens, vec![
        Token { category: "whitespace".to_string(), column: 0, line: 1, value: "\t".to_string() }
    ]);
}

#[test]
fn single_newline() {
    let result = jsonc_lexer_native("\n");
    assert_eq!(result.tokens, vec![
        Token { category: "whitespace".to_string(), column: 0, line: 1, value: "\n".to_string() }
    ]);
}

#[test]
fn spaces_newline_spaces() {
    let result = jsonc_lexer_native("  \n  ");
    assert_eq!(result.tokens, vec![
        Token { category: "whitespace".to_string(), column: 0, line: 1, value: "  ".to_string() },
        Token { category: "whitespace".to_string(), column: 2, line: 1, value: "\n".to_string() },
        Token { category: "whitespace".to_string(), column: 0, line: 2, value: "  ".to_string() },
    ]);
}

#[test]
fn crlf_newline() {
    let result = jsonc_lexer_native("\r\n");
    assert_eq!(result.tokens, vec![
        Token { category: "whitespace".to_string(), column: 0, line: 1, value: "\r\n".to_string() }
    ]);
}

// Braces & Brackets.
#[test]
fn empty_object() {
    let result = jsonc_lexer_native("{}");
    assert_eq!(result.tokens, vec![
        Token { category: "brace".to_string(), column: 0, line: 1, value: "{".to_string() },
        Token { category: "brace".to_string(), column: 1, line: 1, value: "}".to_string() },
    ]);
}

#[test]
fn empty_array() {
    let result = jsonc_lexer_native("[]");
    assert_eq!(result.tokens, vec![
        Token { category: "bracket".to_string(), column: 0, line: 1, value: "[".to_string() },
        Token { category: "bracket".to_string(), column: 1, line: 1, value: "]".to_string() },
    ]);
}

#[test]
fn object_with_space() {
    let result = jsonc_lexer_native("{ }");
    assert_eq!(result.tokens, vec![
        Token { category: "brace".to_string(), column: 0, line: 1, value: "{".to_string() },
        Token { category: "whitespace".to_string(), column: 1, line: 1, value: " ".to_string() },
        Token { category: "brace".to_string(), column: 2, line: 1, value: "}".to_string() },
    ]);
}

// Strings.
#[test]
fn empty_string_literal() {
    let result = jsonc_lexer_native("\"\"");
    assert_eq!(result.tokens, vec![
        Token { category: "string".to_string(), column: 0, line: 1, value: "\"\"".to_string() }
    ]);
}

#[test]
fn simple_string() {
    let result = jsonc_lexer_native("\"hello\"");
    assert_eq!(result.tokens, vec![
        Token { category: "string".to_string(), column: 0, line: 1, value: "\"hello\"".to_string() }
    ]);
}

#[test]
fn string_with_space() {
    let result = jsonc_lexer_native("\"hello world\"");
    assert_eq!(result.tokens, vec![
        Token { category: "string".to_string(), column: 0, line: 1, value: "\"hello world\"".to_string() }
    ]);
}

#[test]
fn string_with_escaped_newline() {
    let result = jsonc_lexer_native("\"line\\nbreak\"");
    assert_eq!(result.tokens, vec![
        Token { category: "string".to_string(), column: 0, line: 1, value: "\"line\\nbreak\"".to_string() }
    ]);
}

#[test]
fn string_with_escaped_quotes() {
    let result = jsonc_lexer_native("\"say \\\"hi\\\"\"");
    assert_eq!(result.tokens, vec![
        Token { category: "string".to_string(), column: 0, line: 1, value: "\"say \\\"hi\\\"\"".to_string() }
    ]);
}

#[test]
fn string_with_escaped_backslash() {
    let result = jsonc_lexer_native("\"back\\\\slash\"");
    assert_eq!(result.tokens, vec![
        Token { category: "string".to_string(), column: 0, line: 1, value: "\"back\\\\slash\"".to_string() }
    ]);
}

#[test]
fn string_with_unicode_escape() {
    let result = jsonc_lexer_native("\"\\u0041\"");
    assert_eq!(result.tokens, vec![
        Token { category: "string".to_string(), column: 0, line: 1, value: "\"\\u0041\"".to_string() }
    ]);
}

#[test]
fn unterminated_string_has_error() {
    let result = jsonc_lexer_native("\"unterminated");
    assert!(result.errors.len() > 0);
}

#[test]
fn unterminated_string_message() {
    let result = jsonc_lexer_native("\"unterminated");
    assert!(result.errors[0].message.contains("Unterminated"));
}

#[test]
fn invalid_escape_has_error() {
    let result = jsonc_lexer_native("\"bad\\x\"");
    assert!(result.errors.len() > 0);
}

#[test]
fn invalid_escape_message() {
    let result = jsonc_lexer_native("\"bad\\x\"");
    assert!(result.errors[0].message.contains("Invalid escape"));
}

// Numbers.
#[test]
fn number_zero() {
    let result = jsonc_lexer_native("0");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "0".to_string() }
    ]);
}

#[test]
fn number_integer() {
    let result = jsonc_lexer_native("123");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "123".to_string() }
    ]);
}

#[test]
fn number_negative() {
    let result = jsonc_lexer_native("-42");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "-42".to_string() }
    ]);
}

#[test]
fn number_decimal() {
    let result = jsonc_lexer_native("3.14");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "3.14".to_string() }
    ]);
}

#[test]
fn number_negative_decimal() {
    let result = jsonc_lexer_native("-0.5");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "-0.5".to_string() }
    ]);
}

#[test]
fn number_scientific_lowercase() {
    let result = jsonc_lexer_native("1e10");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "1e10".to_string() }
    ]);
}

#[test]
fn number_scientific_uppercase() {
    let result = jsonc_lexer_native("1E10");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "1E10".to_string() }
    ]);
}

#[test]
fn number_scientific_negative_exp() {
    let result = jsonc_lexer_native("2.5e-3");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "2.5e-3".to_string() }
    ]);
}

#[test]
fn number_scientific_positive_exp() {
    let result = jsonc_lexer_native("1e+5");
    assert_eq!(result.tokens, vec![
        Token { category: "number".to_string(), column: 0, line: 1, value: "1e+5".to_string() }
    ]);
}

#[test]
fn trailing_decimal_error() {
    let result = jsonc_lexer_native("1.");
    assert!(result.errors.len() > 0);
}

#[test]
fn incomplete_exponent_error() {
    let result = jsonc_lexer_native("1e");
    assert!(result.errors.len() > 0);
}

// Keywords.
#[test]
fn keyword_true() {
    let result = jsonc_lexer_native("true");
    assert_eq!(result.tokens, vec![
        Token { category: "keyword".to_string(), column: 0, line: 1, value: "true".to_string() }
    ]);
}

#[test]
fn keyword_false() {
    let result = jsonc_lexer_native("false");
    assert_eq!(result.tokens, vec![
        Token { category: "keyword".to_string(), column: 0, line: 1, value: "false".to_string() }
    ]);
}

#[test]
fn keyword_null() {
    let result = jsonc_lexer_native("null");
    assert_eq!(result.tokens, vec![
        Token { category: "keyword".to_string(), column: 0, line: 1, value: "null".to_string() }
    ]);
}

// Comments.
#[test]
fn line_comment() {
    let result = jsonc_lexer_native("// hello");
    assert_eq!(result.tokens, vec![
        Token { category: "comment".to_string(), column: 0, line: 1, value: "// hello".to_string() }
    ]);
}

#[test]
fn empty_line_comment() {
    let result = jsonc_lexer_native("//");
    assert_eq!(result.tokens, vec![
        Token { category: "comment".to_string(), column: 0, line: 1, value: "//".to_string() }
    ]);
}

#[test]
fn block_comment() {
    let result = jsonc_lexer_native("/* hello */");
    assert_eq!(result.tokens, vec![
        Token { category: "comment".to_string(), column: 0, line: 1, value: "/* hello */".to_string() }
    ]);
}

#[test]
fn empty_block_comment() {
    let result = jsonc_lexer_native("/**/");
    assert_eq!(result.tokens, vec![
        Token { category: "comment".to_string(), column: 0, line: 1, value: "/**/".to_string() }
    ]);
}

#[test]
fn multiline_block_comment() {
    let result = jsonc_lexer_native("/* line1\nline2 */");
    assert_eq!(result.tokens, vec![
        Token { category: "comment".to_string(), column: 0, line: 1, value: "/* line1\nline2 */".to_string() }
    ]);
}

#[test]
fn unterminated_block_comment_has_error() {
    let result = jsonc_lexer_native("/* unclosed");
    assert!(result.errors.len() > 0);
}

#[test]
fn unterminated_block_comment_message() {
    let result = jsonc_lexer_native("/* unclosed");
    assert!(result.errors[0].message.contains("Unterminated block comment"));
}

// Colons & Commas.
#[test]
fn colon() {
    let result = jsonc_lexer_native(":");
    assert_eq!(result.tokens, vec![
        Token { category: "colon".to_string(), column: 0, line: 1, value: ":".to_string() }
    ]);
}

#[test]
fn comma() {
    let result = jsonc_lexer_native(",");
    assert_eq!(result.tokens, vec![
        Token { category: "comma".to_string(), column: 0, line: 1, value: ",".to_string() }
    ]);
}

// Complex Inputs.
#[test]
fn simple_object() {
    let result = jsonc_lexer_native("{\"key\": \"value\"}");
    assert_eq!(result.tokens, vec![
        Token { category: "brace".to_string(), column: 0, line: 1, value: "{".to_string() },
        Token { category: "string".to_string(), column: 1, line: 1, value: "\"key\"".to_string() },
        Token { category: "colon".to_string(), column: 6, line: 1, value: ":".to_string() },
        Token { category: "whitespace".to_string(), column: 7, line: 1, value: " ".to_string() },
        Token { category: "string".to_string(), column: 8, line: 1, value: "\"value\"".to_string() },
        Token { category: "brace".to_string(), column: 15, line: 1, value: "}".to_string() },
    ]);
}

#[test]
fn simple_array() {
    let result = jsonc_lexer_native("[1, 2, 3]");
    assert_eq!(result.tokens, vec![
        Token { category: "bracket".to_string(), column: 0, line: 1, value: "[".to_string() },
        Token { category: "number".to_string(), column: 1, line: 1, value: "1".to_string() },
        Token { category: "comma".to_string(), column: 2, line: 1, value: ",".to_string() },
        Token { category: "whitespace".to_string(), column: 3, line: 1, value: " ".to_string() },
        Token { category: "number".to_string(), column: 4, line: 1, value: "2".to_string() },
        Token { category: "comma".to_string(), column: 5, line: 1, value: ",".to_string() },
        Token { category: "whitespace".to_string(), column: 6, line: 1, value: " ".to_string() },
        Token { category: "number".to_string(), column: 7, line: 1, value: "3".to_string() },
        Token { category: "bracket".to_string(), column: 8, line: 1, value: "]".to_string() },
    ]);
}

#[test]
fn object_with_keywords_token_count() {
    let result = jsonc_lexer_native("{\"a\": true, \"b\": false, \"c\": null}");
    assert_eq!(result.tokens.len(), 18);
}

#[test]
fn object_with_keywords_no_errors() {
    let result = jsonc_lexer_native("{\"a\": true, \"b\": false, \"c\": null}");
    assert_eq!(result.errors.len(), 0);
}

// README Example.
#[test]
fn readme_example_token_count() {
    let input = "{\n    /* JSONC allows comments and trailing commas! */\n    \"key1\": 1, // note the trailing comma after the value \"TWO\":\n    \"key2\": \"TWO\",\n}";
    let result = jsonc_lexer_native(input);
    assert_eq!(result.tokens.len(), 22);
}

#[test]
fn readme_example_no_errors() {
    let input = "{\n    /* JSONC allows comments and trailing commas! */\n    \"key1\": 1, // note the trailing comma after the value \"TWO\":\n    \"key2\": \"TWO\",\n}";
    let result = jsonc_lexer_native(input);
    assert_eq!(result.errors.len(), 0);
}

#[test]
fn readme_example_full() {
    let input = "{\n    /* JSONC allows comments and trailing commas! */\n    \"key1\": 1, // note the trailing comma after the value \"TWO\":\n    \"key2\": \"TWO\",\n}";
    let result = jsonc_lexer_native(input);
    let expected = vec![
        Token { category: "brace".to_string(), column: 0, line: 1, value: "{".to_string() },
        Token { category: "whitespace".to_string(), column: 1, line: 1, value: "\n".to_string() },
        Token { category: "whitespace".to_string(), column: 0, line: 2, value: "    ".to_string() },
        Token { category: "comment".to_string(), column: 4, line: 2, value: "/* JSONC allows comments and trailing commas! */".to_string() },
        Token { category: "whitespace".to_string(), column: 52, line: 2, value: "\n".to_string() },
        Token { category: "whitespace".to_string(), column: 0, line: 3, value: "    ".to_string() },
        Token { category: "string".to_string(), column: 4, line: 3, value: "\"key1\"".to_string() },
        Token { category: "colon".to_string(), column: 10, line: 3, value: ":".to_string() },
        Token { category: "whitespace".to_string(), column: 11, line: 3, value: " ".to_string() },
        Token { category: "number".to_string(), column: 12, line: 3, value: "1".to_string() },
        Token { category: "comma".to_string(), column: 13, line: 3, value: ",".to_string() },
        Token { category: "whitespace".to_string(), column: 14, line: 3, value: " ".to_string() },
        Token { category: "comment".to_string(), column: 15, line: 3, value: "// note the trailing comma after the value \"TWO\":".to_string() },
        Token { category: "whitespace".to_string(), column: 64, line: 3, value: "\n".to_string() },
        Token { category: "whitespace".to_string(), column: 0, line: 4, value: "    ".to_string() },
        Token { category: "string".to_string(), column: 4, line: 4, value: "\"key2\"".to_string() },
        Token { category: "colon".to_string(), column: 10, line: 4, value: ":".to_string() },
        Token { category: "whitespace".to_string(), column: 11, line: 4, value: " ".to_string() },
        Token { category: "string".to_string(), column: 12, line: 4, value: "\"TWO\"".to_string() },
        Token { category: "comma".to_string(), column: 17, line: 4, value: ",".to_string() },
        Token { category: "whitespace".to_string(), column: 18, line: 4, value: "\n".to_string() },
        Token { category: "brace".to_string(), column: 0, line: 5, value: "}".to_string() },
    ];
    assert_eq!(result.tokens, expected);
}

// Multi-line.
#[test]
fn multiline_object_line_numbers() {
    let result = jsonc_lexer_native("{\n  \"a\": 1\n}");
    assert_eq!(result.tokens[0].line, 1);
    assert_eq!(result.tokens[3].line, 2); // "a"
    assert_eq!(result.tokens[result.tokens.len() - 1].line, 3); // }
}

// Edge Cases.
#[test]
fn lone_slash_has_error() {
    let result = jsonc_lexer_native("/");
    assert!(result.errors.len() > 0);
}

#[test]
fn lone_slash_message() {
    let result = jsonc_lexer_native("/");
    assert!(result.errors[0].message.contains("Unexpected character '/'"));
}

#[test]
fn invalid_character_error() {
    let result = jsonc_lexer_native("@");
    assert!(result.errors.len() > 0);
}

#[test]
fn trailing_comma_valid() {
    let result = jsonc_lexer_native("[1,]");
    assert_eq!(result.errors.len(), 0);
}

#[test]
fn trailing_comma_has_comma_token() {
    let result = jsonc_lexer_native("[1,]");
    assert!(result.tokens.iter().any(|t| t.category == "comma"));
}

#[test]
fn nested_object_no_errors() {
    let result = jsonc_lexer_native("{\"nested\": {\"deep\": true}}");
    assert_eq!(result.errors.len(), 0);
}

#[test]
fn nested_object_has_4_braces() {
    let result = jsonc_lexer_native("{\"nested\": {\"deep\": true}}");
    let brace_count = result.tokens.iter().filter(|t| t.category == "brace").count();
    assert_eq!(brace_count, 4);
}
