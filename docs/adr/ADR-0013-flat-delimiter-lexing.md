# ADR-0013: Flat delimiter lexing; grouping after lexing

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Rust source contains three delimiter families: parentheses, square brackets, and braces.

The Rust Reference describes delimiters as tokens and also requires open brackets to pair with
close brackets. In macro contexts, paired delimiters and their contents form token trees.

Ferraxis needs a durable phase boundary between recognizing delimiter spellings and constructing
or validating grouped syntax.

The existing stable-rustc differential harness from ADR-0012 observes macro token-tree acceptance,
not a raw lexer token stream. As a result, rustc may reject an unmatched or mismatched delimiter
even when a flat lexer can recognize every source character as a token.

## Decision

The Ferraxis lexer emits delimiters as flat tokens with exact source spans.

P1-M005 introduces six explicit delimiter identities:

- open and close parenthesis;
- open and close square bracket;
- open and close brace.

The lexer does not:

- maintain a delimiter stack;
- require matching pairs;
- reject mismatched delimiter kinds;
- construct token-tree groups;
- assign group nesting.

Delimiter pairing, group construction, and unmatched/mismatched delimiter validation belong to a
later frontend layer after lexical token production.

A flat token stream containing unmatched or mismatched delimiter tokens is therefore a valid lexer
result even though it cannot become a valid complete Rust token-tree or parsed program.

## Consequences

The lexer remains deterministic, local, and source-preserving.

Parser or token-tree work can consume explicit delimiter identities without reverse-engineering
generic punctuation tokens.

Future grouping diagnostics can use the exact delimiter token spans produced by the lexer.

Differential cases may legitimately classify as `rustc_rejects` when Ferraxis accepts a flat
delimiter token sequence that the stable rustc macro token-tree probe rejects for grouping reasons.
Such a mismatch is expected evidence under ADR-0012, not a request to add balance validation to the
lexer.

This ADR does not decide the exact implementation crate or API for the future grouping layer. That
decision belongs to the milestone that introduces grouping or parser structure.

## References

- [Rust Reference: tokens](https://doc.rust-lang.org/reference/tokens.html)
- [Rust Reference: macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)
- `docs/adr/ADR-0003-byte-offset-source-spans.md`
- `docs/adr/ADR-0006-differential-testing-against-rustc.md`
- `docs/adr/ADR-0012-stable-rustc-token-tree-observation.md`
- `docs/TESTING.md`
