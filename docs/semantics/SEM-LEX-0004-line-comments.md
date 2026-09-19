# SEM-LEX-0004: Non-doc line comments

## Topic

Recognition and treatment of Rust non-documentation line comments.

## Status

Planned

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference comments grammar: <https://doc.rust-lang.org/reference/comments.html>
- Rust Reference input format:
  <https://doc.rust-lang.org/reference/input-format.html>

## Ferraxis behavior

P1-M001 will implement the Rust Reference `LINE_COMMENT` category as lexical whitespace.

An ordinary non-doc line comment begins with `//` when that prefix is not the start of an inner or
outer line documentation comment. Its body continues until LF or EOF and produces no token.

The milestone will explicitly distinguish:

- `//` and `// text` as ordinary line comments;
- `////...` as an ordinary line comment;
- `///...` as an unsupported outer line documentation comment;
- `//!...` as an unsupported inner line documentation comment.

Documentation comments are not whitespace-equivalent syntax and are therefore not silently
discarded by P1-M001.

Ferraxis source spans continue to use original UTF-8 byte offsets. This record does not claim that
Ferraxis has implemented Rust's complete input-normalization pipeline.

## Tests

Planned coverage:

- unit tests in `crates/ferraxis-lexer/src/lib.rs`;
- token-dump regression coverage when useful;
- versioned cases under `tests/differential/lexer/`.

## Variance

Temporary unsupported subset: line documentation comments remain unsupported after P1-M001.

This is not an intentional Rust language extension. Full documentation-comment handling must be
specified before Ferraxis claims complete Rust comment lexing.
