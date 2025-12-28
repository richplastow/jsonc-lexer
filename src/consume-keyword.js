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
export function consumeKeyword(jsonc, startPos, len, line, startColumn, errors) {
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
