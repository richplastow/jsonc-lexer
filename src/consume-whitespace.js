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
export function consumeWhitespace(jsonc, startPos, len, startLine, startColumn) {
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
