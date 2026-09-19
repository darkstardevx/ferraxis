# Ferraxis Project State

This file records current project state, not architectural history.

## Current release

- Workspace version: `0.0.3`
- Release line: Phase 0 foundation

## Current phase

Phase 0 — compiler foundation.

## Active milestone

No implementation milestone is currently active and `.plans/ACTIVE` is intentionally absent.

## Recently completed milestone

- `P0-M018` — Differential lexer harness skeleton.
- Final validated head: `dc9b792c42e1fe111ea60c88358ec8c2f9d9038a`.
- CI evidence: <https://github.com/darkstardevx/ferraxis/actions/runs/35415625792>.
- Differential artifact: `p0-m018-differential-lexer`.
- Result: 11/11 committed classifications matched.

## Completed workflow milestone

- `P0-M022` — Agent workflow completion.
- Main merge: `4804cc98f13374845d926603680974d6552a2232`.
- Main merge CI: <https://github.com/darkstardevx/ferraxis/actions/runs/35406658633>.

## Other open Phase 0 decisions

- `P0-M021` — Licensing decision.

## Current compiler capability

Ferraxis currently provides source-file storage, byte positions and half-open spans, structured
diagnostic data, initial ASCII whitespace and identifier lexing, exact `fn` recognition, explicit
EOF tokens, deterministic token dumping, and a versioned differential lexer-observation harness.

## Differential-testing boundary

P0-M018 compares Ferraxis lexer acceptance with a stable rustc macro token-tree acceptance probe.
It does not claim access to rustc's raw lexer token stream or token-for-token equivalence.

The final P0-M018 CI evidence used rustc 1.98.1 on `x86_64-unknown-linux-gnu`. All 11 committed
classifications matched. Generated evidence remains under `target/differential/lexer/` during
runs and is uploaded by the dedicated CI differential job.

## Known blockers

The release gate remains intentionally blocked until P0-M021 selects and commits a project license
and package metadata is release-ready.

## Next exact action

No implementation may begin until a new milestone plan is created, reviewed, Approved, and
committed. The next planned compiler milestone is `P1-M001` — line comments. P0-M021 licensing
remains a separate Phase 0 release-governance decision.
