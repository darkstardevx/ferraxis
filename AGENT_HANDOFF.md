# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P0-M018`
- Active plan: `.plans/P0-M018-rustc-differential-lexer.plan.md`
- Plan status: `Approved`
- Implementation status: differential harness implemented; awaiting exact successful CI evidence

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Read the active P0-M018 plan.
6. Read ADR-0001, ADR-0006, ADR-0011, and ADR-0012.
7. Read `docs/SEMANTICS_AUTHORITY.md` and `docs/TESTING.md`.
8. Confirm the Approved-plan commit passed CI before changing Rust/Cargo implementation.
9. Never bypass repository hooks.

## Current work

P0-M018 now contains the non-publishable `ferraxis-diff` tool, committed lexer corpus and expected
classifications, deterministic evidence output, and a dedicated CI differential job.

## Observation boundary

The rustc side is a stable macro token-tree acceptance probe. It is not a raw rustc lexer dump and
must not be described as token-for-token equivalence.

## Next exact action

Require the implementation head to pass the full PR CI matrix. Inspect the uploaded
`p0-m018-differential-lexer` artifact. If all jobs are green and the evidence matches the Approved
plan, record that exact CI run in the plan and close P0-M018.

## Validation rule

Differential classifications are evidence, not correctness verdicts. Interpret mismatches through
Ferraxis's semantic-authority hierarchy.
