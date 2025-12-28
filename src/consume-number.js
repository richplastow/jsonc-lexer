/**
 * @fileoverview
 * Consumes a JSON number from JSONC input.
 */

import { isDigit } from './is-digit.js';

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
export function consumeNumber(jsonc, startPos, len, line, startColumn, errors) {
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
