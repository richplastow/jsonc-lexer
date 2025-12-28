/**
 * @fileoverview
 * Helper function to check if a character is a hexadecimal digit.
 */

/**
 * Checks if a character is a hex digit (0-9, a-f, A-F).
 * @param {string} ch - Single character
 * @returns {boolean}
 */
export function isHexDigit(ch) {
    const code = ch.charCodeAt(0);
    return (code >= 48 && code <= 57) // 0-9
        || (code >= 65 && code <= 70) // A-F
        || (code >= 97 && code <= 102); // a-f
}
