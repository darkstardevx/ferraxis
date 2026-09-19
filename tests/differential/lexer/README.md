# Differential Lexer Corpus

This directory contains the P0-M018 versioned lexer-observation corpus.

## Case files

Each `cases/<id>.rsfrag` file contains the exact UTF-8 fragment presented to Ferraxis and inserted
inside the generated stable-rustc token-tree probe.

Cases execute in deterministic case-ID order.

## Expectations

`expectations.tsv` commits one expected classification per case:

- `agree_accept`
- `agree_reject`
- `ferraxis_rejects`
- `rustc_rejects`

These describe the pair of observations only. They are not correctness verdicts.

## Evidence

A run writes rustc identity, run metadata, TSV and Markdown summaries, and per-case generated probe,
Ferraxis observation, rustc stdout, and rustc stderr below `target/differential/lexer/`.

Generated evidence is intentionally untracked and uploaded by CI for inspection.
