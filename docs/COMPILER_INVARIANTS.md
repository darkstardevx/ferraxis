# Compiler Invariants

Compiler invariants are durable project constraints. A change that violates an invariant must not
be merged until the invariant is explicitly amended through an ADR-backed architecture change.

- **INV-001:** Ferraxis does not depend on rustc-private crates.
- **INV-002:** LLVM is never required by the compiler architecture.
- **INV-003:** GCC is never required by the compiler architecture.
- **INV-004:** Backend-specific types may not leak into frontend IRs.
- **INV-005:** Source positions use byte offsets internally.
- **INV-006:** Every diagnostic referring to source code carries a span when a meaningful source
  location exists.
- **INV-007:** AST preserves syntax; HIR represents semantic structure.
- **INV-008:** Type checking does not perform machine-code generation.
- **INV-009:** Borrow checking is an explicit compiler phase.
- **INV-010:** MIR is independent of any machine architecture.
- **INV-011:** Ferraxis IR is owned and specified by the Ferraxis project.
- **INV-012:** Code-generation backends consume Ferraxis IR through a defined backend interface.
- **INV-013:** Language-behavior disagreements require a conformance test or documented reason a
  test cannot yet exist.
- **INV-014:** Every accepted variance from stable Rust behavior is documented.
- **INV-015:** Compatibility claims require reproducible evidence.
- **INV-016:** Compiler panics caused by user-controlled source input are bugs.
- **INV-017:** Invalid Rust must produce controlled failure data or diagnostics rather than an ICE.
- **INV-018:** New compiler behavior requires regression coverage.
- **INV-019:** Parsing and semantic analysis must remain deterministic for identical inputs and
  configuration.
- **INV-020:** Self-hosting remains a long-term architectural requirement.
- **INV-021:** Documentation describing current behavior must be validated in CI.
- **INV-022:** Milestone, ADR, and semantic-evidence identifiers are permanent once published.
