# Ferraxis Project Specification

## Mission

Ferraxis is an independent implementation of the Rust language focused on compiler diversity,
portability, backend independence, and eventual self-hosting.

Its long-term objective is to provide an independently engineered path from Rust source to
executable machine code without depending on rustc internals or requiring LLVM.

## Non-goals

- Reproducing rustc's internal architecture for compatibility convenience.
- Treating LLVM, GCC, or Cranelift IR as Ferraxis's semantic compiler IR.
- Claiming broad Rust compatibility without reproducible evidence.
- Optimizing unsupported or insufficiently tested semantics.
- Combining unrelated milestones into broad rewrites.

## Semantic authority

Ferraxis resolves language behavior in this order:

1. explicit Rust language definition/specification;
2. stable documented Rust behavior;
3. observable `rustc` behavior;
4. differential tests;
5. documented Ferraxis extension or variance.

See `docs/SEMANTICS_AUTHORITY.md`.

## Architecture contract

The durable architecture contract is defined by:

- `docs/COMPILER_INVARIANTS.md`;
- accepted ADRs under `docs/adr/`;
- architecture documentation under `docs/architecture/`.

Frontend phases remain backend-neutral. Ferraxis owns its eventual code-generation IR.

## Work contract

Implementation is milestone-sized and plan-first.

A Rust or Cargo implementation commit requires an Approved plan already committed in `HEAD`.
Architecture decisions use permanent ADR IDs. Capability targets use permanent milestone IDs.
Language claims use permanent semantic evidence IDs.

## Toolchain contract

- Normal development toolchain: stable Rust.
- Declared MSRV: Rust 1.85.
- Repository checks use the committed `Cargo.lock`.
- Full validation includes all features and no default features.
- Public Rust APIs pass rustdoc with warnings denied.

## Completion definition

A milestone is not complete merely because code exists.

Completion requires:

1. acceptance criteria satisfied;
2. tests and evidence current;
3. relevant documentation current;
4. full local gate green;
5. implementation committed;
6. exact GitHub Actions run for that commit green;
7. completion evidence written into the plan;
8. project state and handoff current.
