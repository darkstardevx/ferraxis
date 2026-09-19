# SEM-LEX-0007: Punctuation tokenization

## Topic

Recognition, identity, and longest-match behavior for Rust punctuation tokens implemented by
P1-M004.

## Status

Planned

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference tokens and punctuation:
  <https://doc.rust-lang.org/reference/tokens.html>
- Rust Reference grammar notation:
  <https://doc.rust-lang.org/reference/notation.html>
- Rust Reference input format:
  <https://doc.rust-lang.org/reference/input-format.html>
- Rust Reference identifiers:
  <https://doc.rust-lang.org/reference/identifiers.html>

## Ferraxis behavior

P1-M004 will add explicit token identity for every current Reference punctuation spelling except the
six delimiters owned by P1-M005.

Implemented spellings will be:

```text
... ..= <<= >>=
!= %= && &= *= += -= -> .. /= :: <- << <= == => >= >> ^= |= ||
! # $ % & * + , - . / : ; < = > ? @ ^ | ~
```

Recognition is longest-first.

Comments retain precedence over slash/star punctuation.

Unsupported top-level documentation comments must remain controlled failures rather than becoming
punctuation streams.

Rust-2024 reserved multi-pound forms such as `##` must remain rejected. Identifier-adjacent pound
forms must not be silently split. Valid raw identifiers remain unsupported until P1-M011.

## Token representation

P1-M004 will add a public `Punctuation` enum and `TokenKind::Punctuation(Punctuation)`.

The punctuation enum preserves exact lexical identity for downstream parser phases.

## Temporary boundaries

- `( ) [ ] { }` — P1-M005 delimiters.
- raw identifiers — P1-M011.
- lifetimes / single quote — P1-M012.
- bare underscore and remaining keywords — keyword milestones.
- literals and their prefix interactions — literal milestones.

## Proc-macro note

Rust's procedural-macro token API exposes operator punctuation as individual characters and
performs conversion at the proc-macro boundary. SEM-LEX-0007 describes the Reference lexer token
production and does not claim proc-macro representation equivalence.

## Tests

Planned coverage:

- lexer unit tests for all variants, spans, longest-match overlaps, comments, reserved-pound/prefix
  guards, and deferred token families;
- compiler-driver token-dump integration tests;
- versioned differential cases under `tests/differential/lexer/`.

## Variance

P1-M004 intentionally leaves delimiter tokens and several composite token families unsupported.
Those are milestone-scoped missing subsets, not Ferraxis language extensions.
