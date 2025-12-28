/**
 * @fileoverview
 * Tests for consume-keyword.js
 */

import { strictEqual as eq } from 'node:assert';
import { consumeKeyword as fn } from './consume-keyword.js';

// true keyword.
eq(fn('true', 0, 4, 1, 0, []).token.value, 'true');
eq(fn('true', 0, 4, 1, 0, []).pos, 4);
eq(fn('true', 0, 4, 1, 0, []).token.category, 'keyword');

// false keyword.
eq(fn('false', 0, 5, 1, 0, []).token.value, 'false');
eq(fn('false', 0, 5, 1, 0, []).pos, 5);
eq(fn('false', 0, 5, 1, 0, []).token.category, 'keyword');

// null keyword.
eq(fn('null', 0, 4, 1, 0, []).token.value, 'null');
eq(fn('null', 0, 4, 1, 0, []).pos, 4);
eq(fn('null', 0, 4, 1, 0, []).token.category, 'keyword');

// Keywords followed by non-identifier chars.
eq(fn('true,', 0, 5, 1, 0, []).token.value, 'true');
eq(fn('true,', 0, 5, 1, 0, []).pos, 4);

eq(fn('false}', 0, 6, 1, 0, []).token.value, 'false');
eq(fn('false}', 0, 6, 1, 0, []).pos, 5);

eq(fn('null]', 0, 5, 1, 0, []).token.value, 'null');
eq(fn('null]', 0, 5, 1, 0, []).pos, 4);

// Keywords mid-input.
eq(fn('xtrue', 1, 5, 1, 5, []).token.value, 'true');
eq(fn('xtrue', 1, 5, 1, 5, []).pos, 5);
eq(fn('xtrue', 1, 5, 1, 5, []).column, 9);

// Token metadata.
eq(fn('true', 0, 4, 5, 10, []).token.line, 5);
eq(fn('true', 0, 4, 5, 10, []).token.column, 10);

// Not keywords - too short.
eq(fn('tru', 0, 3, 1, 0, []), null);
eq(fn('fals', 0, 4, 1, 0, []), null);
eq(fn('nul', 0, 3, 1, 0, []), null);

// Keywords with letters after (still matches the keyword part).
eq(fn('truex', 0, 5, 1, 0, []).token.value, 'true');
eq(fn('truex', 0, 5, 1, 0, []).pos, 4);
eq(fn('falsex', 0, 6, 1, 0, []).token.value, 'false');
eq(fn('falsex', 0, 6, 1, 0, []).pos, 5);
eq(fn('nullx', 0, 5, 1, 0, []).token.value, 'null');
eq(fn('nullx', 0, 5, 1, 0, []).pos, 4);

// Not keywords - completely different.
eq(fn('invalid', 0, 7, 1, 0, []), null);
eq(fn('123', 0, 3, 1, 0, []), null);
eq(fn('xyz', 0, 3, 1, 0, []), null);

// Keywords at end of input.
eq(fn('true', 0, 4, 1, 0, []).pos, 4);
eq(fn('false', 0, 5, 1, 0, []).pos, 5);
eq(fn('null', 0, 4, 1, 0, []).pos, 4);

// Column tracking.
eq(fn('true', 0, 4, 1, 0, []).column, 4);
eq(fn('false', 0, 5, 1, 0, []).column, 5);
eq(fn('null', 0, 4, 1, 0, []).column, 4);
