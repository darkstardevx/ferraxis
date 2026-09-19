# Differential Tests

Differential tests are compatibility evidence. Each observation must capture the compared compiler,
version, target, source input, and property being compared.

## P0-M018 lexer observation

The first executable differential harness lives in the non-publishable `ferraxis-diff` workspace
tool.

Run it with:

```bash
cargo run -p ferraxis-diff --locked -- lexer
```

Evidence is written below `target/differential/lexer/`.

The rustc side is a stable macro token-tree acceptance probe. It is not a raw rustc lexer dump and
must not be described as token-for-token equivalence. See
`docs/adr/ADR-0012-stable-rustc-token-tree-observation.md`.
