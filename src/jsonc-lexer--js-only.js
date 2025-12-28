#!/usr/bin/env node

import { getWasmApi } from './wasm-loader.js';

const BUILD_VARIANT = 'js-only'; // may have ".min" appended during build
const PACKAGE_VERSION = '0.0.1'; // will be checked during build

/**
 * @typedef {Object} JsoncLexerOptions
 * @property {boolean} [debug] - Optional debug flag, false by default
 * @property {'always'|'auto'|'never'} [useWasm] - Ignored in JS-only build
 */

/**
 * @typedef {Object} JsoncLexerDebugOutput
 * @property {string} buildVariant - The build variant used, e.g. 'js-only.min'
 * @property {'js'|'wasm'} implementationUsed - Whether JavaScript or WASM was used to generate these tokens
 * @property {number} lengthInput - The length of the input JSONC string
 * @property {number} lengthOutput - The number of tokens generated
 * @property {string} tokens - The generated tokens
 * @property {string} processingTimeMs - Time taken to process the input, in milliseconds, to 3 fixed decimal places
 * @property {string} version - The jsonc-lexer package version
 */

/**
 * Transforms JSONC into an array of token objects.
 * @param {string} jsoncString
 * @param {JsoncLexerOptions} [options]
 * @returns {string|JsoncLexerDebugOutput}
 */
export const jsoncLexer = (jsoncString, options = {}) => {
    let startTime;
    if (options.debug) startTime = performance.now();

    const tokens = "[456]";

    // Usually, return just the tokens.
    if (!options.debug) return tokens;

    // In debug mode, return runtime info along with the tokens.
    const processingTimeMs = performance.now() - startTime;
    return {
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
