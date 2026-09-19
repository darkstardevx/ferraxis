# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 1 lexer completion

## Current phase

Phase 1 — lexer completion.

## Active milestone

No implementation milestone is currently active and `.plans/ACTIVE` is intentionally absent.

## Recently completed milestone

- `P1-M003` — Nested block comments.
- Final validated implementation head: `e3c37407fed0592c1b6f5c71e24ff355d5fa664b`.
- Exact implementation CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35419741904>.
- Differential artifact: `p0-m018-differential-lexer`, artifact ID `10577216286`.
- Result: 27/27 committed differential classifications matched.
- Recursive block-comment nesting is no longer a Ferraxis variance for ordinary top-level block
  comments.

## Earlier completed Phase 1 milestones

- `P1-M002` — Block comments.
- Merge commit: `6d6762aacdc0ab371404938c95d2a5832755ab6b`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35418267654>.
- `P1-M001` — Line comments.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary non-doc line
comments, recursively nested ordinary non-doc block comments, explicit EOF tokens, deterministic
token dumping, and a versioned differential lexer-observation harness.

Nested ordinary, outer-doc, and inner-doc block forms are tracked recursively when they appear
inside an ordinary outer block comment. The scanner is iterative and introduces no arbitrary
nesting-depth cap.

Top-level line and block documentation comments remain unsupported because their attribute
semantics are still out of scope.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

No Rust or Cargo implementation may begin until a new milestone plan is created, reviewed,
Approved, committed by itself, and validated by CI. The next planned compiler milestone is
`P1-M004` — punctuation.
