# ADR-0006: Differential testing against rustc

- Status: Accepted
- Date: 2026-09-18
- Decision owners: Ferraxis project

## Context

An independent implementation needs broad compatibility evidence beyond hand-written examples.

## Decision

Ferraxis will build versioned differential tests that compare observable behavior with rustc where
such comparison is meaningful. Differential evidence is subordinate to explicit specification and
stable documented behavior.

## Consequences

Test records must capture compiler versions and comparison scope. A difference is investigated, not
automatically classified as a Ferraxis defect.

## References

- `docs/SEMANTICS_AUTHORITY.md`
- `docs/TESTING.md`
