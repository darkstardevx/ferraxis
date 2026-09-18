# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 0 foundation

## Current phase

Phase 0 — compiler foundation.

## Active milestone

No implementation milestone is active. P0-M022 is complete and awaiting merge of its closure
record.

## Completed workflow milestone

- `P0-M022` — Agent workflow completion.
- Implementation head: `93397095be100be722e4bf48c0fc00a159f143b8`
- Verified CI: https://github.com/darkstardevx/ferraxis/actions/runs/35406400983

## Next compiler milestone

- `P0-M018` — Differential lexer harness skeleton.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis currently provides source-file storage, byte positions and half-open spans, structured
diagnostic data, initial ASCII whitespace and identifier lexing, exact `fn` recognition, explicit
EOF tokens, and deterministic token dumping.

## Workflow state

The Rust-agent workflow is now repository-enforced: plan-first commits, repository hooks, tiered
gates, state/handoff records, stable plus MSRV validation, feature isolation, exact CI evidence,
shell validation, and release-readiness helpers are present.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Merge PR #1 only after the P0-M022 closure commit itself passes CI. Then synchronize local
`main`, install repository hooks with `./scripts/setup-dev`, and open P0-M018 with a new Draft
plan that is reviewed, Approved, and committed before implementation begins.
