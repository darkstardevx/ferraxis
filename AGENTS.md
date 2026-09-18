# Ferraxis Agent Rules

These rules apply to human contributors, coding agents, and automated change generators.
Compiler work must remain reviewable years after the original change was made.

## Required reading

Before changing compiler code, read:

1. `docs/COMPILER_INVARIANTS.md`.
2. `docs/SEMANTICS_AUTHORITY.md`.
3. the ADRs relevant to the subsystem being changed.
4. the semantic evidence record relevant to the language behavior being changed.
5. the current milestone definition in `docs/MILESTONES.md`.

## Architecture rules

1. Never add a dependency on rustc-private crates.
2. Never silently reproduce rustc internal architecture merely because rustc does it that way.
3. Never introduce LLVM-, GCC-, Cranelift-, or target-specific types into frontend crates.
4. Keep lexing, parsing, semantic analysis, borrow checking, IR lowering, and code generation as
   explicit phases.
5. New architectural boundaries require an ADR.
6. Existing accepted ADRs may not be bypassed by an implementation shortcut. Amend or supersede
   the ADR first.
7. Ferraxis-owned IR must remain backend neutral.
8. Do not add a dependency without documenting why the standard library is insufficient.

## Semantic rules

1. Never invent Rust semantics from intuition.
2. Resolve semantic questions using the authority order in `docs/SEMANTICS_AUTHORITY.md`.
3. Compatibility claims require reproducible tests and an evidence record.
4. When specification text and `rustc` behavior disagree, preserve the observation, add a test,
   and document the disagreement. Do not silently pick whichever behavior makes a test pass.
5. Every intentional Ferraxis variance must be documented before merge.
6. Unsupported behavior must be distinguished from incorrectly implemented behavior.

## Test rules

1. One semantic behavior change requires at least one regression test.
2. Compiler ICEs caused by user input are bugs and require regression tests.
3. Invalid source must produce controlled failure data or diagnostics, not a panic.
4. Do not weaken a conformance test to make a change pass.
5. Do not mark a milestone complete until its acceptance criteria pass.
6. Differential tests supplement specifications; they do not replace them.

## Documentation rules

1. Documentation is part of the build and must pass the docs gate.
2. Architecture changes require matching architecture documentation.
3. New ADRs must use the next permanent ADR number; never renumber historical ADRs.
4. Superseded ADRs remain in the repository.
5. New language behavior requires or updates a semantic evidence record.
6. Milestone IDs are permanent. Never recycle or renumber them.
7. Broken internal or external documentation links are CI failures unless explicitly exempted.
8. Rust public APIs must remain free of rustdoc warnings.

## Change-scope rules

1. Prefer one milestone-sized change over a broad rewrite.
2. Do not bundle unrelated milestones into one change.
3. Prefer complete correctness for the supported subset over premature feature breadth.
4. Do not optimize behavior that lacks correctness tests.
5. Preserve deterministic compiler output where the architecture permits it.

## Commit requirements

Commit messages for compiler changes should state:

- what changed;
- why it changed;
- architecture affected;
- tests added or updated;
- milestone IDs advanced;
- ADR IDs affected;
- compiler invariant IDs affected;
- semantic evidence IDs affected.

A recommended body footer is:

```text
Milestones: P0-M014, P0-M015
ADRs: ADR-0001, ADR-0003
Invariants: INV-005, INV-013
Semantics: SEM-LEX-0001
```

## Required gate

Before declaring work complete, run:

```bash
./scripts/check.sh
```

When documentation changed, also run:

```bash
./scripts/check-docs.sh
```

## Plan-first workflow

Rust and Cargo implementation is plan-first.

1. Inspect repository state with `./scripts/project-status`.
2. Select one permanent milestone.
3. Create a plan with `./scripts/plan new <milestone> <slug>`.
4. Edit the plan until file boundaries, tests, failure modes, documentation, and acceptance criteria
   are explicit.
5. Approve it with `./scripts/plan approve`.
6. Commit the Approved plan before implementation.
7. Only then modify Rust or Cargo implementation.
8. Run `./scripts/gate.sh fast` during development and `./scripts/gate.sh full` before completion.
9. Push the exact implementation commit and require exact CI success.
10. Close the plan only with the successful CI run recorded.

The Approved plan and implementation must not be introduced in the same commit.

## Agent startup checklist

Before editing:

- run `git status`;
- inspect recent commits;
- run `./scripts/project-status`;
- read `PROJECT_SPEC.md`;
- read `PROJECT_STATE.md`;
- read `AGENT_HANDOFF.md`;
- read the active plan;
- read relevant ADRs, invariants, and semantic evidence.

## No-bypass rules

Coding agents must not:

- use `git commit --no-verify`;
- disable or weaken a failing test merely to advance a milestone;
- skip required documentation updates;
- lower lint levels to hide failures;
- silently raise the MSRV;
- silently change dependency-resolution policy;
- close a plan without exact successful CI evidence.

## Recovery rules

After a partially successful mutating command or script, inspect current state and repair forward.
Do not blindly use `git reset --hard`, `git clean -fd`, or broad file deletion as a recovery
strategy.

## Toolchain and feature rules

The declared MSRV is Rust 1.85 and is tested independently from stable. Repository checks use the
committed `Cargo.lock`. Full validation includes all features and no default features. Future
backend feature flags require explicit isolation coverage.

## Generated text

Tracked text files use LF line endings and end with exactly one newline. Prefer structural patches
over brittle formatting-sensitive anchors.

## Completion evidence

A milestone requires an implementation commit and exact successful GitHub Actions run before its
plan can become Complete. Update `PROJECT_STATE.md` and `AGENT_HANDOFF.md` at handoff boundaries.
