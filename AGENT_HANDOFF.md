# Ferraxis Agent Handoff

## Repository state

- Active milestone: `P0-M022`
- Active plan: `.plans/P0-M022-agent-workflow-completion.plan.md`
- Expected plan status: `Approved`
- Next compiler milestone after closure: `P0-M018`

## Resume checklist

1. Read `PROJECT_SPEC.md`.
2. Read `PROJECT_STATE.md`.
3. Read `AGENTS.md`.
4. Run `./scripts/project-status`.
5. Read `.plans/ACTIVE` and the active plan.
6. Read relevant ADRs, invariants, and semantic evidence before changing implementation.
7. Never begin Rust implementation from a Draft or uncommitted plan.
8. Never bypass repository hooks.

## Current work

Complete the P0-M022 agent-workflow infrastructure and obtain an exact green CI run for the
implementation commit.

## Next exact action

Run the full local gate, commit the workflow implementation, push the exact commit, run
`./scripts/ci-watch`, then close P0-M022 with `./scripts/plan close --ci-run <URL>`.

## Validation rule

Do not claim completion based only on local success. Exact GitHub Actions evidence belongs in the
completed plan.
