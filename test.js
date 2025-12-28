/**
 * @fileoverview
 * Runs all JavaScript unit tests
 * 
 * See also the *_test.rs files in wasm/jsonc_lexer/src/ for Rust/WASM tests.
 */

import './src/consume-block-comment.test.js';
import './src/consume-keyword.test.js';
import './src/consume-line-comment.test.js';
import './src/consume-number.test.js';
import './src/consume-string.test.js';
import './src/consume-whitespace.test.js';
import './src/tokenize-jsonc.test.js';
