#!/usr/bin/env node
/**
 * @fileoverview
 * JavaScript-only build of jsonc-lexer.
 * 
 * Minimal lexical tokenizer for JSONC, that also works for JSON.
 * 
 * - Version: 1.0.0
 * - License: MIT
 * - GitHub: <https://github.com/richplastow/jsonc-lexer>
 * - Live demo: <https://richplastow.com/jsonc-lexer/>
 */

/**
 * @fileoverview
 * Consumes a block comment from JSONC input.
 */

/**
 * @typedef {import('./tokenize-jsonc.js').Token} Token
 * @typedef {import('./tokenize-jsonc.js').TokenizeError} TokenizeError
 */

/**
 * Result from consumeBlockComment.
 * @typedef {Object} ConsumeResult
 * @property {number} column - Updated column position
 * @property {number} line - Updated line number
 * @property {number} pos - Updated position in the string
 * @property {Token} token - The consumed comment token
 */

/**
 * Consumes a block comment.
 * @param {string} jsonc - The input string
 * @param {number} startPos - Starting position (at the first '/')
 * @param {number} len - Length of input
 * @param {number} startLine - Starting line number
 * @param {number} startColumn - Starting column
 * @param {TokenizeError[]} errors - Errors array to append to
 * @returns {ConsumeResult}
 */
function consumeBlockComment(jsonc, startPos, len, startLine, startColumn, errors) {
    let pos = startPos + 2; // skip '/*'
    let line = startLine;
    let column = startColumn + 2;
    let value = '/*';

    // Consume until '*/' or end of input.
    while (pos < len) {
        const ch = jsonc[pos];

        // Check for closing '*/'.
        if (ch === '*' && pos + 1 < len && jsonc[pos + 1] === '/') {
            value += '*/';
            pos += 2;
            column += 2;
            return {
                column,
                line,
                pos,
                token: { category: 'comment', column: startColumn, line: startLine, value },
            };
        }

        // Handle newlines inside block comment.
        if (ch === '\n') {
            value += ch;
            pos += 1;
            line += 1;
            column = 0;
            continue;
        }
        if (ch === '\r') {
            value += ch;
            pos += 1;
            // Handle \r\n.
            if (pos < len && jsonc[pos] === '\n') {
                value += '\n';
                pos += 1;
            }
            line += 1;
            column = 0;
            continue;
        }

        // Regular character.
        value += ch;
        pos += 1;
        column += 1;
    }

    // Reached end without closing '*/'.
    errors.push({
        category: 'syntax',
        column: startColumn,
        line: startLine,
        message: 'Unterminated block comment',
    });

    return {
        column,
        line,
        pos,
        token: { category: 'comment', column: startColumn, line: startLine, value },
    };
}

/**
 * @fileoverview
 * Consumes a JSON keyword from JSONC input.
 */

/**
 * @typedef {import('./tokenize-jsonc.js').Token} Token
 * @typedef {import('./tokenize-jsonc.js').TokenizeError} TokenizeError
 */

/**
 * Result from consumeKeyword.
 * @typedef {Object} ConsumeResult
 * @property {number} column - Updated column position
 * @property {number} line - Line number (unchanged for keywords)
 * @property {number} pos - Updated position in the string
 * @property {Token} token - The consumed keyword token
 */

/**
 * Attempts to consume a keyword (true, false, null).
 * @param {string} jsonc - The input string
 * @param {number} startPos - Starting position
 * @param {number} len - Length of input
 * @param {number} line - Current line number
 * @param {number} startColumn - Starting column
 * @param {TokenizeError[]} errors - Errors array to append to
 * @returns {ConsumeResult|null} The result or null if no keyword matches
 */
function consumeKeyword(jsonc, startPos, len, line, startColumn, errors) {
    // Check for 'true'.
    if (startPos + 4 <= len && jsonc.slice(startPos, startPos + 4) === 'true') {
        return {
            column: startColumn + 4,
            line,
            pos: startPos + 4,
            token: { category: 'keyword', column: startColumn, line, value: 'true' },
        };
    }

    // Check for 'false'.
    if (startPos + 5 <= len && jsonc.slice(startPos, startPos + 5) === 'false') {
        return {
            column: startColumn + 5,
            line,
            pos: startPos + 5,
            token: { category: 'keyword', column: startColumn, line, value: 'false' },
        };
    }

    // Check for 'null'.
    if (startPos + 4 <= len && jsonc.slice(startPos, startPos + 4) === 'null') {
        return {
            column: startColumn + 4,
            line,
            pos: startPos + 4,
            token: { category: 'keyword', column: startColumn, line, value: 'null' },
        };
    }

    // No keyword matched.
    return null;
}

/**
 * @fileoverview
 * Consumes a single-line comment from JSONC input.
 */

/**
 * @typedef {import('./tokenize-jsonc.js').Token} Token
 */

/**
 * Result from consumeLineComment.
 * @typedef {Object} ConsumeResult
 * @property {number} column - Updated column position
 * @property {number} line - Line number (unchanged for line comments)
 * @property {number} pos - Updated position in the string
 * @property {Token} token - The consumed comment token
 */

/**
 * Consumes a single-line comment (// ...).
 * @param {string} jsonc - The input string
 * @param {number} startPos - Starting position (at the first '/')
 * @param {number} len - Length of input
 * @param {number} line - Current line number
 * @param {number} startColumn - Starting column
 * @returns {ConsumeResult}
 */
function consumeLineComment(jsonc, startPos, len, line, startColumn) {
    let pos = startPos + 2; // skip '//'
    let column = startColumn + 2;
    let value = '//';

    // Consume until newline or end of input.
    while (pos < len) {
        const ch = jsonc[pos];
        if (ch === '\n' || ch === '\r') {
            break;
        }
        value += ch;
        pos += 1;
        column += 1;
    }

    return {
        column,
        line,
        pos,
        token: { category: 'comment', column: startColumn, line, value },
    };
}

/**
 * @fileoverview
 * Helper function to check if a character is a digit.
 */

/**
 * Checks if a character is a digit (0-9).
 * @param {string} ch - Single character
 * @returns {boolean}
 */
function isDigit(ch) {
    const code = ch.charCodeAt(0);
    return code >= 48 && code <= 57; // '0' = 48, '9' = 57
}

/**
 * @fileoverview
 * Consumes a JSON number from JSONC input.
 */


/**
 * @typedef {import('./tokenize-jsonc.js').Token} Token
 * @typedef {import('./tokenize-jsonc.js').TokenizeError} TokenizeError
 */

/**
 * Result from consumeNumber.
 * @typedef {Object} ConsumeResult
 * @property {number} column - Updated column position
 * @property {number} line - Line number (unchanged for numbers)
 * @property {number} pos - Updated position in the string
 * @property {Token} token - The consumed number token
 */

/**
 * Consumes a JSON number.
 * @param {string} jsonc - The input string
 * @param {number} startPos - Starting position
 * @param {number} len - Length of input
 * @param {number} line - Current line number
 * @param {number} startColumn - Starting column
 * @param {TokenizeError[]} errors - Errors array to append to
 * @returns {ConsumeResult}
 */
function consumeNumber(jsonc, startPos, len, line, startColumn, errors) {
    let pos = startPos;
    let column = startColumn;
    let value = '';

    // Optional leading minus.
    if (pos < len && jsonc[pos] === '-') {
        value += '-';
        pos += 1;
        column += 1;
    }

    // Integer part.
    if (pos < len && jsonc[pos] === '0') {
        // Leading zero must not be followed by another digit.
        value += '0';
        pos += 1;
        column += 1;
    } else if (pos < len && isDigit(jsonc[pos])) {
        // Non-zero digit followed by optional digits.
        while (pos < len && isDigit(jsonc[pos])) {
            value += jsonc[pos];
            pos += 1;
            column += 1;
        }
    } else {
        // No digit after minus.
        errors.push({
            category: 'syntax',
            column: startColumn,
            line,
            message: 'Invalid number: expected digit',
        });
        return {
            column,
            line,
            pos,
            token: { category: 'number', column: startColumn, line, value },
        };
    }

    // Optional fractional part.
    if (pos < len && jsonc[pos] === '.') {
        value += '.';
        pos += 1;
        column += 1;
        // Must have at least one digit after decimal point.
        if (pos >= len || !isDigit(jsonc[pos])) {
            errors.push({
                category: 'syntax',
                column,
                line,
                message: 'Invalid number: expected digit after decimal point',
            });
        } else {
            while (pos < len && isDigit(jsonc[pos])) {
                value += jsonc[pos];
                pos += 1;
                column += 1;
            }
        }
    }

    // Optional exponent part.
    if (pos < len && (jsonc[pos] === 'e' || jsonc[pos] === 'E')) {
        value += jsonc[pos];
        pos += 1;
        column += 1;
        // Optional sign.
        if (pos < len && (jsonc[pos] === '+' || jsonc[pos] === '-')) {
            value += jsonc[pos];
            pos += 1;
            column += 1;
        }
        // Must have at least one digit.
        if (pos >= len || !isDigit(jsonc[pos])) {
            errors.push({
                category: 'syntax',
                column,
                line,
                message: 'Invalid number: expected digit in exponent',
            });
        } else {
            while (pos < len && isDigit(jsonc[pos])) {
                value += jsonc[pos];
                pos += 1;
                column += 1;
            }
        }
    }

    return {
        column,
        line,
        pos,
        token: { category: 'number', column: startColumn, line, value },
    };
}

/**
 * @fileoverview
 * Helper function to check if a character is a hexadecimal digit.
 */

/**
 * Checks if a character is a hex digit (0-9, a-f, A-F).
 * @param {string} ch - Single character
 * @returns {boolean}
 */
function isHexDigit(ch) {
    const code = ch.charCodeAt(0);
    return (code >= 48 && code <= 57) // 0-9
        || (code >= 65 && code <= 70) // A-F
        || (code >= 97 && code <= 102); // a-f
}

/**
 * @fileoverview
 * Consumes a double-quoted string from JSONC input.
 */


/**
 * @typedef {import('./tokenize-jsonc.js').Token} Token
 * @typedef {import('./tokenize-jsonc.js').TokenizeError} TokenizeError
 */

/**
 * Result from consumeString.
 * @typedef {Object} ConsumeResult
 * @property {number} column - Updated column position
 * @property {number} line - Line number (unchanged for strings)
 * @property {number} pos - Updated position in the string
 * @property {Token} token - The consumed string token
 */

/**
 * Consumes a double-quoted string.
 * @param {string} jsonc - The input string
 * @param {number} startPos - Starting position (at the opening quote)
 * @param {number} len - Length of input
 * @param {number} line - Current line number
 * @param {number} startColumn - Starting column
 * @param {TokenizeError[]} errors - Errors array to append to
 * @returns {ConsumeResult}
 */
function consumeString(jsonc, startPos, len, line, startColumn, errors) {
    let pos = startPos + 1; // skip opening quote
    let column = startColumn + 1;
    let value = '"';

    while (pos < len) {
        const ch = jsonc[pos];

        // Closing quote found.
        if (ch === '"') {
            value += '"';
            pos += 1;
            column += 1;
            return {
                column,
                line,
                pos,
                token: { category: 'string', column: startColumn, line, value },
            };
        }

        // Escape sequence.
        if (ch === '\\') {
            if (pos + 1 >= len) {
                errors.push({
                    category: 'syntax',
                    column,
                    line,
                    message: 'Unterminated escape sequence',
                });
                value += ch;
                pos += 1;
                column += 1;
                break;
            }
            const next = jsonc[pos + 1];
            // Valid escape characters: " \ / b f n r t u
            if (next === '"' || next === '\\' || next === '/' ||
                next === 'b' || next === 'f' || next === 'n' ||
                next === 'r' || next === 't') {
                value += ch + next;
                pos += 2;
                column += 2;
                continue;
            }
            // Unicode escape: \uXXXX
            if (next === 'u') {
                if (pos + 5 >= len) {
                    errors.push({
                        category: 'syntax',
                        column,
                        line,
                        message: 'Incomplete unicode escape sequence',
                    });
                    value += ch + next;
                    pos += 2;
                    column += 2;
                    continue;
                }
                // Check for 4 hex digits.
                let valid = true;
                for (let i = 0; i < 4; i++) {
                    if (!isHexDigit(jsonc[pos + 2 + i])) {
                        valid = false;
                        break;
                    }
                }
                if (!valid) {
                    errors.push({
                        category: 'syntax',
                        column,
                        line,
                        message: 'Invalid unicode escape sequence',
                    });
                }
                // Consume \uXXXX regardless.
                value += jsonc.slice(pos, pos + 6);
                pos += 6;
                column += 6;
                continue;
            }
            // Invalid escape character.
            errors.push({
                category: 'syntax',
                column,
                line,
                message: `Invalid escape character '\\${next}'`,
            });
            value += ch + next;
            pos += 2;
            column += 2;
            continue;
        }

        // Unescaped newline in string is an error.
        if (ch === '\n' || ch === '\r') {
            errors.push({
                category: 'syntax',
                column: startColumn,
                line,
                message: 'Unterminated string literal',
            });
            break;
        }

        // Control characters (U+0000 to U+001F) should be escaped.
        const code = ch.charCodeAt(0);
        if (code < 32) {
            errors.push({
                category: 'syntax',
                column,
                line,
                message: 'Unescaped control character in string',
            });
        }

        // Regular character.
        value += ch;
        pos += 1;
        column += 1;
    }

    // Reached end without closing quote.
    if (value[value.length - 1] !== '"') {
        errors.push({
            category: 'syntax',
            column: startColumn,
            line,
            message: 'Unterminated string literal',
        });
    }

    return {
        column,
        line,
        pos,
        token: { category: 'string', column: startColumn, line, value },
    };
}

/**
 * @fileoverview
 * Consumes whitespace characters from JSONC input.
 */

/**
 * @typedef {import('./tokenize-jsonc.js').Token} Token
 */

/**
 * Result from consumeWhitespace.
 * @typedef {Object} ConsumeResult
 * @property {number} column - Updated column position
 * @property {number} line - Updated line number
 * @property {number} pos - Updated position in the string
 * @property {Token} token - The consumed whitespace token
 */

/**
 * Consumes contiguous whitespace characters.
 * @param {string} jsonc - The input string
 * @param {number} startPos - Starting position
 * @param {number} len - Length of input
 * @param {number} startLine - Starting line number
 * @param {number} startColumn - Starting column
 * @returns {ConsumeResult}
 */
function consumeWhitespace(jsonc, startPos, len, startLine, startColumn) {
    let pos = startPos;
    let line = startLine;
    let column = startColumn;
    let value = '';

    // Consume until non-whitespace or newline boundary.
    while (pos < len) {
        const ch = jsonc[pos];

        // Newline ends this whitespace token (newline is its own token).
        if (ch === '\n') {
            if (value === '') {
                // Newline is the first char, include it and stop.
                value = '\n';
                pos += 1;
                line += 1;
                column = 0;
            }
            // If we already have content, stop before the newline.
            break;
        }

        // Carriage return: handle \r\n as single newline.
        if (ch === '\r') {
            if (value === '') {
                // Check for \r\n sequence.
                if (pos + 1 < len && jsonc[pos + 1] === '\n') {
                    value = '\r\n';
                    pos += 2;
                } else {
                    value = '\r';
                    pos += 1;
                }
                line += 1;
                column = 0;
            }
            break;
        }

        // Space or tab: accumulate.
        if (ch === ' ' || ch === '\t') {
            value += ch;
            pos += 1;
            column += 1;
            continue;
        }

        // Non-whitespace: stop.
        break;
    }

    return {
        column,
        line,
        pos,
        token: { category: 'whitespace', column: startColumn, line: startLine, value },
    };
}

/**
 * @fileoverview
 * Tokenizes JSONC (JSON with Comments) into an array of token objects.
 * 
 * Uses simple for/while loops over characters for easy porting to Rust.
 */


// ================================ TYPES ================================

/**
 * The category of a token.
 * @typedef {'brace'|'bracket'|'colon'|'comma'|'comment'|'keyword'|'number'|'string'|'whitespace'} TokenCategory
 */

/**
 * A single token from the JSONC input.
 * @typedef {Object} Token
 * @property {TokenCategory} category - The type of token
 * @property {number} column - Zero-based column position
 * @property {number} line - One-based line number
 * @property {string} value - The raw token value
 */

/**
 * A syntax error encountered during tokenization.
 * @typedef {Object} TokenizeError
 * @property {'syntax'} category - Always 'syntax' for tokenization errors
 * @property {number} column - Zero-based column where the error occurred
 * @property {number} line - One-based line number where the error occurred
 * @property {string} message - Human-readable error description
 */

/**
 * The result of tokenizing a JSONC string.
 * @typedef {Object} TokenizeResult
 * @property {TokenizeError[]} errors - Any syntax errors encountered
 * @property {Token[]} tokens - The parsed tokens
 */

// ================================ MAIN ================================

/**
 * Tokenizes a JSONC string into tokens and errors.
 * @param {string} jsonc - The JSONC string to tokenize
 * @returns {TokenizeResult} The tokens and any errors found
 */
function tokenizeJsonc(jsonc) {
    /** @type {Token[]} */
    const tokens = [];
    /** @type {TokenizeError[]} */
    const errors = [];

    const len = jsonc.length;
    let pos = 0; // current position in the string
    let line = 1; // one-based line number
    let column = 0; // zero-based column position

    // Main tokenization loop.
    while (pos < len) {
        const ch = jsonc[pos];

        // Whitespace: space, tab, carriage return, or newline.
        if (ch === ' ' || ch === '\t' || ch === '\r' || ch === '\n') {
            const result = consumeWhitespace(jsonc, pos, len, line, column);
            tokens.push(result.token);
            pos = result.pos;
            line = result.line;
            column = result.column;
            continue;
        }

        // Single-character structural tokens.
        if (ch === '{' || ch === '}') {
            tokens.push({ category: 'brace', column, line, value: ch });
            pos += 1;
            column += 1;
            continue;
        }
        if (ch === '[' || ch === ']') {
            tokens.push({ category: 'bracket', column, line, value: ch });
            pos += 1;
            column += 1;
            continue;
        }
        if (ch === ':') {
            tokens.push({ category: 'colon', column, line, value: ch });
            pos += 1;
            column += 1;
            continue;
        }
        if (ch === ',') {
            tokens.push({ category: 'comma', column, line, value: ch });
            pos += 1;
            column += 1;
            continue;
        }

        // String: starts with double quote.
        if (ch === '"') {
            const result = consumeString(jsonc, pos, len, line, column, errors);
            tokens.push(result.token);
            pos = result.pos;
            column = result.column;
            continue;
        }

        // Comment: starts with '/'.
        if (ch === '/') {
            const next = pos + 1 < len ? jsonc[pos + 1] : '';
            if (next === '/') {
                const result = consumeLineComment(jsonc, pos, len, line, column);
                tokens.push(result.token);
                pos = result.pos;
                column = result.column;
                continue;
            }
            if (next === '*') {
                const result = consumeBlockComment(jsonc, pos, len, line, column, errors);
                tokens.push(result.token);
                pos = result.pos;
                line = result.line;
                column = result.column;
                continue;
            }
            // Lone '/' is invalid in JSONC.
            errors.push({
                category: 'syntax',
                column,
                line,
                message: "Unexpected character '/'",
            });
            pos += 1;
            column += 1;
            continue;
        }

        // Number: starts with digit or minus sign.
        if (isDigit(ch) || ch === '-') {
            const result = consumeNumber(jsonc, pos, len, line, column, errors);
            tokens.push(result.token);
            pos = result.pos;
            column = result.column;
            continue;
        }

        // Keywords: true, false, null.
        if (ch === 't' || ch === 'f' || ch === 'n') {
            const result = consumeKeyword(jsonc, pos, len, line, column);
            if (result) {
                tokens.push(result.token);
                pos = result.pos;
                column = result.column;
                continue;
            }
            // Fall through to unexpected character if keyword didn't match.
        }

        // Unexpected character.
        errors.push({
            category: 'syntax',
            column,
            line,
            message: `Unexpected character '${ch}'`,
        });
        pos += 1;
        column += 1;
    }

    return { errors, tokens };
}

const BUILD_VARIANT = 'js-only'; // may have ".min" appended during build
const PACKAGE_VERSION = '1.0.0'; // will be checked during build

/**
 * @typedef {import('./tokenize-jsonc.js').Token} Token
 * @typedef {import('./tokenize-jsonc.js').TokenizeError} TokenizeError
 * @typedef {import('./tokenize-jsonc.js').TokenizeResult} TokenizeResult
 */

/**
 * @typedef {Object} JsoncLexerOptions
 * @property {boolean} [debug] - Optional debug flag, false by default
 * @property {'always'|'auto'|'never'} [useWasm] - Ignored in JS-only build
 */

/**
 * @typedef {Object} JsoncLexerDebugOutput
 * @property {string} buildVariant - The build variant used, e.g. 'js-only.min'
 * @property {TokenizeError[]} [errors] - Any errors encountered during tokenization
 * @property {'js'|'wasm'} implementationUsed - Whether JavaScript or WASM was used to generate these tokens
 * @property {number} lengthInput - The length of the input JSONC string
 * @property {number} lengthOutput - The number of tokens generated
 * @property {Token[]} tokens - The generated tokens
 * @property {string} processingTimeMs - Time taken to process the input, in milliseconds, to 3 fixed decimal places
 * @property {string} version - The jsonc-lexer package version
 */

/**
 * Transforms JSONC into an array of token objects.
 * @param {string} jsoncString
 * @param {JsoncLexerOptions} [options]
 * @returns {Token[]|JsoncLexerDebugOutput}
 */
const jsoncLexer = (jsoncString, options = {}) => {
    let startTime;
    if (options.debug) startTime = performance.now();

    const result = tokenizeJsonc(jsoncString);

    /** @type {TokenizeError[]} */
    const errors = result.errors;
    /** @type {Token[]} */
    const tokens = result.tokens;

    // Usually, return just the tokens.
    if (!options.debug) return tokens;

    // In debug mode, return runtime info along with the tokens.
    const processingTimeMs = performance.now() - startTime;
    return {
        errors,
        buildVariant: BUILD_VARIANT,
        implementationUsed: 'js',
        tokens,
        lengthInput: jsoncString.length,
        lengthOutput: tokens.length,
        processingTimeMs: processingTimeMs.toFixed(3),
        version: PACKAGE_VERSION,
    };
};

// CLI functionality - check if this module is being run from the command line.
if (typeof process === 'object' && Array.isArray(process.argv)) {
    const [, executablePath, jsoncPath] = process.argv;
    if (executablePath && executablePath.endsWith('jsonc-lexer--js-only.js')) {
        if (jsoncPath) { // process the argument as a JSONC string
            const tokens = jsoncLexer(jsoncPath);
            console.log(JSON.stringify(tokens, null, 2));
        } else { // read from stdin
            let input = '';
            process.stdin.on('data', (chunk) => {
                input += chunk;
            });
            process.stdin.on('end', () => {
                const tokens = jsoncLexer(input);
                console.log(JSON.stringify(tokens, null, 2));
            });
        }
    }
}

export { jsoncLexer };
