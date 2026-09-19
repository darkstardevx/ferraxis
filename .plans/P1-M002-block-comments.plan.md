# Plan: P1-M002 — Non-nested block comments

Status: Approved
Milestone: P1-M002
Created: 2026-09-18

## Goal

Implement the non-nested subset of Rust ordinary non-documentation block comments in the Ferraxis
lexer.

P1-M002 treats supported `/* ... */` comments as lexical whitespace, preserves original source byte
offsets, distinguishes block documentation comments from ordinary comments, rejects unterminated
comments in a controlled way, and rejects nested block-comment openers explicitly rather than
mis-tokenizing them.

Full recursive nesting remains P1-M003.

## Non-goals

- No nested block-comment support.
- No outer block documentation-comment lowering.
- No inner block documentation-comment lowering.
- No `/** ... */` outer-doc attribute synthesis.
- No `/*! ... */` inner-doc attribute synthesis.
- No parser, macro-expansion, HIR, type-system, or backend changes.
- No punctuation-token support beyond recognizing block-comment delimiters.
- No new dependency.
- No change to the stable rustc token-tree observation boundary from ADR-0012.
- No claim that P1-M002 implements the complete Rust `BLOCK_COMMENT` grammar; recursive nesting is
  an explicit temporary variance owned by P1-M003.

## Context

The Rust Reference defines ordinary block comments as lexical whitespace and explicitly supports
nested block comments.

P1-M002 intentionally implements the depth-one subset first because the permanent milestone
registry reserves P1-M003 for recursive nesting.

The doc-comment boundary is subtle and must be preserved:

- `/* comment */` is an ordinary non-doc block comment;
- `/**/` is an empty ordinary block comment;
- `/***/` is an ordinary block comment;
- `/*** text */` is an ordinary block comment;
- `/** text */` is an outer block documentation comment;
- `/*! text */` is an inner block documentation comment;
- `/*!! text */` is also an inner block documentation comment.

Any `/*` sequence encountered inside an already-open block comment is nesting under Rust semantics,
including nested ordinary or documentation block comments. P1-M002 rejects that opener cleanly and
leaves recursive handling to P1-M003.

Primary references:

- <https://doc.rust-lang.org/reference/comments.html>
- <https://doc.rust-lang.org/reference/input-format.html>

## Architecture placement

This is a lexer-only expansion inside `ferraxis-lexer`.

```text
SourceFile UTF-8 bytes
        |
        v
ferraxis-lexer
  ASCII whitespace
  non-doc line comments
  non-nested non-doc block comments  <--- P1-M002
  identifiers / fn
  unsupported source
        |
        v
Token stream + EOF
```

No backend, parser, rustc-private, LLVM, or GCC dependency is introduced.

## Data flow

1. Read the current source byte at the lexer cursor.
2. Preserve existing whitespace and line-comment behavior.
3. Detect `/*`.
4. Determine whether the opener is an ordinary block comment or block documentation comment.
5. Leave `/*!...*/` and true `/**...*/` doc comments unsupported.
6. For an ordinary P1-M002 block comment, advance after the opening `/*`.
7. Scan bytes until the first matching `*/` at depth one.
8. If another `/*` appears before the closing delimiter, return controlled failure at that nested
   opener because recursive nesting belongs to P1-M003.
9. If EOF arrives before `*/`, return controlled failure at the original opening slash.
10. Resume tokenization after the closing slash.
11. Preserve original byte offsets for all emitted tokens.
12. Emit exactly one zero-width EOF token at original source byte length.

## Invariants

- INV-005 — source positions remain byte offsets.
- INV-013 — behavior disagreements require conformance evidence.
- INV-014 — accepted variances are documented.
- INV-015 — compatibility claims require reproducible evidence.
- INV-016 — source input must not cause a compiler panic.
- INV-017 — invalid or unsupported Rust produces controlled failure.
- INV-018 — new lexer behavior requires regression coverage.
- INV-019 — lexing remains deterministic.
- INV-021 — behavior documentation remains CI-validated.
- INV-022 — milestone and semantic-evidence IDs remain permanent.

## ADRs

- ADR-0001 — semantic authority hierarchy.
- ADR-0003 — byte-offset source spans.
- ADR-0006 — differential testing against rustc.
- ADR-0011 — toolchain and reproducible validation.
- ADR-0012 — stable rustc token-tree observation boundary.

No new architecture ADR is required because P1-M002 stays inside the existing lexer phase and does
not change long-lived compiler architecture.

## Semantic evidence

Create and maintain:

- `SEM-LEX-0005` — non-nested non-doc block comments.

Primary authority is L1, the Rust Reference comment grammar.

The differential harness supplies additional L4 observation evidence. P1-M002 must include an
explicit nested-comment corpus case that remains `ferraxis_rejects` until P1-M003.

## Public API / CLI

No new token kind is introduced. Supported non-doc block comments are whitespace.

No public API shape change is required. Unterminated comments and nested comments in the P1-M002
subset use the existing structured `LexError` / `UnexpectedByte` mechanism, with stable source
spans that identify the relevant opener.

The token-dump CLI should accept supported ordinary block comments without emitting comment tokens.

## Compatibility analysis

The Rust Reference `BLOCK_COMMENT` grammar is recursive. P1-M002 intentionally implements only
depth-one ordinary non-doc comments and records recursive nesting as a temporary variance.

Doc-comment distinction:

- `/*!...*/` is inner block documentation syntax and remains unsupported.
- `/**...*/` is outer block documentation syntax only when the character immediately after `/**`
  is neither `*` nor `/`.
- `/**/` is therefore an ordinary empty block comment.
- `/***/` and `/*** text */` are ordinary block comments.

Other behavior:

- supported comments may span lines;
- UTF-8 bytes in ordinary comment bodies are skipped without decoding requirements;
- `//` inside a block comment is ordinary comment-body text;
- bare CR may occur in an ordinary block comment body;
- EOF before `*/` is a controlled lexical failure;
- a nested `/*` before closure is a controlled temporary P1-M002 rejection;
- tokens after a closed comment retain offsets from the original UTF-8 source;
- P1-M002 does not introduce a general CRLF-normalization phase.

## Dependency analysis

No dependency is required.

Implementation uses only existing source bytes, span types, diagnostics, and the standard library.

## Expected file boundary

Plan checkpoint:

- `.plans/ACTIVE`
- `.plans/P1-M002-block-comments.plan.md`
- `docs/MILESTONES.md`
- `docs/semantics/README.md`
- `docs/semantics/SEM-LEX-0005-block-comments.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Implementation:

- `crates/ferraxis-lexer/src/lib.rs`
- `tests/differential/lexer/cases/`
- `tests/differential/lexer/expectations.tsv`
- `docs/semantics/SEM-LEX-0005-block-comments.md`
- `docs/semantics/README.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Closure may additionally update:

- this plan;
- `docs/MILESTONES.md`;
- `.plans/ACTIVE`.

No Cargo manifest or dependency changes are expected.

## Test-first matrix

| Behavior | Test | Expected result |
| --- | --- | --- |
| Basic block comment | `/* comment */` | only EOF |
| Inline comment | `fn/* comment */main` | `Fn`, `Identifier`, EOF |
| Multiline comment | `/* a\nb */fn` | comment skipped, then `Fn` |
| Empty block comment | `/**/fn` | comment skipped, then `Fn` |
| Triple-star ordinary comment | `/***/fn` | comment skipped, then `Fn` |
| Longer-star ordinary comment | `/*** text */fn` | comment skipped, then `Fn` |
| Unicode body | `/* café */fn` | comment skipped, then `Fn` |
| Slash text in body | `/* // text */fn` | comment skipped, then `Fn` |
| Bare CR in body | `/* a\rb */fn` | comment skipped, then `Fn` |
| Outer block doc | `/** docs */fn` | controlled unsupported-byte failure at first `/` |
| Inner block doc | `/*! docs */fn` | controlled unsupported-byte failure at first `/` |
| Inner block doc with extra bang | `/*!! docs */fn` | controlled unsupported-byte failure at first `/` |
| Unterminated comment | `/* comment` | controlled failure at opening `/` |
| Nested ordinary comment | `/* a /* b */ c */` | controlled failure at nested `/` |
| Nested doc comment | `/* a /** b */ c */` | controlled failure at nested `/` |
| Span after comment | `/* x */fn` | `Fn` begins at original byte offset 7 |
| Differential basic block comment | corpus case | `agree_accept` |
| Differential empty block comment | corpus case | `agree_accept` |
| Differential triple-star comment | corpus case | `agree_accept` |
| Differential Unicode body | corpus case | `agree_accept` |
| Differential unterminated comment | corpus case | `agree_reject` |
| Differential nested comment | corpus case | `ferraxis_rejects` until P1-M003 |

## Implementation sequence

1. Commit this Approved plan, Planned semantic record, milestone activation, and state/handoff
   records without Rust or Cargo changes.
2. Require that exact plan-only checkpoint to pass the complete repository CI matrix.
3. Add regression tests covering the test-first matrix.
4. Add an explicit helper that distinguishes ordinary block comments from block doc comments.
5. Implement depth-one scanning to `*/`.
6. Reject nested `/*` at the nested opener rather than truncating the outer comment.
7. Reject EOF before closure at the original opening slash.
8. Preserve existing line-comment, identifier, keyword, span, and EOF behavior.
9. Add differential corpus cases and committed expectations.
10. Run the complete repository validation matrix.
11. Inspect uploaded differential evidence.
12. Record exact implementation CI and artifact identity.
13. Close P1-M002 only after all acceptance criteria are satisfied.
14. Remove `.plans/ACTIVE` only in the closure commit.
15. Merge only after exact closed-state CI succeeds.

## Failure modes

- Treating `/**/` as an outer doc comment even though it is ordinary.
- Treating `/***/` as an outer doc comment even though it is ordinary.
- Silently skipping `/** docs */` or `/*! docs */` and destroying doc semantics.
- Stopping at the inner `*/` of nested input and then tokenizing the outer remainder incorrectly.
- Accidentally implementing partial nesting without P1-M003 evidence.
- Running past EOF on an unterminated comment.
- Emitting a block-comment token instead of treating non-doc comments as whitespace.
- Shifting token spans after comments.
- Breaking line-comment behavior.
- Decoding arbitrary UTF-8 comment content unsafely.
- Calling differential acceptance a correctness verdict.

## Documentation impact

- Add `SEM-LEX-0005` as `Planned` before implementation.
- Mark it `Active` only after implementation and exact CI validation.
- Document recursive nesting as a temporary P1-M002 variance assigned to P1-M003.
- Keep block doc comments explicitly unsupported.
- Update project state and handoff at implementation and closure checkpoints.

## Quality gates

- `./scripts/gate.sh precommit`
- `./scripts/gate.sh fast`
- `./scripts/gate.sh full`
- `cargo run -p ferraxis-diff --locked -- lexer`
- exact GitHub Actions success for the implementation commit
- inspection of the uploaded differential evidence artifact
- exact GitHub Actions success for the closed-state commit
- successful post-merge `main` CI

## Acceptance criteria

- [ ] Depth-one ordinary non-doc block comments are interpreted as whitespace.
- [ ] Basic and multiline block comments are supported.
- [ ] Empty `/**/` is supported as an ordinary block comment.
- [ ] `/***/` is supported as an ordinary block comment.
- [ ] `/*** text */` is supported as an ordinary block comment.
- [ ] UTF-8 block-comment bodies are skipped safely.
- [ ] `//` inside a block comment is body text.
- [ ] Bare CR inside an ordinary block-comment body is handled safely.
- [ ] Outer block doc comments remain explicitly unsupported.
- [ ] Inner block doc comments remain explicitly unsupported.
- [ ] Unterminated block comments fail in a controlled way.
- [ ] Nested block-comment openers fail at the nested opener until P1-M003.
- [ ] Token spans after supported comments preserve original byte offsets.
- [ ] EOF remains exactly one zero-width token at original source length.
- [ ] Existing line-comment behavior remains green.
- [ ] Existing lexer behavior remains green.
- [ ] Differential supported-comment cases match `agree_accept`.
- [ ] Differential unterminated-comment case matches `agree_reject`.
- [ ] Differential nested-comment case remains `ferraxis_rejects` and documents P1-M003.
- [ ] No dependency is added.
- [ ] Relevant semantic evidence is current.
- [ ] Exact implementation commit passes the complete CI matrix.
- [ ] Differential artifact is inspected before closure.
- [ ] Exact closed-state commit passes CI.
- [ ] `PROJECT_STATE.md` is current.
- [ ] `AGENT_HANDOFF.md` is current.

## Completion record

Implementation commit:
CI run:
CI result:
Completed:
Notes:
