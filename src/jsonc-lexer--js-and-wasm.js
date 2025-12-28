#!/usr/bin/env node

import { getWasmApi } from './wasm-loader.js';

const BUILD_VARIANT = 'js-and-wasm'; // may have ".min" appended during build
const PACKAGE_VERSION = '0.0.1'; // will be checked during build
const WASM_THRESHOLD = 1000;

/**
 * @typedef {Object} JsoncLexerOptions
 * @property {boolean} [debug] - Optional debug flag, false by default
 * @property {'always'|'auto'|'never'} [useWasm] - When to use WASM
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

    const useWasm = normalizeUseWasm(options.useWasm);
    const shouldUseWasm =
        useWasm === 'always'
            || (
                useWasm === 'auto'
                && typeof jsoncString === 'string'
                && jsoncString.length >= WASM_THRESHOLD
            );

    let tokens;
    if (shouldUseWasm) {
        const wasm = getWasmApi();
        if (wasm) {
            tokens = wasm.jsoncLexer(jsoncString);
        } else if (useWasm === 'always') throw new Error(
            'Rust WASM backend was requested but is unavailable. Rebuild the wasm package to enable it.');
    } else {
        tokens = "[789]";
    }

    // Usually, return just the tokens.
    if (!options.debug) return tokens;

    // In debug mode, return runtime info along with the tokens.
    const processingTimeMs = performance.now() - startTime;
    return {
        buildVariant: BUILD_VARIANT,
        implementationUsed: shouldUseWasm ? 'wasm' : 'js',
        tokens,
        lengthInput: jsoncString.length,
        lengthOutput: tokens.length,
        processingTimeMs: processingTimeMs.toFixed(3),
        version: PACKAGE_VERSION,
    };
};

const normalizeUseWasm = (value) => {
    switch (value) {
        case 'always':
        case 'auto':
        case 'never':
            return value;
        default:
            return 'auto';
    }
};

// CLI functionality - check if this module is being run from the command line.
if (typeof process === 'object' && Array.isArray(process.argv)) {
    const [, executablePath, jsoncPath] = process.argv;
    if (executablePath && executablePath.endsWith('jsonc-lexer--js-and-wasm.js')) {
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
