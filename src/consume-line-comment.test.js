/**
 * @fileoverview
 * Tests for consume-line-comment.js
 */

import { strictEqual as eq } from 'node:assert';
import { consumeLineComment as fn } from './consume-line-comment.js';

// Empty comment.
eq(fn('//', 0, 2, 1, 0).token.value, '//');
eq(fn('//', 0, 2, 1, 0).pos, 2);
eq(fn('//', 0, 2, 1, 0).token.category, 'comment');

// Simple comment.
eq(fn('// hello', 0, 8, 1, 0).token.value, '// hello');
eq(fn('// hello', 0, 8, 1, 0).pos, 8);

// Comment with special characters.
eq(fn('// @TODO: fix this!', 0, 19, 1, 0).token.value, '// @TODO: fix this!');
eq(fn('// @TODO: fix this!', 0, 19, 1, 0).pos, 19);

// Comment stops at newline.
eq(fn('// test\nmore', 0, 12, 1, 0).token.value, '// test');
eq(fn('// test\nmore', 0, 12, 1, 0).pos, 7);
eq(fn('// test\nmore', 0, 12, 1, 0).line, 1);

// Comment stops at CR.
eq(fn('// test\rmore', 0, 12, 1, 0).token.value, '// test');
eq(fn('// test\rmore', 0, 12, 1, 0).pos, 7);

// Comment with trailing spaces.
eq(fn('// test   ', 0, 10, 1, 0).token.value, '// test   ');
eq(fn('// test   ', 0, 10, 1, 0).pos, 10);

// Comment at end of file.
eq(fn('// EOF', 0, 6, 1, 0).token.value, '// EOF');
eq(fn('// EOF', 0, 6, 1, 0).pos, 6);

// Comment starting mid-input.
eq(fn('x// test', 1, 8, 1, 5).token.value, '// test');
eq(fn('x// test', 1, 8, 1, 5).pos, 8);
eq(fn('x// test', 1, 8, 1, 5).column, 12);

// Token metadata.
eq(fn('// x', 0, 4, 5, 10).token.line, 5);
eq(fn('// x', 0, 4, 5, 10).token.column, 10);

// Comment with slashes inside.
eq(fn('// path/to/file', 0, 15, 1, 0).token.value, '// path/to/file');

// Comment with unicode.
{
    const input = '// emoji 🎉';
    eq(fn(input, 0, input.length, 1, 0).token.value, input);
}

// Very long comment.
{
    const longText = '// ' + 'a'.repeat(1000);
    eq(fn(longText, 0, longText.length, 1, 0).token.value.length, 1003);
    eq(fn(longText, 0, longText.length, 1, 0).pos, 1003);
}

// Column tracking.
eq(fn('// test', 0, 7, 1, 0).column, 7);
