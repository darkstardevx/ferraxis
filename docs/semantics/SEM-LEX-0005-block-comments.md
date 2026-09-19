# SEM-LEX-0005: Non-nested non-doc block comments

## Topic

Recognition and treatment of the P1-M002 non-nested subset of Rust ordinary block comments.

## Status

Planned

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference comments grammar: <https://doc.rust-lang.org/reference/comments.html>
- Rust Reference input format:
  <https://doc.rust-lang.org/reference/input-format.html>

## Ferraxis behavior

P1-M002 will treat supported depth-one ordinary `/* ... */` comments as lexical whitespace.

The milestone preserves the Rust documentation-comment boundary:

- `/**/` is an ordinary empty block comment;
- `/***/` is an ordinary block comment;
- `/*** text */` is an ordinary block comment;
- `/** text */` is outer block documentation syntax and remains unsupported;
- `/*! text */` and `/*!! text */` are inner block documentation syntax and remain unsupported.

Rust's full `BLOCK_COMMENT` grammar is recursive. P1-M002 does not claim recursive support. If a
nested `/*` opener appears before the outer `*/`, Ferraxis will fail in a controlled way at that
nested opener. P1-M003 owns recursive nesting.

EOF before the closing `*/` is a controlled lexical failure.

Ferraxis source spans continue to use original UTF-8 byte offsets. P1-M002 does not add a general
input-normalization layer.

## Tests

Planned coverage:

- unit tests in `crates/ferraxis-lexer/src/lib.rs`;
- versioned differential cases under `tests/differential/lexer/`.

The differential suite must include both supported comments and the intentional nested-comment gap.

## Variance

Temporary variance from Rust:

- recursive nested block comments are unsupported until P1-M003;
- outer and inner block documentation comments remain unsupported.

These are explicit missing-language subsets, not Ferraxis language extensions.
