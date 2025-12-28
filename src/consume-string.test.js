/**
 * @fileoverview
 * Tests for consume-string.js
 */

import { strictEqual as eq } from 'node:assert';
import { consumeString as fn } from './consume-string.js';

// Empty string.
{
    const errors = [];
    eq(fn('""', 0, 2, 1, 0, errors).token.value, '""');
    eq(fn('""', 0, 2, 1, 0, errors).pos, 2);
    eq(errors.length, 0);
}

// Simple string.
{
    const errors = [];
    eq(fn('"hello"', 0, 7, 1, 0, errors).token.value, '"hello"');
    eq(fn('"hello"', 0, 7, 1, 0, errors).pos, 7);
    eq(errors.length, 0);
}

// String with spaces.
{
    const errors = [];
    eq(fn('"hello world"', 0, 13, 1, 0, errors).token.value, '"hello world"');
    eq(fn('"hello world"', 0, 13, 1, 0, errors).pos, 13);
}

// Escaped quote.
{
    const errors = [];
    eq(fn('"say \\"hi\\""', 0, 12, 1, 0, errors).token.value, '"say \\"hi\\""');
    eq(errors.length, 0);
}

// Escaped backslash.
{
    const errors = [];
    eq(fn('"back\\\\slash"', 0, 13, 1, 0, errors).token.value, '"back\\\\slash"');
    eq(errors.length, 0);
}

// All valid escapes.
{
    const errors = [];
    const input = '"\\"\\\\\\/\\b\\f\\n\\r\\t"';
    eq(fn(input, 0, input.length, 1, 0, errors).token.value, input);
    eq(errors.length, 0);
}

// Unicode escape.
{
    const errors = [];
    eq(fn('"\\u0041"', 0, 8, 1, 0, errors).token.value, '"\\u0041"');
    eq(errors.length, 0);
}

// Multiple unicode escapes.
{
    const errors = [];
    eq(fn('"\\u0041\\u0042\\u0043"', 0, 20, 1, 0, errors).token.value, '"\\u0041\\u0042\\u0043"');
    eq(errors.length, 0);
}

// Invalid escape character.
{
    const errors = [];
    const result = fn('"bad\\x"', 0, 7, 1, 0, errors);
    eq(errors.length, 1);
    eq(errors[0].message.includes('Invalid escape'), true);
    eq(result.token.value, '"bad\\x"');
}

// Incomplete unicode (too short).
{
    const errors = [];
    fn('"\\u00"', 0, 6, 1, 0, errors);
    eq(errors.length, 1);
    eq(errors[0].message.includes('Incomplete unicode'), true);
}

// Invalid unicode (non-hex chars).
{
    const errors = [];
    fn('"\\u00XY"', 0, 8, 1, 0, errors);
    eq(errors.length, 1);
    eq(errors[0].message.includes('Invalid unicode'), true);
}

// Unterminated string.
{
    const errors = [];
    const result = fn('"hello', 0, 6, 1, 0, errors);
    eq(errors.length, 1);
    eq(errors[0].message.includes('Unterminated'), true);
    eq(result.token.value, '"hello');
}

// Unterminated escape at end.
{
    const errors = [];
    const result = fn('"test\\', 0, 6, 1, 0, errors);
    eq(errors.length >= 1, true);
    eq(result.token.value.includes('\\'), true);
}

// String with newline (error).
{
    const errors = [];
    const result = fn('"hello\nworld"', 0, 13, 1, 0, errors);
    eq(errors.length >= 1, true);
    eq(errors[0].message.includes('Unterminated'), true);
    eq(result.token.value, '"hello');
}

// String with CR (error).
{
    const errors = [];
    fn('"hello\rworld"', 0, 13, 1, 0, errors);
    eq(errors.length >= 1, true);
}

// Control character in string.
{
    const errors = [];
    fn('"\x01"', 0, 3, 1, 0, errors);
    eq(errors.length, 1);
    eq(errors[0].message.includes('control character'), true);
}

// String starting mid-input.
{
    const errors = [];
    const result = fn('x"test"', 1, 7, 1, 5, errors);
    eq(result.token.value, '"test"');
    eq(result.pos, 7);
    eq(result.column, 11);
}

// Token metadata.
{
    const errors = [];
    const result = fn('"x"', 0, 3, 5, 10, errors);
    eq(result.token.category, 'string');
    eq(result.token.line, 5);
    eq(result.token.column, 10);
}

// Mixed escapes and normal chars.
{
    const errors = [];
    eq(fn('"a\\nb\\tc\\rd"', 0, 12, 1, 0, errors).token.value, '"a\\nb\\tc\\rd"');
    eq(errors.length, 0);
}
