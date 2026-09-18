# Development Workflow

Ferraxis uses repository-enforced, plan-first development.

## Start every work session

```bash
git status
git log -5 --oneline
./scripts/project-status
./scripts/plan status
```

Then read `PROJECT_SPEC.md`, `PROJECT_STATE.md`, `AGENTS.md`, the active plan, relevant ADRs,
compiler invariants, and semantic evidence.

## Start a milestone

```bash
./scripts/plan new P0-M018 rustc-differential-lexer
```

Edit the plan until its architecture placement, test matrix, file boundary, failure modes, and
acceptance criteria are reviewable.

Then:

```bash
./scripts/plan approve
git add .plans/ docs/MILESTONES.md
git commit -m "docs(plan): approve P0-M018 differential lexer harness"
```

Only after that commit may Rust or Cargo implementation begin.

## Gate levels

### Pre-commit

`./scripts/gate.sh precommit`

Checks staged whitespace/text policy and, for implementation changes, requires the Approved plan
from `HEAD`, formatting, all-target/all-feature Cargo check, and strict Clippy.

### Fast

`./scripts/gate.sh fast`

Runs stable-toolchain formatting, metadata, check, Clippy, tests, and repository validation.

### Full

`./scripts/gate.sh full`

Adds no-default-feature validation, MSRV validation, shell-script checks, and the complete
documentation gate.

### Release

`./scripts/gate.sh release`

Adds package-content/readiness checks. It is intentionally blocked until P0-M021 completes the
licensing decision.

## Exact CI closure

After the implementation commit:

```bash
git push
./scripts/ci-watch
```

Use `./scripts/ci-failures` if the run fails.

A milestone closes only after the exact implementation commit has a successful GitHub Actions run.

## Recovery

After a failed mutating script, inspect the actual repository state and repair forward. Do not
blindly run destructive cleanup such as `git reset --hard` or `git clean -fd`.

## Hook policy

Install hooks once per clone:

```bash
./scripts/install-hooks
```

Coding agents must never use `git commit --no-verify`.

## Generated text

Tracked text files use LF line endings and end with exactly one newline. Prefer structural patches
and validation over formatting-sensitive replacement anchors.
