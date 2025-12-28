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
export function consumeLineComment(jsonc, startPos, len, line, startColumn) {
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
