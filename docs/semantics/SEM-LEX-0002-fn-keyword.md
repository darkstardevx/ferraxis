# SEM-LEX-0002: Exact `fn` keyword boundary

## Topic

Recognition of the strict keyword `fn` without consuming identifier prefixes.

## Status

Active

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference keywords: <https://doc.rust-lang.org/reference/keywords.html>

## Ferraxis behavior

The byte sequence `fn` is emitted as `TokenKind::Fn` only when the complete identifier lexeme equals
`fn`. Inputs such as `fnn` and `fn1` are emitted as identifiers in the Phase 0 subset.

## Tests

- `crates/ferraxis-lexer/src/lib.rs`

## Variance

None for the supported ASCII subset.
