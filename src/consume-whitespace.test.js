/**
 * @fileoverview
 * Tests for consume-whitespace.js
 */

import { deepStrictEqual as deq, strictEqual as eq } from 'node:assert';
import { consumeWhitespace as fn } from './consume-whitespace.js';

// Single space.
eq(fn(' ', 0, 1, 1, 0).token.value, ' ');
eq(fn(' ', 0, 1, 1, 0).pos, 1);
eq(fn(' ', 0, 1, 1, 0).column, 1);
eq(fn(' ', 0, 1, 1, 0).line, 1);

// Multiple spaces.
eq(fn('    ', 0, 4, 1, 0).token.value, '    ');
eq(fn('    ', 0, 4, 1, 0).pos, 4);
eq(fn('    ', 0, 4, 1, 0).column, 4);

// Single tab.
eq(fn('\t', 0, 1, 1, 0).token.value, '\t');
eq(fn('\t', 0, 1, 1, 0).pos, 1);
eq(fn('\t', 0, 1, 1, 0).column, 1);

// Mixed spaces and tabs.
eq(fn('  \t \t', 0, 5, 1, 0).token.value, '  \t \t');
eq(fn('  \t \t', 0, 5, 1, 0).pos, 5);
eq(fn('  \t \t', 0, 5, 1, 0).column, 5);

// Single newline.
eq(fn('\n', 0, 1, 1, 0).token.value, '\n');
eq(fn('\n', 0, 1, 1, 0).pos, 1);
eq(fn('\n', 0, 1, 1, 0).column, 0);
eq(fn('\n', 0, 1, 1, 0).line, 2);

// CRLF sequence.
eq(fn('\r\n', 0, 2, 1, 0).token.value, '\r\n');
eq(fn('\r\n', 0, 2, 1, 0).pos, 2);
eq(fn('\r\n', 0, 2, 1, 0).column, 0);
eq(fn('\r\n', 0, 2, 1, 0).line, 2);

// Lone CR.
eq(fn('\r', 0, 1, 1, 0).token.value, '\r');
eq(fn('\r', 0, 1, 1, 0).pos, 1);
eq(fn('\r', 0, 1, 1, 0).column, 0);
eq(fn('\r', 0, 1, 1, 0).line, 2);

// Spaces before newline (should stop at newline).
eq(fn('  \n  ', 0, 5, 1, 0).token.value, '  ');
eq(fn('  \n  ', 0, 5, 1, 0).pos, 2);
eq(fn('  \n  ', 0, 5, 1, 0).column, 2);
eq(fn('  \n  ', 0, 5, 1, 0).line, 1);

// Newline followed by spaces (consume just newline).
eq(fn('\n  ', 0, 3, 1, 0).token.value, '\n');
eq(fn('\n  ', 0, 3, 1, 0).pos, 1);
eq(fn('\n  ', 0, 3, 1, 0).column, 0);
eq(fn('\n  ', 0, 3, 1, 0).line, 2);

// Whitespace followed by non-whitespace.
eq(fn('  x', 0, 3, 1, 0).token.value, '  ');
eq(fn('  x', 0, 3, 1, 0).pos, 2);
eq(fn('  x', 0, 3, 1, 0).column, 2);

// Starting from middle position.
eq(fn('x   y', 1, 5, 1, 10).token.value, '   ');
eq(fn('x   y', 1, 5, 1, 10).pos, 4);
eq(fn('x   y', 1, 5, 1, 10).column, 13);

// Token metadata.
deq(fn('  ', 0, 2, 5, 10).token, {
    category: 'whitespace',
    column: 10,
    line: 5,
    value: '  '
});
