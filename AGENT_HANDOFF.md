# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P1-M002`.
- Active plan: `.plans/P1-M002-block-comments.plan.md`.
- Plan status: `Approved`.
- Implementation status: not started.
- Required checkpoint: exact plan-only CI success before Rust or Cargo changes.

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Read the active P1-M002 plan.
6. Read `SEM-LEX-0005`.
7. Read ADR-0001, ADR-0003, ADR-0006, ADR-0011, and ADR-0012.
8. Confirm the exact Approved-plan checkpoint passed CI before implementation.
9. Never bypass repository hooks or weaken a gate.

## Current work

P1-M002 will implement the non-nested subset of ordinary Rust block comments.

Important frozen boundaries:

- `/* ... */` ordinary comments become whitespace;
- `/**/` is an ordinary empty block comment;
- `/***/` and `/*** text */` are ordinary comments;
- `/** text */` remains unsupported outer block documentation syntax;
- `/*! text */` remains unsupported inner block documentation syntax;
- nested `/*` inside an open block comment is rejected at the nested opener until P1-M003;
- EOF before `*/` is a controlled lexical failure.

## Evidence basis

Primary authority is the Rust Reference comments grammar. The differential harness supplies
additional L4 observation evidence.

P1-M002 will add supported block-comment cases, an unterminated case expected to `agree_reject`,
and an intentional nested-comment case expected to remain `ferraxis_rejects` until P1-M003.

## Observation boundary

The rustc side remains a stable macro token-tree acceptance probe. It is not a raw rustc lexer
dump and must not be described as token-for-token equivalence.

## Next exact action

Require the plan-only head to pass the complete CI matrix. If green, implement only the approved
depth-one block-comment scope, inspect differential evidence, close P1-M002, and merge only after
closed-state CI succeeds.

## Validation rule

Do not collapse block documentation comments into ordinary comments. Do not accidentally truncate
nested comments at an inner `*/`. Differential classifications are evidence, not correctness
verdicts.
