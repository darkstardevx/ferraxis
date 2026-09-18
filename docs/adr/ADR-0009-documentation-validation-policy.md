# ADR-0009: Documentation is a validated build artifact

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

Compiler architecture and semantic policy live partly in documentation. Stale ADRs, broken evidence
records, invalid milestone references, and broken links can mislead contributors as severely as
failing code.

## Decision

Documentation participates in CI. Ferraxis validates rustdoc warnings, repository metadata,
Markdown style, and links. ADR, milestone, and semantic-evidence structure is checked by project
owned tooling.

## Consequences

Documentation changes can fail CI. The repository must update code and authoritative documentation
together.

## References

- `scripts/check-docs.sh`
- `tools/xtask/`
