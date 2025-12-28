import {
  initSync as initSyncWasm,
  jsonc_lexer as wasmJsoncLexer,
} from '../wasm/jsonc_lexer/pkg/jsonc_lexer.js';
import { wasmBase64 } from './wasm-bytes.js';

/**
 * @typedef {import('./tokenize-jsonc.js').TokenizeResult} TokenizeResult
 */

let wasmReady = false;
let wasmError = null;
let cachedBytes = null;

const decodeWasm = () => {
  if (cachedBytes) {
    return cachedBytes;
  }
  if (typeof Buffer === 'function') {
    cachedBytes = Uint8Array.from(Buffer.from(wasmBase64, 'base64'));
    return cachedBytes;
  }
  const binaryString = typeof atob === 'function'
    ? atob(wasmBase64)
    : globalThis.atob(wasmBase64);
  const bytes = new Uint8Array(binaryString.length);
  for (let i = 0; i < binaryString.length; i += 1) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  cachedBytes = bytes;
  return cachedBytes;
};

const ensureWasm = () => {
  if (wasmReady || wasmError) {
    return wasmReady;
  }
  try {
    const bytes = decodeWasm();
    initSyncWasm({ module: bytes });
    wasmReady = true;
    return true;
  } catch (error) {
    wasmError = error;
    return false;
  }
};

/**
 * Returns wasm bindings if instantiation succeeded. Otherwise `null` is
 * returned so callers can gracefully fall back to the JS implementation.
 * @returns {{
 *   jsoncLexer: (input: string) => TokenizeResult,
 * } | null}
 */
export const getWasmApi = () => {
  if (!ensureWasm()) {
    return null;
  }
  return {
    jsoncLexer: wasmJsoncLexer,
  };
};

/**
 * Exposes the underlying instantiation error, if any, which is helpful for
 * surfacing actionable diagnostics in CLI contexts.
 * @returns {Error|null}
 */
export const getWasmError = () => wasmError;
