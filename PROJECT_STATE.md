# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 0 foundation

## Current phase

Phase 0 — compiler foundation.

## Active milestone

- `P0-M018` — Differential lexer harness skeleton.
- Plan: `.plans/P0-M018-rustc-differential-lexer.plan.md`
- Plan status: Approved.
- Implementation status: harness implemented on the feature branch; final CI evidence and closure are still required.

## Completed workflow milestone

- `P0-M022` — Agent workflow completion.
- Main merge: `4804cc98f13374845d926603680974d6552a2232`
- Main merge CI: <https://github.com/darkstardevx/ferraxis/actions/runs/35406658633>

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis currently provides source-file storage, byte positions and half-open spans, structured
diagnostic data, initial ASCII whitespace and identifier lexing, exact `fn` recognition, explicit
EOF tokens, and deterministic token dumping.

## Differential-testing boundary

P0-M018 compares Ferraxis lexer acceptance with a stable rustc macro token-tree acceptance
probe. It does not claim access to rustc's raw lexer token stream. Generated evidence is written
below `target/differential/lexer/` and uploaded by the dedicated CI differential job.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the P0-M018 implementation head to pass the complete PR CI matrix, including the
dedicated differential job and MSRV. Only then record exact CI evidence and close P0-M018.
