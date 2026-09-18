# Ferraxis Implementation Plans

Ferraxis implementation work is plan-first.

Architecture Decision Records explain durable architecture. Milestones identify long-lived
capability targets. Semantic evidence records explain why language behavior is believed correct.
Implementation plans describe how one milestone-sized change will be implemented and verified.

## Active plan

`.plans/ACTIVE` contains the path of the one active implementation plan.

Rust or Cargo implementation changes require the active plan to:

1. exist in `HEAD`;
2. have `Status: Approved`;
3. remain unchanged in the implementation commit.

The Approved plan and implementation are intentionally separate commits.

## States

Plans use these states:

- `Status: Draft`
- `Status: Approved`
- `Status: Complete`

Completed plans remain in the repository and record the implementation commit plus the exact
successful CI run.

## Typical workflow

```bash
./scripts/plan new P0-M018 rustc-differential-lexer
# edit the generated plan
./scripts/plan approve
git add .plans/ docs/MILESTONES.md
git commit -m "docs(plan): approve P0-M018 differential lexer harness"

# implementation begins only after that commit
./scripts/gate.sh fast
./scripts/gate.sh full
git commit

git push
./scripts/ci-watch

# after exact CI success
./scripts/plan close --ci-run https://github.com/darkstardevx/ferraxis/actions/runs/<id>
```
