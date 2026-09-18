# Plan: P0-M022 — Agent workflow completion

Status: Complete
Milestone: P0-M022
Created: 2026-09-18

## Goal

Bring Ferraxis into full alignment with the Rust-agent blueprint so compiler governance and agent
workflow are both repository-enforced rather than convention-only.

## Non-goals

- No Rust language semantics change.
- No lexer behavior change.
- No compiler dependency addition.
- No licensing decision; P0-M021 remains separate.
- No implementation of P0-M018.

## Context

Ferraxis already has compiler invariants, numbered ADRs, permanent milestone IDs, semantic
evidence, code gates, documentation gates, and an MSRV CI job. It is missing the stronger
plan-first, handoff, repository-hook, tiered-gate, local MSRV/feature-isolation, CI-observation,
and release-readiness workflow from the Rust blueprint.

## Architecture placement

This milestone changes repository governance and development infrastructure only. It does not
change the compiler pipeline.

## Data flow

```text
milestone
  -> approved committed plan
  -> implementation
  -> local gates
  -> implementation commit
  -> exact CI result
  -> plan closure
  -> project state + agent handoff
```

## Invariants

- INV-018
- INV-021
- INV-022

## ADRs

- ADR-0009
- ADR-0010
- ADR-0011

## Semantic evidence

None. This milestone changes workflow, not Rust language behavior.

## Public API / CLI

No compiler CLI changes. New repository workflow commands live under `scripts/`.

## Compatibility analysis

Stable remains the normal development toolchain. Rust 1.85.0 remains the declared MSRV and is
validated independently. Existing compiler and documentation gates must remain green.

## Dependency analysis

No compiler dependencies are added. Repository helpers use existing developer tools such as Git,
Cargo, rustup, GitHub CLI, npx, Lychee, and optionally ShellCheck/Bacon.

## Expected file boundary

- `.githooks/`
- `.plans/`
- `.github/workflows/ci.yml`
- `scripts/`
- `AGENTS.md`
- `CONTRIBUTING.md`
- `README.md`
- `PROJECT_SPEC.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`
- `bacon.toml`
- `docs/DEVELOPMENT_WORKFLOW.md`
- `docs/RELEASES.md`
- `tools/xtask/src/main.rs`

## Test-first matrix

| Behavior | Test | Expected result |
| --- | --- | --- |
| Rust change without committed Approved plan | pre-commit gate | rejected |
| Rust change with Approved plan in HEAD | pre-commit gate | accepted if code gates pass |
| Plan and Rust implementation staged together | pre-commit gate | rejected |
| All-feature workspace build | full gate | passes |
| No-default-feature workspace build | feature gate | passes |
| Rust 1.85.0 workspace check/test | MSRV gate | passes |
| Markdown and links | docs gate | passes |
| Workflow scripts | Bash syntax/ShellCheck gate | passes |
| Workflow records | xtask validation | passes |

## Implementation sequence

1. Commit this Approved plan and workflow ADRs before implementation.
2. Add repository-owned hooks and plan tooling.
3. Add tiered gates plus local MSRV and feature-isolation checks.
4. Add project state, handoff, CI observation, and release-readiness helpers.
5. Expand repository validation and CI.
6. Run the full local gate.
7. Commit implementation while this Approved plan remains unchanged in HEAD.
8. Push and require exact CI success.
9. Close the plan only with implementation commit and CI run recorded.

## Failure modes

- Hook bypass weakens repository policy.
- Agent changes the plan in the same commit as implementation.
- Stable-only testing silently breaks MSRV.
- All-feature-only testing hides no-default-feature failures.
- Documentation state diverges from implementation.
- A milestone is closed without exact CI evidence.

## Documentation impact

Add explicit project specification, project state, handoff, workflow documentation, and
repository rules.

## Quality gates

- `./scripts/gate.sh precommit`
- `./scripts/gate.sh fast`
- `./scripts/gate.sh full`
- exact GitHub Actions success for the implementation commit

## Acceptance criteria

- [x] Plan-first workflow is repository-enforced.
- [x] Hooks are repository-owned and installable per clone.
- [x] Tiered local gates exist.
- [x] Stable, MSRV, all-feature, and no-default-feature paths are tested.
- [x] Documentation remains a hard gate.
- [x] Project state and agent handoff records exist.
- [x] GitHub CI observation helpers exist.
- [x] Release-readiness helper exists and respects unfinished licensing metadata.
- [x] Workflow structure is validated by xtask.
- [x] Exact implementation CI evidence is required before closure.

## Completion record

Implementation commit: 93397095be100be722e4bf48c0fc00a159f143b8
CI run: https://github.com/darkstardevx/ferraxis/actions/runs/35406400983
CI result: success
Completed: 2026-09-18
Notes: CI exposed and verified two workflow-hardening corrections before closure: canonical rustfmt layout in xtask and shell-safe Markdown handoff generation. Final validation passed stable code, feature isolation, Rust 1.85.0 MSRV, repository workflow/ShellCheck, and documentation/link jobs.
