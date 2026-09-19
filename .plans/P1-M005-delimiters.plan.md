# Plan: P1-M005 — Delimiters

Status: Approved
Milestone: P1-M005
Created: 2026-09-18

## Goal

Implement the six Rust delimiter token spellings in the Ferraxis lexer while preserving a strict
phase boundary between flat lexical tokenization and delimiter grouping.

The lexer will emit exact-span delimiter tokens for parentheses, square brackets, and braces.
Pairing, balance validation, token-tree construction, and unmatched/mismatched delimiter
diagnostics are intentionally deferred to a later grouping or parsing layer under ADR-0013.

## Non-goals

- No delimiter pairing or stack validation in the lexer.
- No token-tree group construction.
- No parser AST construction.
- No macro matcher or macro token-tree semantics.
- No unmatched- or mismatched-delimiter diagnostics beyond flat token production.
- No invisible delimiter concept.
- No punctuation redesign.
- No literal, lifetime, raw-identifier, keyword, or Unicode-identifier work.
- No dependency additions.
- No change to the stable rustc differential observation contract from ADR-0012.

## Context

The Rust Reference identifies delimiters as one of Rust's token kinds and lists three bracket
families:

- `(` and `)` — parentheses;
- `[` and `]` — square brackets;
- `{` and `}` — braces.

The Reference also states that open brackets must pair with close brackets and that bracketed tokens
form token trees in macro contexts.

Ferraxis separates two responsibilities:

1. the lexer recognizes each delimiter spelling and preserves its exact source span;
2. a later grouping/parser boundary validates matching and constructs grouped structure.

This separation is recorded permanently in ADR-0013.

P1-M004 intentionally excluded the six delimiters from `Punctuation` so P1-M005 can give them
stable delimiter-specific identity without mixing grouping concerns into punctuation recognition.

Primary references:

- <https://doc.rust-lang.org/reference/tokens.html#delimiters>
- <https://doc.rust-lang.org/reference/notation.html>
- <https://doc.rust-lang.org/reference/macros-by-example.html>

## Architecture placement

P1-M005 remains inside the lexer and produces a flat token stream.

```text
Rust source bytes
      |
      v
ferraxis-lexer
  comments / whitespace
  identifiers / keyword
  delimiters  <--- P1-M005: flat tokens only
  punctuation
      |
      v
flat token stream
      |
      +--> future delimiter grouping / parser validation
```

The lexer must not construct groups or reject flat delimiter sequences merely because they are
unbalanced or mismatched.

## Data flow

1. Preserve existing whitespace and comment recognition.
2. Preserve identifier, reserved-prefix, and punctuation behavior.
3. At the current cursor, recognize one of the six single-byte delimiter spellings.
4. Emit `TokenKind::Delimiter(Delimiter)` with the exact one-byte half-open source span.
5. Advance one byte.
6. Continue normal lexing.
7. Perform no delimiter-stack mutation and no matching check.
8. Emit exactly one zero-width EOF token at original source byte length.

## Invariants

- INV-005 — source positions use byte offsets.
- INV-006 — future source diagnostics carry meaningful spans.
- INV-007 — syntax structure is preserved for later frontend phases.
- INV-013 — language disagreements require conformance evidence.
- INV-014 — accepted variances and phase-boundary differences are documented.
- INV-015 — compatibility claims require reproducible evidence.
- INV-016 — user-controlled source must not cause a compiler panic.
- INV-017 — invalid Rust must eventually fail in a controlled way rather than ICE.
- INV-018 — new lexer behavior has regression coverage.
- INV-019 — tokenization remains deterministic.
- INV-021 — behavior documentation remains CI-validated.
- INV-022 — permanent IDs are never recycled.

## ADRs

- ADR-0001 — semantic authority hierarchy.
- ADR-0003 — byte-offset source spans.
- ADR-0006 — differential testing against rustc.
- ADR-0010 — plan-first agent workflow.
- ADR-0011 — toolchain and reproducible validation.
- ADR-0012 — stable rustc token-tree observation boundary.
- ADR-0013 — flat delimiter lexing; grouping after lexing.

ADR-0013 is introduced in the plan-only checkpoint because delimiter grouping ownership is a durable
compiler architecture decision rather than an implementation detail.

## Semantic evidence

Create and maintain:

- `SEM-LEX-0008` — delimiter tokenization.

Primary authority is L1, the Rust Reference token definition.

The differential harness remains L4 evidence only. Because the rustc probe requires a valid
`macro_rules!` token-tree sequence, it may reject unmatched or mismatched delimiters that the
Ferraxis flat lexer correctly tokenizes. Those cases are expected `rustc_rejects` observations,
not lexer correctness failures.

## Public API / CLI

Add a public delimiter identity:

```rust
pub enum Delimiter {
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}
```

Add:

```rust
TokenKind::Delimiter(Delimiter)
```

`Delimiter::as_str()` returns the exact one-byte source spelling.

No matching-pair API is required in P1-M005. Pair/group semantics remain outside the lexer.

The token-dump CLI must render delimiter tokens deterministically with exact source spelling and
byte spans.

## Compatibility analysis

The six delimiter spellings are single-byte ASCII characters and do not require maximal-munch
logic.

Important phase-boundary behavior:

- `(` alone is a valid delimiter token for the lexer even though the complete Rust source is not a
  valid paired-delimiter token-tree sequence.
- `)` alone is likewise tokenized flatly.
- `(]` produces two delimiter tokens; mismatch validation belongs after lexing.
- balanced `()`, `[]`, and `{}` produce flat open/close token pairs.
- nested `({[]})` produces six flat delimiter tokens in source order.
- punctuation adjacent to delimiters retains P1-M004 token identity.
- comments inside delimiters remain skipped as lexical whitespace.
- documentation-comment handling is unchanged.

The Rust Reference requires brackets to pair for valid Rust input, but P1-M005 deliberately
separates recognition from validation. Final Ferraxis compilation must reject unmatched or
mismatched delimiters in a later frontend stage.

## Differential-observation adjustment

The stable rustc observation used by `ferraxis-diff` is a macro token-tree probe, not a raw lexer
oracle.

Therefore P1-M005 intentionally changes the interpretation of delimiter evidence:

- existing `paired-delimiters`: `ferraxis_rejects` -> `agree_accept`;
- existing `unmatched-open-delimiter`: `agree_reject` -> `rustc_rejects`;
- new unmatched-close case: `rustc_rejects`;
- new mismatched-delimiter case: `rustc_rejects`;
- balanced delimiter cases: `agree_accept`.

A `rustc_rejects` classification here records the narrower rustc token-tree boundary from ADR-0012
and does not mean Ferraxis should add balancing to the lexer.

## Dependency analysis

No new dependency is required.

Implementation uses existing token, source, and span types plus standard-library matching.

## Expected file boundary

Plan checkpoint:

- `.plans/ACTIVE`
- `.plans/P1-M005-delimiters.plan.md`
- `docs/adr/ADR-0013-flat-delimiter-lexing.md`
- `docs/adr/README.md`
- `docs/TESTING.md`
- `docs/MILESTONES.md`
- `docs/semantics/README.md`
- `docs/semantics/SEM-LEX-0008-delimiters.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Implementation:

- `crates/ferraxis-lexer/src/lib.rs`
- `crates/ferraxis/tests/token_dump.rs`
- `tests/differential/lexer/cases/`
- `tests/differential/lexer/expectations.tsv`
- `docs/semantics/SEM-LEX-0008-delimiters.md`
- `docs/semantics/README.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Closure may additionally update:

- this plan;
- `docs/MILESTONES.md`;
- `.plans/ACTIVE`.

No Cargo manifest or dependency changes are expected.

## Test-first matrix

| Behavior | Input | Expected lexer result |
| --- | --- | --- |
| Open parenthesis | `(` | OpenParenthesis, EOF |
| Close parenthesis | `)` | CloseParenthesis, EOF |
| Open bracket | `[` | OpenBracket, EOF |
| Close bracket | `]` | CloseBracket, EOF |
| Open brace | `{` | OpenBrace, EOF |
| Close brace | `}` | CloseBrace, EOF |
| All pairs | `()[]{}` | six delimiter tokens, EOF |
| Nested balanced | `({[]})` | six delimiter tokens in source order |
| Unmatched open | `(` | lexer accepts flat token |
| Unmatched close | `)` | lexer accepts flat token |
| Mismatched pair | `(]` | lexer accepts two flat tokens |
| Delimiter + punctuation | `(->)` | open, ThinArrow, close |
| Comment between delimiters | `(/*x*/)` | open, close |
| Token spans | `fn(main)` | exact original byte offsets |
| CLI rendering | `fn(main)` | deterministic delimiter spelling and spans |
| Differential paired | existing paired case | `agree_accept` |
| Differential balanced all | new balanced case | `agree_accept` |
| Differential nested | new nested case | `agree_accept` |
| Differential punct mix | new mix case | `agree_accept` |
| Differential unmatched open | existing unmatched case | `rustc_rejects` |
| Differential unmatched close | new case | `rustc_rejects` |
| Differential mismatched | new case | `rustc_rejects` |

## Implementation sequence

1. Commit the Approved plan, ADR-0013, Planned semantic record, milestone activation, testing note,
   and state/handoff records without Rust or Cargo changes.
2. Inspect the exact plan commit file list before moving the branch ref.
3. Require the exact plan-only commit to pass all six CI jobs.
4. Inspect any failure and classify it before making a repair.
5. Add delimiter unit tests and CLI regression coverage.
6. Add the `Delimiter` public enum and `TokenKind::Delimiter(Delimiter)`.
7. Add exact single-byte delimiter recognition without grouping state.
8. Preserve comments, punctuation, identifiers, reserved-prefix guards, and EOF behavior.
9. Update differential expectations and add balanced/unbalanced delimiter cases.
10. Inspect the exact implementation diff before moving the branch ref.
11. Require the exact implementation head to pass all six CI jobs.
12. Inspect the uploaded differential artifact.
13. Record exact implementation CI, artifact identity, and expected `rustc_rejects` delimiter
    observations.
14. Close P1-M005 and remove `.plans/ACTIVE` only after evidence is clean.
15. Inspect the exact closure diff.
16. Require exact closed-state CI success.
17. Merge with full history preserved.
18. Require exact post-merge `main` CI success.

## Failure modes

- Adding balance validation to the lexer.
- Conflating delimiter tokens with grouped token trees.
- Treating `rustc_rejects` for unmatched delimiters as a Ferraxis lexer failure.
- Leaving the old `unmatched-open-delimiter` expectation as `agree_reject` after delimiter support.
- Emitting delimiters as generic `Punctuation` despite the P1-M004/P1-M005 boundary.
- Shifting byte spans around delimiter tokens.
- Breaking punctuation adjacent to delimiters.
- Letting comments become delimiter/punctuation tokens.
- Accidentally accepting unsupported documentation comments.
- Adding parser or macro semantics to the lexer.
- Repairing a CI failure without first identifying its category.
- Force-resetting a branch instead of repairing forward.

## Documentation impact

- Add ADR-0013 and bring the ADR registry current through ADR-0013.
- Add `SEM-LEX-0008` as `Planned` before implementation.
- Update the differential-testing documentation to explain expected unmatched-delimiter
  `rustc_rejects` observations after P1-M005.
- Mark SEM-LEX-0008 `Active` only after exact implementation validation.
- Keep pairing/grouping ownership explicit in state and handoff records.

## Quality gates

- `./scripts/gate.sh precommit`
- `./scripts/gate.sh fast`
- `./scripts/gate.sh full`
- `cargo run -p ferraxis-diff --locked -- lexer`
- exact plan-head GitHub Actions success
- exact implementation-head GitHub Actions success
- differential artifact inspection
- exact closed-state GitHub Actions success
- successful post-merge `main` CI

## Acceptance criteria

- [ ] Six explicit delimiter token identities exist.
- [ ] Every delimiter has exact one-byte source spelling.
- [ ] `TokenKind::Delimiter(Delimiter)` is public and documented.
- [ ] `Delimiter::as_str()` returns exact spellings.
- [ ] All balanced delimiter families tokenize correctly.
- [ ] Nested balanced delimiters remain a flat token sequence.
- [ ] Unmatched open delimiters tokenize without lexer balance validation.
- [ ] Unmatched close delimiters tokenize without lexer balance validation.
- [ ] Mismatched delimiters tokenize without lexer balance validation.
- [ ] Pairing/group construction is absent from the lexer.
- [ ] Punctuation adjacency remains correct.
- [ ] Comment precedence remains correct.
- [ ] Delimiter spans preserve original byte offsets.
- [ ] EOF remains exactly one zero-width token at source length.
- [ ] CLI token dump renders delimiters deterministically.
- [ ] Existing lexer behavior remains green.
- [ ] `paired-delimiters` differential evidence becomes `agree_accept`.
- [ ] `unmatched-open-delimiter` becomes expected `rustc_rejects` evidence.
- [ ] Added unmatched-close and mismatched cases classify `rustc_rejects`.
- [ ] Added balanced delimiter cases classify `agree_accept`.
- [ ] ADR-0013 is Accepted and registry-current.
- [ ] SEM-LEX-0008 is current.
- [ ] No dependency or Cargo change is introduced.
- [ ] Exact implementation head passes all six CI jobs.
- [ ] Differential artifact is inspected before closure.
- [ ] Exact closed-state head passes all six CI jobs.
- [ ] `PROJECT_STATE.md` is current.
- [ ] `AGENT_HANDOFF.md` is current.

## Completion record

Implementation commit:
CI run:
CI result:
Completed:
Notes:
