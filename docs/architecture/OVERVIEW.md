# Architecture Overview

Ferraxis is designed as an independent compiler pipeline with replaceable code-generation
backends.

```text
Rust source
    |
    v
source + spans
    |
    v
lexer -> parser -> AST -> expansion/resolution -> HIR
                                           |
                                           v
                               types / traits / coherence
                                           |
                                           v
                                      typed HIR
                                           |
                                           v
                                          MIR
                                           |
                                  borrow / region analysis
                                           |
                                           v
                                      Ferraxis IR
                                           |
                     +---------------------+---------------------+
                     |                     |                     |
                     v                     v                     v
                native backend       C bring-up backend    future backends
```

Phase 0 intentionally implements only source storage, spans, diagnostic data, the first lexer
slice, and repository validation machinery.
