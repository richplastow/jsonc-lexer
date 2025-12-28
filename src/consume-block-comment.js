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
export function consumeBlockComment(jsonc, startPos, len, startLine, startColumn, errors) {
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
