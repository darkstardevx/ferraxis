# Contributing to Ferraxis

Ferraxis is intentionally developed in small, evidence-backed milestones. A compiler can become
unmaintainable quickly if architecture, semantics, and tests drift apart.

## Before starting

Read `AGENTS.md`, `docs/COMPILER_INVARIANTS.md`, `docs/SEMANTICS_AUTHORITY.md`, and all ADRs
relevant to the subsystem you plan to change.

## Workflow

1. Select one milestone or a tightly related milestone set.
2. Identify the relevant semantic evidence records.
3. Add or update tests first when practical.
4. Implement the smallest complete behavior.
5. Update docs, ADRs, evidence records, and milestone status in the same change.
6. Run `./scripts/check.sh`.
7. If docs changed, run `./scripts/check-docs.sh`.
8. Commit with detailed rationale and traceability IDs.

## Architecture changes

Architecture changes require an ADR before or with the implementation. Historical ADR numbers
are permanent. To change an accepted decision, add a superseding ADR rather than rewriting
history.

## Semantic changes

Semantic claims must cite the strongest available source according to
`docs/SEMANTICS_AUTHORITY.md`. `rustc` behavior is evidence, not automatically the language
specification.

## Pull requests

Pull requests should identify:

- milestones;
- ADRs;
- invariants;
- semantic evidence records;
- tests and commands run;
- known unsupported behavior.

Do not describe a feature as "Rust compatible" without stating the tested behavior and evidence.

## Plan-first implementation

Implementation changes require an Approved plan already committed in `HEAD`.

```bash
./scripts/plan new P0-M018 rustc-differential-lexer
# edit the generated plan
./scripts/plan approve
git add .plans/ docs/MILESTONES.md
git commit -m "docs(plan): approve P0-M018 differential lexer harness"
```

After that commit, implement the milestone and use:

```bash
./scripts/gate.sh fast
./scripts/gate.sh full
```

Install repository hooks once per clone with `./scripts/install-hooks`.

Milestone closure requires the exact successful GitHub Actions run for the implementation commit.
See `docs/DEVELOPMENT_WORKFLOW.md`.
