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

- `P1-M002` — Block comments.
- Final validated implementation head: `92ea27771b195f341da9062ca871eddb131e7363`.
- Exact implementation CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35418129218>.
- Differential artifact: `p0-m018-differential-lexer`, artifact ID `10576662999`.
- Result: 22/22 committed differential classifications matched.
- Intentional remaining lexer gap: nested block comments are reserved for `P1-M003`.

## Earlier completed Phase 1 milestone

- `P1-M001` — Line comments.
- Merge commit: `cc28d3265d6cc18af520593226158016b5423990`.
- Post-merge main CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35417271818>.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis provides source-file storage, byte positions and half-open spans, structured diagnostic
data, ASCII whitespace and identifier lexing, exact `fn` recognition, ordinary non-doc line
comments, depth-one ordinary non-doc block comments, explicit EOF tokens, deterministic token
dumping, and a versioned differential lexer-observation harness.

Supported block comments include multiline, UTF-8, `/**/`, `/***/`, and `/*** text */`
forms. Outer `/** text */` and inner `/*! text */` block documentation comments remain
unsupported.

Unterminated block comments fail in a controlled way. Nested `/*` fails at the nested opener
until P1-M003 implements recursive nesting.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

No Rust or Cargo implementation may begin until a new milestone plan is created, reviewed,
Approved, committed by itself, and validated by CI. The next planned compiler milestone is
`P1-M003` — nested block comments.
