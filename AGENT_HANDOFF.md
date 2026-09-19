# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P1-M001`.
- Active plan: `.plans/P1-M001-line-comments.plan.md`.
- Plan status: `Approved`.
- Implementation status: not started.
- Required checkpoint: complete green CI for the plan-only commit before Rust/Cargo changes.

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Read the active P1-M001 plan.
6. Read `SEM-LEX-0004`.
7. Read ADR-0001, ADR-0003, ADR-0006, ADR-0011, and ADR-0012.
8. Confirm the Approved-plan commit passed CI before changing Rust or Cargo implementation.
9. Never bypass repository hooks or weaken a gate.

## Current work

P1-M001 will add ordinary Rust non-documentation line comments to the lexer.

The semantic boundary is intentionally narrow:

- ordinary `//...` comments are whitespace;
- EOF and LF termination are supported;
- `////...` is ordinary comment syntax;
- `///...` remains unsupported outer documentation-comment syntax;
- `//!...` remains unsupported inner documentation-comment syntax.

## Evidence basis

Primary authority is the Rust Reference comments grammar. The P0-M018 differential harness will be
used as additional L4 evidence after implementation.

The existing `line-comment` differential case should move from `ferraxis_rejects` to
`agree_accept` once implementation lands.

## Next exact action

Require the plan-only head to pass the full CI matrix. If it is green, implement P1-M001 inside the
approved file boundary, update differential expectations, inspect evidence, and close only after
exact implementation CI success.

## Validation rule

Do not collapse documentation comments into ordinary comments. Differential classifications are
evidence, not correctness verdicts.
