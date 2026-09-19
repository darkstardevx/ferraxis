# SEM-LEX-0004: Non-doc line comments

## Topic

Recognition and treatment of Rust non-documentation line comments.

## Status

Active

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference comments grammar: <https://doc.rust-lang.org/reference/comments.html>
- Rust Reference input format:
  <https://doc.rust-lang.org/reference/input-format.html>

## Ferraxis behavior

Ferraxis implements the Rust Reference `LINE_COMMENT` category as lexical whitespace.

An ordinary non-doc line comment begins with `//` when that prefix is not the start of an inner or
outer line documentation comment. Its body continues until LF or EOF and produces no token.

The implemented boundary distinguishes:

- `//` and `// text` as ordinary line comments;
- `////...` as an ordinary line comment;
- `///...` as an unsupported outer line documentation comment;
- `//!...` as an unsupported inner line documentation comment.

UTF-8 bytes in the ordinary comment body are skipped without decoding requirements, and a bare CR
does not end a line comment. Ferraxis source spans continue to use original UTF-8 byte offsets.

Ferraxis does not yet claim a complete Rust input-normalization pipeline. In particular, P1-M001
does not add a general CRLF-normalization stage.

## Tests

- unit tests in `crates/ferraxis-lexer/src/lib.rs`;
- versioned differential cases under `tests/differential/lexer/`.

EOF-terminated comments are covered directly by unit tests because tracked text fixtures must end
with exactly one LF under repository policy.

## Variance

Temporary unsupported subset: line documentation comments remain unsupported after P1-M001.

This is not an intentional Rust language extension. Full documentation-comment handling must be
specified before Ferraxis claims complete Rust comment lexing.
