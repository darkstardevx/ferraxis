# Semantic Evidence Records

Semantic evidence records connect language behavior to authoritative sources, observations, tests,
and intentional variances.

## Identifier format

Records use permanent IDs grouped by domain:

```text
SEM-LEX-NNNN   lexing
SEM-PAR-NNNN   parsing
SEM-RES-NNNN   resolution
SEM-TYP-NNNN   type system
SEM-TRT-NNNN   traits
SEM-BOR-NNNN   borrowing and regions
SEM-MIR-NNNN   MIR semantics
SEM-ABI-NNNN   ABI and layout
```

IDs are never recycled.

## Required sections

Every semantic evidence record contains:

- Topic;
- Status;
- Primary authority;
- Sources;
- Ferraxis behavior;
- Tests;
- Variance.

A record may begin before implementation. Use `Status: Planned` and state clearly when behavior is
not yet implemented.

## Registry

| ID | Status | Topic |
| --- | --- | --- |
| SEM-LEX-0001 | Active | Initial identifier lexical grammar |
| SEM-LEX-0002 | Active | Exact `fn` keyword boundary |
| SEM-LEX-0003 | Active | EOF token and zero-width terminal span |
| SEM-LEX-0004 | Active | Non-doc line comments |
| SEM-LEX-0005 | Active | Non-doc block comments |
| SEM-LEX-0006 | Active | Recursive nested block comments |
| SEM-LEX-0007 | Active | Punctuation tokenization |
| SEM-LEX-0008 | Planned | Delimiter tokenization |
