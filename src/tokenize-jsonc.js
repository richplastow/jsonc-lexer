/**
 * @fileoverview
 * Tokenizes JSONC (JSON with Comments) into an array of token objects.
 * 
 * Uses simple for/while loops over characters for easy porting to Rust.
 */

import { consumeBlockComment } from './consume-block-comment.js';
import { consumeKeyword } from './consume-keyword.js';
import { consumeLineComment } from './consume-line-comment.js';
import { consumeNumber } from './consume-number.js';
import { consumeString } from './consume-string.js';
import { consumeWhitespace } from './consume-whitespace.js';
import { isDigit } from './is-digit.js';

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
export function tokenizeJsonc(jsonc) {
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
            const result = consumeKeyword(jsonc, pos, len, line, column, errors);
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
