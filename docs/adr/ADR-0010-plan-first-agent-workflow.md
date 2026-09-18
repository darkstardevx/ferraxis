# ADR-0010: Plan-first agent workflow

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Compiler implementation changes can alter architecture, compatibility behavior, and long-term
maintenance cost. Milestones and ADRs do not by themselves describe the exact intended file
boundary, test matrix, failure modes, or implementation sequence of one concrete change.

## Decision

Ferraxis uses a plan-first implementation workflow.

Rust and Cargo implementation changes require one active implementation plan in `.plans/ACTIVE`.
That plan must already exist in `HEAD` with `Status: Approved` before implementation is
committed. The Approved plan and implementation may not be introduced in the same commit.

Completed plans remain in the repository and record the implementation commit and exact successful
CI run.

Repository-owned pre-commit hooks enforce this boundary for implementation changes. Coding agents
must not bypass hooks with `git commit --no-verify`.

## Consequences

Implementation intent becomes reviewable before code exists. Historical work can be reconstructed
from milestones, ADRs, semantic evidence, plans, commits, and CI records.

## References

- `.plans/README.md`
- `AGENTS.md`
- `docs/DEVELOPMENT_WORKFLOW.md`
