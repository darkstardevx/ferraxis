# SEM-LEX-0006: Recursive nested block comments

## Topic

Recursive nesting of block-comment forms inside an ordinary Rust block comment.

## Status

Planned

## Primary authority

L1 — explicit Rust language definition/specification.

## Sources

- Rust Reference comments grammar: <https://doc.rust-lang.org/reference/comments.html>
- Rust Reference grammar notation: <https://doc.rust-lang.org/reference/notation.html>
- Rust Reference input format:
  <https://doc.rust-lang.org/reference/input-format.html>

## Ferraxis behavior

P1-M003 will complete recursive block-comment nesting for top-level ordinary non-doc block
comments.

The scanner will use iterative depth tracking:

- top-level ordinary `/*` starts depth one;
- every nested `/*` increments depth;
- every `*/` decrements depth;
- scanning ends only when depth returns to zero;
- EOF with nonzero depth is a controlled lexical failure anchored at the original outer opener.

Nested openers are counted regardless of whether their local spelling is ordinary, outer-doc, or
inner-doc syntax. Inside an already-open ordinary block comment, those forms participate in comment
nesting and do not become documentation attributes.

Top-level outer and inner block documentation comments remain unsupported.

## Resource behavior

No arbitrary nesting limit is introduced.

Depth is tracked with `usize` and is bounded by the number of two-byte openers present in the source.
The existing Ferraxis source-length constraint bounds the source to representable byte positions.
The scanner is iterative, linear in source length, and constant in auxiliary memory.

## Tests

Planned coverage:

- unit tests in `crates/ferraxis-lexer/src/lib.rs`;
- versioned differential cases under `tests/differential/lexer/`.

Coverage includes one-level and deeper nesting, nested outer/inner doc forms, mixed forms,
unterminated nesting, span preservation, UTF-8 bodies, CR, and line-marker text.

## Variance

After P1-M003, recursive nesting is no longer a Ferraxis variance for ordinary top-level block
comments.

Remaining unsupported comment subsets are top-level line and block documentation comments and
their attribute semantics.
