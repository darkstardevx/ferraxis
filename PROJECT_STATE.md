# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 1 lexer completion

## Current phase

Phase 1 — lexer completion.

## Active milestone

- `P1-M002` — Block comments.
- Active plan: `.plans/P1-M002-block-comments.plan.md`.
- Plan status: Approved.
- Implementation status: implemented on the feature branch; exact implementation CI and evidence
  inspection are still required.
- Approved plan checkpoint: `0ce202345327701705763364da3dc3859a55a376`.
- Plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35417983279>.

## Recently completed milestone

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

## P1-M002 semantic boundary

Supported depth-one ordinary block comments are lexical whitespace.

The implementation correctly treats `/**/`, `/***/`, and `/*** text */` as ordinary comments while
leaving `/** text */` and `/*! text */` block documentation syntax unsupported.

Unterminated comments fail at the original opening slash. Nested `/*` fails at the nested opener
until P1-M003 implements recursive nesting.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the exact P1-M002 implementation head to pass the complete CI matrix. Inspect uploaded
differential evidence, then close P1-M002 only if supported, unterminated, and nested cases match
their committed classifications.
