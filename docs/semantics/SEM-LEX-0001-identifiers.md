# SEM-LEX-0001: Initial identifier lexical grammar

## Topic

Initial ASCII identifier recognition during the Phase 0 lexer slice.

## Status

Active

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference identifier grammar: <https://doc.rust-lang.org/reference/identifiers.html>

## Ferraxis behavior

Phase 0 implements a deliberately smaller ASCII subset while preserving keyword boundaries. An
identifier may begin with an ASCII alphabetic character or `_`; continuation bytes may also contain
ASCII digits. A bare `_` is not accepted as an identifier by this Phase 0 lexer and remains reserved
for later token support.

Unicode XID behavior, raw identifiers, and the full Rust keyword set are explicitly unsupported in
this slice rather than approximated.

## Tests

- `crates/ferraxis-lexer/src/lib.rs`
- `crates/ferraxis/tests/token_dump.rs`

## Variance

Temporary implementation subset, not an intentional language extension. Full Unicode identifier
compatibility is tracked by Phase 1 milestones.
