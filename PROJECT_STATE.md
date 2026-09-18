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
- Implementation status: not started; the Approved plan checkpoint must pass CI first.

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

P0-M018 will compare Ferraxis lexer acceptance with a stable rustc macro token-tree acceptance
probe. It will not claim access to rustc's raw lexer token stream.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Require the P0-M018 Approved-plan commit to pass CI. Only then add the differential harness
implementation under the unchanged Approved plan.
