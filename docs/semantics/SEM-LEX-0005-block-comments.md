# SEM-LEX-0005: Non-nested non-doc block comments

## Topic

Recognition and treatment of the P1-M002 non-nested subset of Rust ordinary block comments.

## Status

Active

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference comments grammar: <https://doc.rust-lang.org/reference/comments.html>
- Rust Reference input format:
  <https://doc.rust-lang.org/reference/input-format.html>

## Ferraxis behavior

Ferraxis treats supported depth-one ordinary `/* ... */` comments as lexical whitespace.

The implemented documentation-comment boundary is:

- `/**/` is an ordinary empty block comment;
- `/***/` is an ordinary block comment;
- `/*** text */` is an ordinary block comment;
- `/** text */` is outer block documentation syntax and remains unsupported;
- `/*! text */` and `/*!! text */` are inner block documentation syntax and remain unsupported.

P1-M002 established depth-one ordinary block comments. P1-M003 extends the same ordinary-comment
boundary with recursive nesting, so nested ordinary, outer-doc, and inner-doc block forms are now
tracked until the outer ordinary comment closes.

EOF before the closing `*/` is a controlled lexical failure at the original opening slash.

Supported ordinary block-comment bodies may span lines and contain UTF-8 bytes, line-comment
markers, and bare CR. Ferraxis source spans continue to use original UTF-8 byte offsets. P1-M002
does not add a general input-normalization layer.

## Tests

Coverage exists in:

- unit tests in `crates/ferraxis-lexer/src/lib.rs`;
- versioned differential cases under `tests/differential/lexer/`.

The differential suite includes supported comments, an unterminated comment expected to
`agree_reject`, and recursive nested-comment coverage maintained by SEM-LEX-0006.

## Variance

Temporary variance from Rust:

- top-level outer and inner block documentation comments remain unsupported.

These are explicit missing-language subsets, not Ferraxis language extensions.
