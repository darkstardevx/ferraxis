# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 0 foundation

## Current phase

Phase 0 — compiler foundation.

## Active milestone

- `P0-M022` — Agent workflow completion.

## Next compiler milestone

- `P0-M018` — Differential lexer harness skeleton.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis currently provides source-file storage, byte positions and half-open spans, structured
diagnostic data, initial ASCII whitespace and identifier lexing, exact `fn` recognition, explicit
EOF tokens, and deterministic token dumping.

## Workflow state

P0-M022 establishes the complete Rust-agent workflow: plan-first commits, repository hooks, tiered
gates, state/handoff records, MSRV and feature isolation, CI evidence, and release-readiness
helpers.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

Obtain an exact green CI run for the P0-M022 implementation commit, close P0-M022 with that run as
evidence, then begin P0-M018 with a new committed Approved plan.
