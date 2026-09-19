# Plan: P1-M004 — Punctuation

Status: Complete
Milestone: P1-M004
Created: 2026-09-18

## Goal

Implement the non-delimiter subset of the Rust Reference `PUNCTUATION` lexer production with
explicit token identity, longest-match behavior, stable byte spans, CLI token-dump support, and
differential evidence.

P1-M004 adds 46 punctuation spellings:

```text
...
..=
<<=
>>=
!=
%=
&&
&=
*=
+=
-=
->
..
/=
::
<-
<<
<=
==
=>
>=
>>
^=
|=
||
!
#
$
%
&
*
+
,
-
.
/
:
;
<
=
>
?
@
^
|
~
```

The six bracket characters `( ) [ ] { }` remain P1-M005 because the permanent milestone registry
separates delimiter pairing and token-tree structure from other punctuation.

## Non-goals

- No delimiter tokens; P1-M005 owns `( ) [ ] { }`.
- No bare underscore token; `_` remains part of the strict-keyword milestone.
- No lifetime or single-quote tokenization; P1-M012 owns lifetimes.
- No raw identifiers; P1-M011 owns `r#ident`.
- No integer, float, string, character, byte, raw-string, or C-string literal support.
- No parser precedence or operator semantics.
- No expression parsing.
- No macro expansion.
- No backend changes.
- No dependency changes.
- No attempt to reproduce proc-macro punctuation splitting; P1-M004 follows the Reference lexer
  token production, not the proc-macro API conversion model.

## Context

The Rust Reference identifies keywords, identifiers, literals, lifetimes, punctuation, and
delimiters as token categories.

The current Reference `PUNCTUATION` production includes multi-character and single-character
punctuation, including the six bracket characters. Ferraxis already has a permanent P1-M005
delimiter milestone, so P1-M004 intentionally implements every Reference punctuation spelling
except those six brackets.

The grammar uses ordered alternatives, so overlapping punctuation must be recognized with
longest/specific forms before shorter prefixes. Examples include:

- `...` before `..` and `.`;
- `..=` before `..` and `.`;
- `<<=` before `<<`, `<=`, and `<`;
- `>>=` before `>>`, `>=`, and `>`;
- `::` before `:`;
- `->` before `-`;
- `=>` before `=`;
- `&&` before `&`;
- `||` before `|`.

Comments have lexical priority over slash punctuation: `//...` and `/*...*/` remain comments,
while `/` and `/=` are punctuation.

Adding slash/star/pound punctuation creates compatibility hazards that P1-M004 must explicitly
guard:

1. line and block documentation-comment prefixes must remain unsupported rather than becoming
   sequences of `/`, `*`, and `!` punctuation;
2. Rust 2024 reserves `##` and longer pound runs;
3. Rust 2021+ reserves many identifier-adjacent prefixes such as `name#foo`;
4. raw identifiers such as `r#name` are valid Rust but remain unsupported until P1-M011 and must
   not be silently split into identifier + pound + identifier.

Primary references:

- <https://doc.rust-lang.org/reference/tokens.html>
- <https://doc.rust-lang.org/reference/notation.html>
- <https://doc.rust-lang.org/reference/input-format.html>
- <https://doc.rust-lang.org/reference/identifiers.html>
- <https://doc.rust-lang.org/reference/keywords.html>

## Architecture placement

P1-M004 expands token representation and lexer recognition only.

```text
SourceFile UTF-8 bytes
        |
        v
ferraxis-lexer
  whitespace
  comments
  identifiers / fn
  punctuation  <--- P1-M004
  unsupported future token families
        |
        v
Token { kind, byte span }
        |
        v
ferraxis --emit=tokens
```

The frontend remains backend-neutral.

## Data flow

1. Preserve existing whitespace and comment recognition.
2. If a slash-prefixed sequence was not consumed as an ordinary comment but still begins `//` or
   `/*`, reject it as an unsupported documentation-comment form before punctuation recognition.
3. Scan identifiers using the existing subset.
4. If a recognized identifier is immediately followed by `#`, reject at the pound byte so raw or
   reserved-prefix families are not silently split.
5. Before ordinary pound punctuation recognition, reject `##` and longer pound runs.
6. Match punctuation from longest spellings to shortest.
7. Emit `TokenKind::Punctuation(Punctuation)` with the exact half-open byte span.
8. Continue tokenization immediately after the matched spelling.
9. Leave delimiters, lifetimes, literals, bare underscore, Unicode identifiers, and other future
   token families controlled failures.
10. Emit exactly one zero-width EOF token.

## Token representation

Add a public `Punctuation` enum with one variant per P1-M004 spelling and a stable `as_str()`
mapping.

Add:

```rust
TokenKind::Punctuation(Punctuation)
```

This keeps operator/separator identity in the token stream rather than forcing later parser phases
to re-slice source text.

The six delimiters will get their own representation in P1-M005 rather than being shoehorned into
`Punctuation` now.

## Invariants

- INV-004 — backend-specific types do not leak into frontend token representation.
- INV-005 — source positions remain UTF-8 byte offsets.
- INV-013 — semantic disagreements require conformance evidence.
- INV-014 — temporary implementation subsets are documented.
- INV-015 — compatibility claims require reproducible evidence.
- INV-016 — user input never causes compiler panic.
- INV-017 — invalid or unsupported Rust fails in a controlled way.
- INV-018 — new behavior has regression coverage.
- INV-019 — lexing remains deterministic.
- INV-021 — behavior documentation remains CI-validated.
- INV-022 — milestone and semantic IDs are permanent.

## ADRs

- ADR-0001 — semantic authority hierarchy.
- ADR-0003 — byte-offset spans.
- ADR-0006 — differential testing against rustc.
- ADR-0011 — reproducible toolchain validation.
- ADR-0012 — stable rustc token-tree observation boundary.

No new architecture ADR is required. The public token representation grows within the existing
lexer ownership boundary.

## Semantic evidence

Create:

- `SEM-LEX-0007` — punctuation tokenization and longest-match boundaries.

The semantic record must explicitly document:

- all 46 P1-M004 spellings;
- the P1-M005 delimiter split;
- comment precedence over slash/star punctuation;
- doc-comment preservation;
- Rust 2024 reserved-pound preservation;
- identifier-adjacent pound preservation;
- raw-identifier deferral to P1-M011;
- the difference between Reference lexer punctuation and proc-macro punctuation conversion.

## Public API / CLI

Public lexer API:

- add `Punctuation`;
- add `TokenKind::Punctuation(Punctuation)`;
- add `Punctuation::as_str()`.

Token-dump CLI:

- emit punctuation token spans and exact spellings;
- keep existing `Fn`, `Identifier`, and `Eof` output stable.

No parser API is introduced.

## Compatibility analysis

### Reference punctuation

P1-M004 implements all non-delimiter alternatives currently listed in the Reference
`PUNCTUATION` production.

### Ordered / longest matching

Recognition must prefer longer valid punctuation spellings over shorter prefixes.

### Comment precedence

`//` and `/*` are comments, not pairs of slash/star punctuation tokens. Existing nested-comment
behavior must remain unchanged.

Documentation comments remain unsupported at top level and must continue to fail at the first
slash instead of becoming punctuation sequences.

### Reserved pounds and prefixes

Rust 2024 reserves two-or-more pound runs. P1-M004 must reject `##` and longer runs.

Rust 2021+ reserves many identifier/keyword prefixes immediately followed by `#`. Ferraxis's
current identifier subset must not accidentally accept such source as separate tokens.

Raw identifiers like `r#name` are valid Rust but remain a known `ferraxis_rejects` gap assigned to
P1-M011.

### Delimiters

Matched delimiters are valid Rust token trees but remain a temporary `ferraxis_rejects` gap until
P1-M005.

### Proc-macro distinction

The procedural-macro API exposes operator punctuation as individual characters and converts
multi-character operators when crossing the proc-macro boundary. That API conversion is not the
Reference lexer token model and does not change P1-M004 representation.

## Dependency analysis

No dependency is required.

Implementation uses static punctuation tables / byte-prefix checks and existing span types.

## Expected file boundary

Plan checkpoint:

- `.plans/ACTIVE`
- `.plans/P1-M004-punctuation.plan.md`
- `docs/MILESTONES.md`
- `docs/semantics/README.md`
- `docs/semantics/SEM-LEX-0007-punctuation.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Implementation:

- `crates/ferraxis-lexer/src/lib.rs`
- `crates/ferraxis/src/main.rs`
- `crates/ferraxis/tests/token_dump.rs`
- `tests/differential/lexer/cases/`
- `tests/differential/lexer/expectations.tsv`
- `docs/semantics/SEM-LEX-0007-punctuation.md`
- `docs/semantics/README.md`
- `PROJECT_STATE.md`
- `AGENT_HANDOFF.md`

Closure may additionally update:

- this plan;
- `docs/MILESTONES.md`;
- `.plans/ACTIVE`.

No Cargo manifest, lockfile, or dependency change is expected.

## Test-first matrix

| Behavior | Test | Expected result |
| --- | --- | --- |
| All 21 single-char non-delimiter punctuations | table-driven unit test | exact variants/spans |
| All 21 two-char punctuations | table-driven unit test | exact variants/spans |
| All 4 three-char punctuations | table-driven unit test | exact variants/spans |
| `...` vs `..` vs `.` | longest-match unit test | Ellipsis, DotDot, Dot |
| `..=` vs `..` | longest-match unit test | DotDotEq wins |
| `<<=` / `<<` / `<=` / `<-` / `<` | boundary unit test | exact variants |
| `>>=` / `>>` / `>=` / `>` | boundary unit test | exact variants |
| `::` / `:` | boundary unit test | exact variants |
| `->` / `-` | boundary unit test | exact variants |
| `=>` / `==` / `=` | boundary unit test | exact variants |
| `&&` / `&=` / `&` | boundary unit test | exact variants |
| double-pipe / pipe-equals / pipe | boundary unit test | exact variants |
| `// comment` | regression | still a comment |
| `/* nested /* x */ */` | regression | still a nested comment |
| `/// docs` | regression | still controlled unsupported failure |
| `//! docs` | regression | still controlled unsupported failure |
| `/** docs */` | regression | still controlled unsupported failure |
| `/*! docs */` | regression | still controlled unsupported failure |
| `##` | unit + differential | controlled reject / `agree_reject` |
| `name#foo` | unit + differential | controlled reject / `agree_reject` |
| `r#name` | unit + differential | controlled reject / `ferraxis_rejects` |
| `()` | unit + differential | controlled reject / `ferraxis_rejects` until P1-M005 |
| punctuation after identifier | `fn->main` | exact token order/spans |
| punctuation token dump | CLI integration | stable spelling/span output |
| differential single-char group | corpus | `agree_accept` |
| differential double-char group | corpus | `agree_accept` |
| differential triple-char group | corpus | `agree_accept` |
| differential comment mix | corpus | `agree_accept` |

## Implementation sequence

1. Commit the Approved plan, Planned SEM-LEX-0007, milestone activation, and state/handoff records
   without Rust or Cargo changes.
2. Require exact plan-only CI success.
3. Add `Punctuation` and `TokenKind::Punctuation` with full public documentation.
4. Add a longest-first punctuation recognition table/helper.
5. Preserve comment precedence.
6. Add explicit fallback rejection for unsupported doc-comment prefixes before slash punctuation.
7. Add reserved-pound rejection for `##` and longer.
8. Reject identifier-adjacent `#` before emitting the identifier token.
9. Keep delimiters, underscore, lifetime/single-quote, raw identifiers, literals, and Unicode
   identifiers outside this milestone.
10. Update token-dump rendering and integration coverage.
11. Add full punctuation unit coverage and maximal-match tests.
12. Add differential punctuation/boundary cases and expectations.
13. Require exact implementation CI success.
14. Inspect uploaded differential evidence.
15. Record exact implementation commit, run, artifact ID, digest, rustc identity, and results.
16. Close P1-M004 and remove `.plans/ACTIVE`.
17. Require exact closed-state CI success.
18. Merge with history preservation.
19. Require post-merge `main` CI success.

## Failure modes

- Matching `..` before `...` or `..=`.
- Matching `<<` before `<<=`, or analogous shift/assignment cases.
- Turning comments into slash/star punctuation.
- Turning unsupported doc comments into successful punctuation streams.
- Accepting Rust-2024 `##` as two pound tokens.
- Accepting reserved `name#foo` as identifier + pound + identifier.
- Silently accepting `r#name` as three tokens rather than keeping the P1-M011 gap visible.
- Accidentally implementing delimiters early.
- Treating bare `_` as punctuation.
- Treating `'` as punctuation despite lifetime ownership.
- Losing exact punctuation identity and forcing parser re-slicing.
- Changing existing CLI output for previously supported tokens.
- Confusing Reference lexer tokens with proc-macro punctuation conversion.
- Changing comment/nesting behavior from P1-M001 through P1-M003.

## Documentation impact

- Add `SEM-LEX-0007` as Planned at the plan checkpoint.
- Mark it Active only after implementation validation.
- Document the complete 46-spelling subset.
- Document temporary gaps owned by P1-M005, P1-M011, P1-M012, and literal milestones.
- Update project state and handoff at implementation and closure.

## Quality gates

- `./scripts/gate.sh precommit`
- `./scripts/gate.sh fast`
- `./scripts/gate.sh full`
- `cargo run -p ferraxis-diff --locked -- lexer`
- exact GitHub Actions success for the implementation commit
- differential artifact inspection
- exact GitHub Actions success for closed state
- successful post-merge `main` CI

## Acceptance criteria

- [x] All 46 non-delimiter Reference punctuation spellings have explicit token identities.
- [x] `TokenKind::Punctuation(Punctuation)` is implemented.
- [x] `Punctuation::as_str()` maps every variant exactly.
- [x] Longest-match behavior is deterministic and regression tested.
- [x] Comment recognition retains priority over punctuation.
- [x] Existing nested block comments remain green.
- [x] Top-level line doc comments remain unsupported.
- [x] Top-level block doc comments remain unsupported.
- [x] `##` and longer pound runs remain rejected in edition-2024 behavior.
- [x] Identifier-adjacent pound reserved prefixes remain rejected.
- [x] `r#name` remains an explicit P1-M011 gap rather than being split.
- [x] Delimiters remain an explicit P1-M005 gap.
- [x] Bare underscore remains outside P1-M004.
- [x] Lifetimes/single quote remain outside P1-M004.
- [x] Existing identifier and `fn` behavior remains green.
- [x] Token spans are exact byte ranges.
- [x] Token-dump CLI renders punctuation deterministically.
- [x] Differential punctuation groups classify `agree_accept`.
- [x] Reserved-pound and reserved-prefix cases classify `agree_reject`.
- [x] Raw-identifier and matched-delimiter cases classify `ferraxis_rejects`.
- [x] No dependency is added.
- [x] SEM-LEX-0007 is current.
- [x] Exact implementation head passes all CI jobs.
- [x] Differential artifact is inspected before closure.
- [x] Exact closed-state head passes all CI jobs.
- [x] `PROJECT_STATE.md` is current.
- [x] `AGENT_HANDOFF.md` is current.

## Completion record

Implementation commit: 7baa2974e1fbc109399ccc91e94d9802489d6b7e
CI run: <https://github.com/darkstardevx/ferraxis/actions/runs/35420906959>
CI result: success
Completed: 2026-09-18
Notes: P1-M004 completed after implementation repair, exact CI validation, and differential artifact inspection.

Evidence:

- Initial Approved plan commit: `e81717b3849b43970e94a9a779430d9f24f60ac7`.
- Initial plan CI run 35420510194 failed only on Markdown table lint.
- Final Approved plan checkpoint: `d7070206d1cbb8cff04edea49ca688f653259060`.
- Final Approved plan checkpoint CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35420536691>.
- Initial semantic implementation commit: `fce2ce1055c2b7a61d44f9729e78ebea5975fc53`.
- Implementation CI run 35420746526 exposed a generated-source assembly defect and rustfmt drift.
- Repair commit: `16f5ed90658808ed66fc782aadf4ef1636c8f9f5`.
- Run 35420884872 was superseded by the final rustfmt-only repair and cancelled.
- Final validated implementation head: `7baa2974e1fbc109399ccc91e94d9802489d6b7e`.
- Exact successful implementation CI:
  <https://github.com/darkstardevx/ferraxis/actions/runs/35420906959>.
- Differential artifact: `p0-m018-differential-lexer`, artifact ID `10577067943`.
- Artifact digest:
  `sha256:7f6ec43b847a84d6cbe72d8f2b1eab32d66b01239c119d6915a8a288b053ae7a`.
- CI rustc identity: `rustc 1.98.1 (48a229cea 2026-09-01)`,
  host `x86_64-unknown-linux-gnu`.
- Differential classifications matched: 33/33.
- `punct-all` and `punct-comment-mix` classified `agree_accept`.
- `reserved-pounds` and `reserved-pound-prefix` classified `agree_reject`.
- `raw-identifier` remains `ferraxis_rejects` until P1-M011.
- `paired-delimiters` remains `ferraxis_rejects` until P1-M005.
- Existing lexer gaps for bare underscore, integer literals, and Unicode identifiers remained unchanged.
- Stable code, feature isolation, MSRV 1.85.0, repository workflow, differential evidence, and
  documentation validation all passed on the exact final implementation head.
- Manual representation audit confirmed 46 explicit variants, 46 exact source spellings, and
  longest-first 3-byte / 2-byte / 1-byte recognition order.
