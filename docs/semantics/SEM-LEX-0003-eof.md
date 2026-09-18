# SEM-LEX-0003: EOF token

## Topic

Internal representation of end-of-file in the Ferraxis token stream.

## Status

Active

## Primary authority

L5 — documented Ferraxis implementation representation.

## Sources

- `docs/adr/ADR-0003-byte-offset-source-spans.md`

## Ferraxis behavior

The lexer emits exactly one explicit `Eof` token after all supported source bytes are consumed. Its
span is the zero-width half-open span at the source byte length.

The explicit EOF token is an internal Ferraxis representation choice and does not claim to be a
Rust language-level token requirement.

## Tests

- `crates/ferraxis-lexer/src/lib.rs`

## Variance

Internal representation only; no source-language variance.
