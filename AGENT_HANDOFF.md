# Ferraxis Agent Handoff

## Repository state

- Completed milestone: `P0-M022`
- Active milestone: none
- Active plan: none
- Next compiler milestone: `P0-M018`
- Verified P0-M022 implementation CI: https://github.com/darkstardevx/ferraxis/actions/runs/35406400983

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Confirm there is no stale `.plans/ACTIVE`.
6. Read ADR-0006 before differential-testing work.
7. Create the P0-M018 plan before changing Rust implementation.
8. Never bypass repository hooks.

## Completed work

P0-M022 made Ferraxis compiler-governance complete and agent-workflow complete. The final verified
implementation head passed stable code, feature-isolation, MSRV 1.85.0, repository workflow, and
documentation gates.

## Next exact action

After PR #1 merges and `main` CI is green, create a dedicated P0-M018 branch and run:

```bash
./scripts/plan new P0-M018 rustc-differential-lexer
```

Fill the plan with the differential-evidence contract, corpus boundary, version capture, result
classification, failure modes, and tests. Approve and commit that plan before implementing the
harness.

## Validation rule

Do not claim P0-M018 progress from prototype code alone. Differential observations must be
reproducible, versioned, and interpreted through Ferraxis's semantic-authority hierarchy.
