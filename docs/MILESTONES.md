# Milestones

Milestone IDs are permanent. Completed, cancelled, split, or superseded milestones retain their
original IDs so old commits and discussions remain meaningful.

Status values are `planned`, `active`, `complete`, `blocked`, `split`, and `superseded`.

## Phase 0 — compiler foundation

| ID | Status | Milestone | Acceptance signal |
| --- | --- | --- | --- |
| P0-M001 | complete | Repository bootstrap | Workspace builds and repository layout exists. |
| P0-M002 | complete | Project governance | `AGENTS.md` and `CONTRIBUTING.md` define change rules. |
| P0-M003 | complete | Semantic authority policy | Five-level authority order is documented and validated. |
| P0-M004 | complete | Compiler invariants | Permanent invariant registry exists. |
| P0-M005 | complete | ADR system | ADR registry and initial decisions exist. |
| P0-M006 | complete | CI baseline | Code and documentation jobs are defined. |
| P0-M007 | complete | Test taxonomy | Compile, run, lexer, and differential test areas are reserved. |
| P0-M008 | complete | `SourceFile` abstraction | UTF-8 source can be stored and span-sliced. |
| P0-M009 | complete | `Span` representation | Byte positions and half-open spans are implemented. |
| P0-M010 | complete | Diagnostic skeleton | Structured diagnostic data exists independently of rendering. |
| P0-M011 | complete | Lexer token representation | Token and token-kind types exist. |
| P0-M012 | complete | EOF token | Empty input and completed streams emit EOF. |
| P0-M013 | complete | ASCII whitespace | Initial ASCII whitespace is skipped deterministically. |
| P0-M014 | complete | `fn` token | Exact `fn` keyword recognition exists. |
| P0-M015 | complete | ASCII identifiers | Initial identifier subset and keyword boundaries are tested. |
| P0-M016 | complete | Token-dump CLI | `--emit=tokens` emits deterministic token information. |
| P0-M017 | complete | Lexer regression tests | Boundary, span, whitespace, and unsupported-byte tests exist. |
| P0-M018 | complete | Differential lexer harness skeleton | Harness can record versioned rustc observations. |
| P0-M019 | complete | Documentation gate baseline | Rustdoc, structure validation, Markdown lint, and link checks exist. |
| P0-M020 | complete | Release metadata baseline | Pre-1.0 release ladder and release criteria are documented. |
| P0-M021 | planned | Licensing decision | Project license is selected and committed explicitly. |
| P0-M022 | complete | Agent workflow completion | Plan-first workflow, repository hooks, tiered gates, state/handoff records, MSRV/feature checks, and exact CI evidence are enforced. |

## Phase 1 — lexer completion

| ID | Status | Milestone |
| --- | --- | --- |
| P1-M001 | complete | Line comments |
| P1-M002 | complete | Block comments |
| P1-M003 | active | Nested block comments |
| P1-M004 | planned | Punctuation |
| P1-M005 | planned | Delimiters |
| P1-M006 | planned | Integer literals |
| P1-M007 | planned | Character literals |
| P1-M008 | planned | String literals |
| P1-M009 | planned | Raw strings |
| P1-M010 | planned | Byte strings |
| P1-M011 | planned | Raw identifiers |
| P1-M012 | planned | Lifetimes |
| P1-M013 | planned | Strict keywords |
| P1-M014 | planned | Reserved keywords |
| P1-M015 | planned | Unicode identifiers |
| P1-M016 | planned | Unknown-token diagnostics |
| P1-M017 | planned | Lexer conformance corpus |
| P1-M018 | planned | Lexer differential suite |

## Future phase namespace reservations

The high-level phase namespace is intentionally reserved now while detailed milestone IDs are
created only when the work is close enough to define meaningful acceptance criteria.

- `P2-*` — parser foundation;
- `P3-*` — name resolution and module system;
- `P4-*` — HIR;
- `P5-*` — type system and inference;
- `P6-*` — traits and coherence;
- `P7-*` — MIR;
- `P8-*` — borrow checking and regions;
- `P9-*` — Ferraxis IR;
- `P10-*` — code-generation interfaces and first backends;
- `P11-*` — `core` bring-up;
- `P12-*` — bootstrap and self-hosting.
