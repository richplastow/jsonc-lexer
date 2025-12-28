/**
 * @fileoverview
 * Helper function to check if a character is a digit.
 */

/**
 * Checks if a character is a digit (0-9).
 * @param {string} ch - Single character
 * @returns {boolean}
 */
export function isDigit(ch) {
    const code = ch.charCodeAt(0);
    return code >= 48 && code <= 57; // '0' = 48, '9' = 57
}
