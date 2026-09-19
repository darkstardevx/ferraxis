# SEM-LEX-0008: Delimiter tokenization

## Topic

Flat lexical recognition of Rust parentheses, square brackets, and braces.

## Status

Active

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference tokens:
  <https://doc.rust-lang.org/reference/tokens.html#delimiters>
- Rust Reference macros by example:
  <https://doc.rust-lang.org/reference/macros-by-example.html>

## Ferraxis behavior

Ferraxis emits six explicit flat delimiter token identities:

- `(` — open parenthesis;
- `)` — close parenthesis;
- `[` — open square bracket;
- `]` — close square bracket;
- `{` — open brace;
- `}` — close brace.

Each token carries its exact one-byte source span.

Under ADR-0013, the lexer does not validate matching, nesting, or balance. Unmatched and mismatched
delimiter spellings are still lexical tokens. A later grouping/parser stage must reject invalid
group structure before complete Rust source is accepted.

The current stable rustc differential probe operates at a token-tree boundary. As a result,
unmatched or mismatched delimiter cases are expected to produce `rustc_rejects` classifications
after P1-M005.

## Tests

Coverage exists in:

- unit tests in `crates/ferraxis-lexer/src/lib.rs`;
- token-dump integration tests;
- balanced and unbalanced differential cases under `tests/differential/lexer/`.

## Variance

There is no delimiter-spelling variance for the six token forms.

Ferraxis intentionally assigns delimiter balance validation to a later frontend phase rather than
the lexer. This is an internal phase-boundary decision recorded by ADR-0013, not an extension that
makes unmatched delimiters valid Rust programs.
