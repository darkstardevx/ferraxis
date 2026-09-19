# Plan: P1-M001 — Non-doc line comments

Status: Approved
Milestone: P1-M001
Created: 2026-09-18

## Goal

Implement Rust non-documentation line comments in the Ferraxis lexer according to the Rust
Reference `LINE_COMMENT` grammar.

Ordinary line comments are treated as whitespace and produce no token. The implementation must
preserve byte-accurate spans for tokens surrounding comments and must not erase documentation
comments that have distinct Rust semantics.

## Non-goals

- No block comments.
- No nested block comments.
- No line documentation comments.
- No `///` outer documentation-comment lowering.
- No `//!` inner documentation-comment lowering.
- No attribute parsing or `#[doc]` synthesis.
- No punctuation-token implementation beyond recognizing the `//` comment introducer.
- No general CRLF-normalization layer.
- No parser, macro-expansion, or semantic-analysis changes.
- No change to the stable-rustc differential observation boundary from ADR-0012.

## Context

The Rust Reference defines comments as a lexical category and distinguishes ordinary
`LINE_COMMENT` from `INNER_LINE_DOC` and `OUTER_LINE_DOC`.

Non-doc comments are interpreted as whitespace. Therefore P1-M001 should skip ordinary line
comments rather than emit a comment token.

The distinction is semantically important:

- `// comment` is a non-doc line comment;
- `//` at EOF is a non-doc line comment;
- `//` immediately followed by LF is a non-doc line comment;
- `//// comment` is a non-doc line comment;
- `/// comment` is an outer line documentation comment;
- `//! comment` is an inner line documentation comment.

P1-M001 deliberately implements only the non-doc grammar. Documentation comments remain
unsupported and must continue to fail in a controlled way instead of being silently discarded.

Primary references:

- <https://doc.rust-lang.org/reference/comments.html>
- <https://doc.rust-lang.org/reference/input-format.html>

## Architecture placement

This is a lexer-only semantic expansion inside `ferraxis-lexer`.

The implementation remains before parsing and does not add any new compiler phase, backend
dependency, or rustc-private integration.

```text
SourceFile UTF-8 bytes
        |
        v
ferraxis-lexer
  whitespace
  non-doc line comments  <--- P1-M001
  identifiers / fn
  unsupported source
        |
        v
Token stream + EOF
```

## Data flow

1. Read the current source byte at the lexer cursor.
2. Preserve existing ASCII-whitespace handling.
3. Detect whether the cursor begins an ordinary non-doc line comment.
4. Refuse to classify `///...` or `//!...` as ordinary comments.
5. For a non-doc line comment, advance until LF or EOF.
6. Leave LF to the ordinary whitespace path or consume it equivalently without changing token
   output.
7. Resume lexing the next supported token.
8. Preserve original source byte offsets for all emitted token spans.
9. Emit exactly one EOF token at the original source byte length.

## Invariants

- INV-005 — source positions remain byte offsets.
- INV-013 — behavior disagreements require conformance evidence.
- INV-014 — unsupported documentation-comment behavior is explicit.
- INV-015 — compatibility claims require reproducible evidence.
- INV-016 — source input must not cause a compiler panic.
- INV-017 — unsupported input produces controlled failure.
- INV-018 — new lexer behavior requires regression coverage.
- INV-019 — lexing remains deterministic.
- INV-021 — behavior documentation remains CI-validated.
- INV-022 — permanent milestone and semantic-evidence IDs are preserved.

## ADRs

- ADR-0001 — semantic authority hierarchy.
- ADR-0003 — byte-offset source spans.
- ADR-0006 — differential testing against rustc.
- ADR-0011 — toolchain and reproducible validation.
- ADR-0012 — stable rustc token-tree observation boundary.

No new architecture ADR is required because P1-M001 changes language behavior inside the existing
lexer boundary without changing phase ownership or long-lived architecture.

## Semantic evidence

Create and maintain:

- `SEM-LEX-0004` — non-doc line comments.

Primary authority is L1, the Rust Reference comment grammar.

The existing P0-M018 differential harness supplies additional L4 evidence. The current
`line-comment` corpus case is expected to change from `ferraxis_rejects` to `agree_accept`
after implementation.

## Public API / CLI

No public API shape changes are required.

`TokenKind` does not gain an ordinary-comment token because Rust non-doc comments are whitespace
for tokenization.

The existing token-dump CLI should simply stop reporting an error for supported ordinary line
comments.

## Compatibility analysis

The implementation targets the Rust Reference `LINE_COMMENT` grammar, not every source sequence
beginning with `//`.

Important compatibility boundaries:

- LF terminates an ordinary line comment.
- EOF terminates an ordinary line comment.
- Bare CR is not a line terminator for this grammar.
- CRLF input normalization is a prior input-format transformation in Rust; Ferraxis does not yet
  claim a general normalization layer.
- UTF-8 text inside the comment body is ignored as comment content.
- `///...` is not an ordinary line comment.
- `//!...` is not an ordinary line comment.
- four or more leading slashes beginning with `////` are ordinary non-doc line comments.

Documentation comments have attribute semantics and therefore must not be skipped by P1-M001.

## Dependency analysis

No new dependency is required.

The implementation should use the existing source bytes and standard-library operations only.

## Expected file boundary

Plan checkpoint:

- `.plans/ACTIVE`
- `.plans/P1-M001-line-comments.plan.md`
- `docs/MILESTONES.md`
- `docs/semantics/README.md`
- `docs/semantics/SEM-LEX-0004-line-comments.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Implementation:

- `crates/ferraxis-lexer/src/lib.rs`
- `crates/ferraxis/tests/token_dump.rs` if CLI regression coverage needs extension
- `tests/differential/lexer/cases/`
- `tests/differential/lexer/expectations.tsv`
- `docs/semantics/SEM-LEX-0004-line-comments.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Closure may additionally update:

- this plan;
- `docs/MILESTONES.md`;
- `.plans/ACTIVE`.

No Cargo dependency changes are expected.

## Test-first matrix

| Behavior | Test | Expected result |
| --- | --- | --- |
| Empty line comment | `//\nfn` | comment skipped, then `Fn`, EOF |
| EOF line comment | `// comment` | only EOF |
| Inline line comment | `fn// comment\nmain` | `Fn`, `Identifier`, EOF |
| Comment-only line | `// comment\n` | only EOF |
| Four-slash comment | `//// comment\nfn` | comment skipped, then `Fn`, EOF |
| Unicode comment body | `// café\nfn` | comment skipped, then `Fn`, EOF |
| CR before LF | `// comment\r\nfn` | comment skipped through CR, then `Fn`, EOF |
| Bare CR in body | `// a\rb\nfn` | CR remains comment content; then `Fn`, EOF |
| Outer doc comment | `/// docs\nfn` | controlled unsupported-byte failure at first `/` |
| Inner doc comment | `//! docs\nfn` | controlled unsupported-byte failure at first `/` |
| Token span after comment | `// x\nfn` | `Fn` span begins at original byte offset 5 |
| Differential line comment | existing corpus case | `agree_accept` |
| Differential newline comment | existing `line-comment` case | `agree_accept` |
| Differential four-slash comment | new corpus case | `agree_accept` |

## Implementation sequence

1. Commit this Approved plan, semantic-evidence record, milestone activation, and state/handoff
   records without Rust or Cargo changes.
2. Require the plan-only checkpoint to pass the complete repository CI matrix.
3. Add lexer regression tests before or alongside the smallest implementation change.
4. Implement an explicit helper that distinguishes ordinary line comments from line doc comments.
5. Skip ordinary comment bytes until LF or EOF.
6. Preserve current controlled failure for `///` and `//!`.
7. Update differential corpus expectations and add representable boundary cases.
8. Run `./scripts/gate.sh fast`.
9. Run the differential harness and inspect generated evidence.
10. Run `./scripts/gate.sh full`.
11. Push the exact implementation head and require complete CI success.
12. Inspect the uploaded differential artifact.
13. Close P1-M001 only after exact implementation CI evidence is recorded.

## Corpus representation constraint

Tracked text files must end with exactly one LF under `scripts/check-text-files`. Therefore a
literal `.rsfrag` fixture cannot represent an EOF-terminated line comment without violating
repository policy.

P1-M001 covers EOF termination directly in lexer unit tests. Differential corpus coverage remains
for representable newline-terminated and four-slash comment cases. A future generated-case
mechanism may add exact EOF differential evidence without weakening the tracked-text invariant.

## Failure modes


- Treating all `//` prefixes as ordinary comments and silently discarding doc comments.
- Incorrectly treating exactly three slashes as an ordinary comment.
- Incorrectly rejecting `////` even though it is a normal line comment.
- Stopping a comment at CR instead of LF.
- Consuming source after LF into the comment.
- Emitting an ordinary-comment token even though non-doc comments are whitespace.
- Shifting subsequent token spans away from original byte offsets.
- Breaking explicit EOF placement.
- Introducing UTF-8 decoding assumptions while scanning comment bytes.
- Changing the rustc differential observation boundary.
- Calling a differential mismatch a correctness verdict without semantic-authority review.

## Documentation impact

- Add `SEM-LEX-0004` before implementation with `Status: Planned`.
- Mark it `Active` only when the behavior is implemented and validated.
- Keep documentation-comment behavior explicitly unsupported.
- Update project state and handoff at implementation and closure boundaries.

## Quality gates

- `./scripts/gate.sh precommit`
- `./scripts/gate.sh fast`
- `./scripts/gate.sh full`
- `cargo run -p ferraxis-diff --locked -- lexer`
- exact GitHub Actions success for the implementation commit
- inspection of the uploaded differential evidence artifact

## Acceptance criteria

- [ ] Ordinary Rust non-doc line comments are interpreted as whitespace.
- [ ] Empty `//` comments are supported.
- [ ] EOF-terminated `//` comments are supported.
- [ ] LF terminates a line comment.
- [ ] Bare CR does not terminate a line comment.
- [ ] Unicode UTF-8 bytes inside an ordinary line comment are ignored safely.
- [ ] `////...` is handled as an ordinary non-doc line comment.
- [ ] `///...` remains explicitly unsupported by this milestone.
- [ ] `//!...` remains explicitly unsupported by this milestone.
- [ ] Token spans after comments preserve original byte offsets.
- [ ] EOF remains exactly one zero-width token at original source length.
- [ ] Existing lexer behavior remains green.
- [ ] Differential line-comment evidence changes to `agree_accept`.
- [ ] Added representable differential boundary cases match committed expectations.
- [ ] No dependency is added.
- [ ] Relevant semantic evidence is current.
- [ ] Full local gate passes.
- [ ] Exact implementation commit passes CI.
- [ ] Differential artifact is inspected before closure.
- [ ] `PROJECT_STATE.md` is current.
- [ ] `AGENT_HANDOFF.md` is current.

## Completion record

Implementation commit:
CI run:
CI result:
Completed:
Notes:
