/* should not generate diagnostics */
s.toUpperCase() === 'ABC';
s.toLowerCase() === 'abc';
s.toUpperCase() === '20';
s.toLowerCase() === '20';
s.toLowerCase() === `eFg${12}`;
s.toLowerCase() == `eFg${12}`;
s.toLowerCase() === "\xaa";
s.toLowerCase() === "\xAA";
s.toUpperCase() === "\u001b";
s.toLowerCase() === "\u001B";
s.toUpperCase() === "\u000D";
s.toLowerCase() === "\u000D";
s.toLowerCase() === "\u{a}aa";
s.toLowerCase() === "\u{A}aa";
s.toUpperCase() === "{}";
s.toLowerCase() === "{}";