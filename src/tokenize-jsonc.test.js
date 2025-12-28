/**
 * @fileoverview
 * Tests for tokenize-jsonc.js
 */

import { strictEqual as eq, deepStrictEqual as deq } from 'node:assert';
import { tokenizeJsonc as fn } from './tokenize-jsonc.js';

// Empty Input.
eq(fn('').tokens.length, 0);
eq(fn('').errors.length, 0);

// Whitespace.
deq(fn(' ').tokens, [
    { category: 'whitespace', column: 0, line: 1, value: ' ' }
]);

deq(fn('    ').tokens, [
    { category: 'whitespace', column: 0, line: 1, value: '    ' }
]);

deq(fn('\t').tokens, [
    { category: 'whitespace', column: 0, line: 1, value: '\t' }
]);

deq(fn('\n').tokens, [
    { category: 'whitespace', column: 0, line: 1, value: '\n' }
]);

deq(fn('  \n  ').tokens, [
    { category: 'whitespace', column: 0, line: 1, value: '  ' },
    { category: 'whitespace', column: 2, line: 1, value: '\n' },
    { category: 'whitespace', column: 0, line: 2, value: '  ' }
]);

deq(fn('\r\n').tokens, [
    { category: 'whitespace', column: 0, line: 1, value: '\r\n' }
]);

// Braces and Brackets.
deq(fn('{}').tokens, [
    { category: 'brace', column: 0, line: 1, value: '{' },
    { category: 'brace', column: 1, line: 1, value: '}' }
]);

deq(fn('[]').tokens, [
    { category: 'bracket', column: 0, line: 1, value: '[' },
    { category: 'bracket', column: 1, line: 1, value: ']' }
]);

deq(fn('{ }').tokens, [
    { category: 'brace', column: 0, line: 1, value: '{' },
    { category: 'whitespace', column: 1, line: 1, value: ' ' },
    { category: 'brace', column: 2, line: 1, value: '}' }
]);

// Strings.
deq(fn('""').tokens, [
    { category: 'string', column: 0, line: 1, value: '""' }
]);

deq(fn('"hello"').tokens, [
    { category: 'string', column: 0, line: 1, value: '"hello"' }
]);

deq(fn('"hello world"').tokens, [
    { category: 'string', column: 0, line: 1, value: '"hello world"' }
]);

deq(fn('"line\\nbreak"').tokens, [
    { category: 'string', column: 0, line: 1, value: '"line\\nbreak"' }
]);

deq(fn('"say \\"hi\\""').tokens, [
    { category: 'string', column: 0, line: 1, value: '"say \\"hi\\""' }
]);

deq(fn('"back\\\\slash"').tokens, [
    { category: 'string', column: 0, line: 1, value: '"back\\\\slash"' }
]);

deq(fn('"\\u0041"').tokens, [
    { category: 'string', column: 0, line: 1, value: '"\\u0041"' }
]);

eq(fn('"unterminated').errors.length > 0, true);
eq(fn('"unterminated').errors[0].message.includes('Unterminated'), true);

eq(fn('"bad\\x"').errors.length > 0, true);
eq(fn('"bad\\x"').errors[0].message.includes('Invalid escape'), true);

// Numbers.
deq(fn('0').tokens, [
    { category: 'number', column: 0, line: 1, value: '0' }
]);

deq(fn('123').tokens, [
    { category: 'number', column: 0, line: 1, value: '123' }
]);

deq(fn('-42').tokens, [
    { category: 'number', column: 0, line: 1, value: '-42' }
]);

deq(fn('3.14').tokens, [
    { category: 'number', column: 0, line: 1, value: '3.14' }
]);

deq(fn('-0.5').tokens, [
    { category: 'number', column: 0, line: 1, value: '-0.5' }
]);

deq(fn('1e10').tokens, [
    { category: 'number', column: 0, line: 1, value: '1e10' }
]);

deq(fn('1E10').tokens, [
    { category: 'number', column: 0, line: 1, value: '1E10' }
]);

deq(fn('2.5e-3').tokens, [
    { category: 'number', column: 0, line: 1, value: '2.5e-3' }
]);

deq(fn('1e+5').tokens, [
    { category: 'number', column: 0, line: 1, value: '1e+5' }
]);

eq(fn('1.').errors.length > 0, true);
eq(fn('1e').errors.length > 0, true);

// Keywords.
deq(fn('true').tokens, [
    { category: 'keyword', column: 0, line: 1, value: 'true' }
]);

deq(fn('false').tokens, [
    { category: 'keyword', column: 0, line: 1, value: 'false' }
]);

deq(fn('null').tokens, [
    { category: 'keyword', column: 0, line: 1, value: 'null' }
]);

// Comments.
deq(fn('// hello').tokens, [
    { category: 'comment', column: 0, line: 1, value: '// hello' }
]);

deq(fn('//').tokens, [
    { category: 'comment', column: 0, line: 1, value: '//' }
]);

deq(fn('/* hello */').tokens, [
    { category: 'comment', column: 0, line: 1, value: '/* hello */' }
]);

deq(fn('/**/').tokens, [
    { category: 'comment', column: 0, line: 1, value: '/**/' }
]);

deq(fn('/* line1\nline2 */').tokens, [
    { category: 'comment', column: 0, line: 1, value: '/* line1\nline2 */' }
]);

eq(fn('/* unclosed').errors.length > 0, true);
eq(fn('/* unclosed').errors[0].message.includes('Unterminated block comment'), true);

// Colons and Commas.
deq(fn(':').tokens, [
    { category: 'colon', column: 0, line: 1, value: ':' }
]);

deq(fn(',').tokens, [
    { category: 'comma', column: 0, line: 1, value: ',' }
]);

// Complex Inputs.
deq(fn('{"key": "value"}').tokens, [
    { category: 'brace', column: 0, line: 1, value: '{' },
    { category: 'string', column: 1, line: 1, value: '"key"' },
    { category: 'colon', column: 6, line: 1, value: ':' },
    { category: 'whitespace', column: 7, line: 1, value: ' ' },
    { category: 'string', column: 8, line: 1, value: '"value"' },
    { category: 'brace', column: 15, line: 1, value: '}' }
]);

deq(fn('[1, 2, 3]').tokens, [
    { category: 'bracket', column: 0, line: 1, value: '[' },
    { category: 'number', column: 1, line: 1, value: '1' },
    { category: 'comma', column: 2, line: 1, value: ',' },
    { category: 'whitespace', column: 3, line: 1, value: ' ' },
    { category: 'number', column: 4, line: 1, value: '2' },
    { category: 'comma', column: 5, line: 1, value: ',' },
    { category: 'whitespace', column: 6, line: 1, value: ' ' },
    { category: 'number', column: 7, line: 1, value: '3' },
    { category: 'bracket', column: 8, line: 1, value: ']' }
]);

eq(fn('{"a": true, "b": false, "c": null}').tokens.length, 18);
eq(fn('{"a": true, "b": false, "c": null}').errors.length, 0);

// README Example.
{
    const input = `{
    /* JSONC allows comments and trailing commas! */
    "key1": 1, // note the trailing comma after the value "TWO":
    "key2": "TWO",
}`;
    const result = fn(input);
    
    const expected = [
        { category: "brace", column: 0, line: 1, value: "{" },
        { category: "whitespace", column: 1, line: 1, value: "\n" },
        { category: "whitespace", column: 0, line: 2, value: "    " },
        { category: "comment", column: 4, line: 2, value: "/* JSONC allows comments and trailing commas! */" },
        { category: "whitespace", column: 52, line: 2, value: "\n" },
        { category: "whitespace", column: 0, line: 3, value: "    " },
        { category: "string", column: 4, line: 3, value: '"key1"' },
        { category: "colon", column: 10, line: 3, value: ":" },
        { category: "whitespace", column: 11, line: 3, value: " " },
        { category: "number", column: 12, line: 3, value: "1" },
        { category: "comma", column: 13, line: 3, value: "," },
        { category: "whitespace", column: 14, line: 3, value: " " },
        { category: "comment", column: 15, line: 3, value: '// note the trailing comma after the value "TWO":' },
        { category: "whitespace", column: 64, line: 3, value: "\n" },
        { category: "whitespace", column: 0, line: 4, value: "    " },
        { category: "string", column: 4, line: 4, value: '"key2"' },
        { category: "colon", column: 10, line: 4, value: ":" },
        { category: "whitespace", column: 11, line: 4, value: " " },
        { category: "string", column: 12, line: 4, value: '"TWO"' },
        { category: "comma", column: 17, line: 4, value: "," },
        { category: "whitespace", column: 18, line: 4, value: "\n" },
        { category: "brace", column: 0, line: 5, value: "}" }
    ];
    
    eq(result.tokens.length, expected.length);
    eq(result.errors.length, 0);
    deq(result.tokens, expected);
}

// Multi-line.
{
    const result = fn('{\n  "a": 1\n}');
    eq(result.tokens[0].line, 1);
    eq(result.tokens[3].line, 2); // "a"
    eq(result.tokens[result.tokens.length - 1].line, 3); // }
}

// Edge Cases.
eq(fn('/').errors.length > 0, true);
eq(fn('/').errors[0].message.includes("Unexpected character '/'"), true);

eq(fn('@').errors.length > 0, true);

eq(fn('[1,]').errors.length, 0);
eq(fn('[1,]').tokens.some(t => t.category === 'comma'), true);

eq(fn('{"nested": {"deep": true}}').errors.length, 0);
eq(fn('{"nested": {"deep": true}}').tokens.filter(t => t.category === 'brace').length, 4);
