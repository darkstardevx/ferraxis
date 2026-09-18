# Ferraxis

Ferraxis is an independent Rust compiler implementation focused on compiler diversity,
portability, self-hosting, and backend independence.

## The gap

Rust has an exceptionally capable reference compiler, but production Rust remains highly
concentrated around a single compiler implementation and its primary LLVM backend.

Ferraxis is an independent implementation of the Rust language designed around compiler
diversity, portability and self-hosting. It implements its own frontend, semantic analysis,
intermediate representation and code-generation interfaces, without depending on rustc
internals or requiring LLVM.

The project's long-term objective is not to replace rustc, but to provide a second independently
engineered path from Rust source code to executable machine code.

## Project principles

- Correctness over compatibility hacks.
- Compatibility over optimization.
- Specification over implementation accident.
- Portability over backend convenience.
- Evidence over assumptions.

## Semantic authority

When sources disagree or a behavior is unclear, Ferraxis uses this order of authority:

1. Explicit Rust language definition/specification.
2. Stable documented Rust behavior.
3. `rustc` observable behavior.
4. Differential tests.
5. Documented Ferraxis extension/variance.

See [Semantic authority](docs/SEMANTICS_AUTHORITY.md) for the complete policy.

## Current phase

Ferraxis is in **Phase 0: compiler foundation**. The first executable slice provides:

- source-file storage;
- byte positions and spans;
- diagnostic data structures;
- `fn`, ASCII identifiers, ASCII whitespace, and EOF lexing;
- a deterministic token-dump CLI;
- architecture, semantic-evidence, milestone, and documentation validation.

```text
fn main
```

can be inspected with:

```bash
cargo run -p ferraxis -- --emit=tokens path/to/input.rs
```

## Development gates

Run the complete local gate with:

```bash
./scripts/check.sh
```

Documentation is a build artifact in Ferraxis. It has its own gate:

```bash
./scripts/check-docs.sh
```

The docs gate checks Rust API docs, project-structure metadata, Markdown style, and links.

## Repository map

```text
crates/              compiler crates
docs/adr/            architecture decision records
docs/semantics/      semantic evidence records
docs/architecture/   compiler architecture documentation
tests/               future conformance and differential suites
tools/xtask/         repository validation tooling
scripts/              local development gates
```

Read [AGENTS.md](AGENTS.md), [CONTRIBUTING.md](CONTRIBUTING.md), and
[Compiler invariants](docs/COMPILER_INVARIANTS.md) before changing compiler behavior.

## License

No open-source license has been selected yet. Licensing is intentionally tracked as a Phase 0
project decision rather than being silently chosen by bootstrap tooling.

## Repository workflow

Ferraxis uses plan-first compiler development.

```bash
./scripts/setup-dev
./scripts/project-status
./scripts/plan status
./scripts/gate.sh fast
./scripts/gate.sh full
```

See [Project specification](PROJECT_SPEC.md), [Project state](PROJECT_STATE.md),
[Agent handoff](AGENT_HANDOFF.md), and
[Development workflow](docs/DEVELOPMENT_WORKFLOW.md).
