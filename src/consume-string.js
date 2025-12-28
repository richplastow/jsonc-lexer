/**
 * @fileoverview
 * Consumes a double-quoted string from JSONC input.
 */

import { isHexDigit } from './is-hex-digit.js';

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
export function consumeString(jsonc, startPos, len, line, startColumn, errors) {
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
