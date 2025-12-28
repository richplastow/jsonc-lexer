/**
 * @fileoverview
 * Tests for consume-number.js
 */

import { strictEqual as eq } from 'node:assert';
import { consumeNumber as fn } from './consume-number.js';

// Simple integers.
eq(fn('0', 0, 1, 1, 0, []).token.value, '0');
eq(fn('0', 0, 1, 1, 0, []).pos, 1);
eq(fn('0', 0, 1, 1, 0, []).token.category, 'number');

eq(fn('1', 0, 1, 1, 0, []).token.value, '1');
eq(fn('123', 0, 3, 1, 0, []).token.value, '123');
eq(fn('9876543210', 0, 10, 1, 0, []).token.value, '9876543210');

// Negative integers.
eq(fn('-0', 0, 2, 1, 0, []).token.value, '-0');
eq(fn('-1', 0, 2, 1, 0, []).token.value, '-1');
eq(fn('-123', 0, 4, 1, 0, []).token.value, '-123');

// Decimals.
eq(fn('0.0', 0, 3, 1, 0, []).token.value, '0.0');
eq(fn('1.5', 0, 3, 1, 0, []).token.value, '1.5');
eq(fn('123.456', 0, 7, 1, 0, []).token.value, '123.456');
eq(fn('-123.456', 0, 8, 1, 0, []).token.value, '-123.456');

// Scientific notation.
eq(fn('1e10', 0, 4, 1, 0, []).token.value, '1e10');
eq(fn('1E10', 0, 4, 1, 0, []).token.value, '1E10');
eq(fn('1e+10', 0, 5, 1, 0, []).token.value, '1e+10');
eq(fn('1e-10', 0, 5, 1, 0, []).token.value, '1e-10');
eq(fn('1.23e45', 0, 7, 1, 0, []).token.value, '1.23e45');
eq(fn('-1.23e-45', 0, 9, 1, 0, []).token.value, '-1.23e-45');

// Number stops at non-digit.
eq(fn('123abc', 0, 6, 1, 0, []).token.value, '123');
eq(fn('123abc', 0, 6, 1, 0, []).pos, 3);

// Number mid-input.
eq(fn('x123', 1, 4, 1, 5, []).token.value, '123');
eq(fn('x123', 1, 4, 1, 5, []).pos, 4);
eq(fn('x123', 1, 4, 1, 5, []).column, 8);

// Token metadata.
eq(fn('123', 0, 3, 5, 10, []).token.line, 5);
eq(fn('123', 0, 3, 5, 10, []).token.column, 10);

// Leading zero is allowed for 0.
eq(fn('0', 0, 1, 1, 0, []).token.value, '0');

// Invalid: just minus.
{
    const errors = [];
    fn('-', 0, 1, 1, 0, errors);
    eq(errors.length >= 1, true);
}

// Leading zero - just consumes the 0.
eq(fn('01', 0, 2, 1, 0, []).token.value, '0');
eq(fn('01', 0, 2, 1, 0, []).pos, 1);

// Invalid: decimal without fraction.
{
    const errors = [];
    fn('1.', 0, 2, 1, 0, errors);
    eq(errors.length >= 1, true);
}

// Invalid: exponent without digits.
{
    const errors = [];
    fn('1e', 0, 2, 1, 0, errors);
    eq(errors.length >= 1, true);
}

// Invalid: exponent sign without digits.
{
    const errors = [];
    fn('1e+', 0, 3, 1, 0, errors);
    eq(errors.length >= 1, true);
}

// Complex valid number.
eq(fn('-123.456e-789', 0, 13, 1, 0, []).token.value, '-123.456e-789');

// Column tracking.
eq(fn('123', 0, 3, 1, 0, []).column, 3);
