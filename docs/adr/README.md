# Architecture Decision Records

ADRs are permanent numbered records of consequential project decisions.

## Numbering

- IDs use `ADR-NNNN`.
- Numbers are never recycled.
- Historical ADRs are never renumbered.
- A changed decision is superseded by a new ADR that references the old one.
- Number ranges may be left unused so related future decisions can cluster naturally.

## Required metadata

Every ADR contains `Status`, `Date`, and `Decision owners` fields followed by Context, Decision,
Consequences, and References sections.

## Registry

| ID | Status | Decision |
| --- | --- | --- |
| ADR-0001 | Accepted | Semantic authority hierarchy |
| ADR-0002 | Accepted | Independent compiler implementation |
| ADR-0003 | Accepted | Byte-offset source spans |
| ADR-0004 | Accepted | Multi-crate compiler architecture |
| ADR-0005 | Accepted | Backend-neutral Ferraxis IR |
| ADR-0006 | Accepted | Differential testing against rustc |
| ADR-0007 | Accepted | Compiler panic and ICE policy |
| ADR-0008 | Accepted | Dependency-minimization policy |
| ADR-0009 | Accepted | Documentation is a validated build artifact |
| ADR-0010 | Accepted | Plan-first agent workflow |
| ADR-0011 | Accepted | Toolchain and reproducible validation |
| ADR-0012 | Accepted | Stable rustc token-tree observation boundary |
| ADR-0013 | Accepted | Flat delimiter lexing; grouping after lexing |
