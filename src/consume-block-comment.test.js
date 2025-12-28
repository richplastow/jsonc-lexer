/**
 * @fileoverview
 * Tests for consume-block-comment.js
 */

import { strictEqual as eq } from 'node:assert';
import { consumeBlockComment as fn } from './consume-block-comment.js';

// Empty comment.
eq(fn('/**/', 0, 4, 1, 0, []).token.value, '/**/');
eq(fn('/**/', 0, 4, 1, 0, []).pos, 4);
eq(fn('/**/', 0, 4, 1, 0, []).token.category, 'comment');

// Simple comment.
eq(fn('/* hello */', 0, 11, 1, 0, []).token.value, '/* hello */');
eq(fn('/* hello */', 0, 11, 1, 0, []).pos, 11);

// Comment with special characters.
eq(fn('/* @TODO: fix! */', 0, 17, 1, 0, []).token.value, '/* @TODO: fix! */');

// Comment with asterisks inside.
eq(fn('/* * */', 0, 7, 1, 0, []).token.value, '/* * */');

// Multi-line comment.
{
    const input = '/* line1\nline2 */';
    eq(fn(input, 0, input.length, 1, 0, []).token.value, input);
    eq(fn(input, 0, input.length, 1, 0, []).line, 2);
    eq(fn(input, 0, input.length, 1, 0, []).column, 8);
}

// Comment with CRLF.
{
    const input = '/* line1\r\nline2 */';
    eq(fn(input, 0, input.length, 1, 0, []).token.value, input);
    eq(fn(input, 0, input.length, 1, 0, []).line, 2);
}

// Comment with CR.
{
    const input = '/* line1\rline2 */';
    eq(fn(input, 0, input.length, 1, 0, []).token.value, input);
    eq(fn(input, 0, input.length, 1, 0, []).line, 2);
}

// Comment starting mid-input.
eq(fn('x/* test */', 1, 11, 1, 5, []).token.value, '/* test */');
eq(fn('x/* test */', 1, 11, 1, 5, []).pos, 11);
eq(fn('x/* test */', 1, 11, 1, 5, []).column, 15);

// Token metadata.
eq(fn('/* x */', 0, 7, 5, 10, []).token.line, 5);
eq(fn('/* x */', 0, 7, 5, 10, []).token.column, 10);

// Comment with slashes.
eq(fn('/* // */', 0, 9, 1, 0, []).token.value, '/* // */');

// Comment with unicode.
{
    const input = '/* emoji 🎉 */';
    eq(fn(input, 0, input.length, 1, 0, []).token.value, input);
}

// Unterminated comment - error case.
{
    const errors = [];
    fn('/* incomplete', 0, 13, 1, 0, errors);
    eq(errors.length >= 1, true);
    eq(errors[0].message.includes('Unterminated'), true);
}

// Unterminated with newline.
{
    const input = '/* test\n';
    const errors = [];
    fn(input, 0, input.length, 1, 0, errors);
    eq(errors.length >= 1, true);
}

// Comment ending with just *.
{
    const errors = [];
    fn('/* test *', 0, 9, 1, 0, errors);
    eq(errors.length >= 1, true);
}

// Very long comment.
{
    const longText = '/* ' + 'a'.repeat(1000) + ' */';
    eq(fn(longText, 0, longText.length, 1, 0, []).token.value.length, 1006);
    eq(fn(longText, 0, longText.length, 1, 0, []).pos, 1006);
}

// Multi-line with multiple newlines.
{
    const input = '/* line1\nline2\nline3 */';
    eq(fn(input, 0, input.length, 1, 0, []).line, 3);
}

// Column tracking.
eq(fn('/* test */', 0, 10, 1, 0, []).column, 10);

// Starting column with offset.
eq(fn('  /* test */', 2, 12, 1, 2, []).column, 12);
