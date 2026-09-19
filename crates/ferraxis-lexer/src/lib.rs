//! The Ferraxis lexer.
//!
//! The current slice recognizes ASCII whitespace, ordinary non-documentation line comments,
//! the `fn` keyword, an ASCII identifier subset, and EOF. Unsupported source produces
//! structured lexical errors instead of being guessed.

use ferraxis_source::SourceFile;
use ferraxis_span::{BytePos, Span};

/// One lexical token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// Token classification.
    pub kind: TokenKind,
    /// Half-open source span.
    pub span: Span,
}

/// Token kinds implemented by the current lexer slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// The exact strict keyword `fn`.
    Fn,
    /// An identifier in the currently supported ASCII subset.
    Identifier,
    /// Explicit end-of-file marker used internally by Ferraxis.
    Eof,
}

/// A lexical error produced for source outside the supported lexer slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexError {
    /// Span covering the unsupported byte.
    pub span: Span,
    /// Error classification.
    pub kind: LexErrorKind,
}

/// Kinds of lexical failure in the currently supported slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexErrorKind {
    /// An unsupported source byte was encountered.
    UnexpectedByte(u8),
    /// A source file exceeds the current `u32` position representation.
    SourceTooLarge,
}

/// Tokenizes one source file using the currently supported lexical subset.
///
/// Ordinary non-documentation line comments are treated as whitespace. Line documentation
/// comments remain unsupported until their attribute semantics are implemented.
///
/// # Errors
///
/// Returns [`LexError`] for unsupported source bytes or files too large for the current byte
/// position representation.
pub fn lex(source: &SourceFile) -> Result<Vec<Token>, LexError> {
    let bytes = source.text().as_bytes();
    let source_len = source.byte_len().ok_or(LexError {
        span: Span::empty(BytePos::new(u32::MAX)),
        kind: LexErrorKind::SourceTooLarge,
    })?;

    let mut tokens = Vec::new();
    let mut cursor = 0usize;

    while cursor < bytes.len() {
        if is_ascii_whitespace(bytes[cursor]) {
            cursor += 1;
            continue;
        }

        if is_non_doc_line_comment_start(bytes, cursor) {
            cursor += 2;
            while cursor < bytes.len() && bytes[cursor] != b'\n' {
                cursor += 1;
            }
            continue;
        }

        if is_identifier_start(bytes[cursor]) {
            let start = cursor;
            cursor += 1;
            while cursor < bytes.len() && is_identifier_continue(bytes[cursor]) {
                cursor += 1;
            }

            if bytes[start] == b'_' && cursor == start + 1 {
                return Err(unexpected(bytes[start], start));
            }

            let span = span(start, cursor);
            let kind = if &source.text()[start..cursor] == "fn" {
                TokenKind::Fn
            } else {
                TokenKind::Identifier
            };
            tokens.push(Token { kind, span });
            continue;
        }

        return Err(unexpected(bytes[cursor], cursor));
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span::empty(BytePos::new(source_len)),
    });
    Ok(tokens)
}

fn is_ascii_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

fn is_non_doc_line_comment_start(bytes: &[u8], cursor: usize) -> bool {
    if bytes.get(cursor) != Some(&b'/') || bytes.get(cursor + 1) != Some(&b'/') {
        return false;
    }

    match bytes.get(cursor + 2) {
        Some(&b'!') => false,
        Some(&b'/') => bytes.get(cursor + 3) == Some(&b'/'),
        _ => true,
    }
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_continue(byte: u8) -> bool {
    is_identifier_start(byte) || byte.is_ascii_digit()
}

fn span(start: usize, end: usize) -> Span {
    Span::new(
        BytePos::new(u32::try_from(start).expect("source length checked before lexing")),
        BytePos::new(u32::try_from(end).expect("source length checked before lexing")),
    )
}

fn unexpected(byte: u8, offset: usize) -> LexError {
    let start = u32::try_from(offset).expect("source length checked before lexing");
    let end = start + 1;
    LexError {
        span: Span::new(BytePos::new(start), BytePos::new(end)),
        kind: LexErrorKind::UnexpectedByte(byte),
    }
}

#[cfg(test)]
mod tests {
    use ferraxis_source::SourceFile;

    use super::{LexErrorKind, TokenKind, lex};

    fn kinds(input: &str) -> Vec<TokenKind> {
        lex(&SourceFile::new("test.rs", input))
            .expect("test input should lex")
            .into_iter()
            .map(|token| token.kind)
            .collect()
    }

    #[test]
    fn empty_input_emits_eof() {
        assert_eq!(kinds(""), vec![TokenKind::Eof]);
    }

    #[test]
    fn recognizes_fn() {
        assert_eq!(kinds("fn"), vec![TokenKind::Fn, TokenKind::Eof]);
    }

    #[test]
    fn recognizes_identifier() {
        assert_eq!(kinds("main"), vec![TokenKind::Identifier, TokenKind::Eof]);
    }

    #[test]
    fn keyword_boundary_is_exact() {
        assert_eq!(
            kinds("fnn fn1 fn"),
            vec![
                TokenKind::Identifier,
                TokenKind::Identifier,
                TokenKind::Fn,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn accepts_initial_ascii_identifier_subset() {
        assert_eq!(
            kinds("_main a1 Z9"),
            vec![
                TokenKind::Identifier,
                TokenKind::Identifier,
                TokenKind::Identifier,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn skips_ascii_whitespace() {
        assert_eq!(
            kinds(" \t\n\r\u{000b}\u{000c}fn\nmain"),
            vec![TokenKind::Fn, TokenKind::Identifier, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_empty_line_comment() {
        assert_eq!(kinds("//\nfn"), vec![TokenKind::Fn, TokenKind::Eof]);
    }

    #[test]
    fn skips_eof_terminated_line_comment() {
        let source = SourceFile::new("test.rs", "// comment");
        let tokens = lex(&source).expect("EOF-terminated line comment should lex");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Eof);
        assert_eq!(tokens[0].span.lo().get(), 10);
        assert_eq!(tokens[0].span.hi().get(), 10);
    }

    #[test]
    fn skips_inline_line_comment() {
        assert_eq!(
            kinds("fn// comment\nmain"),
            vec![TokenKind::Fn, TokenKind::Identifier, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_four_slash_line_comment() {
        assert_eq!(
            kinds("//// comment\nfn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_unicode_line_comment_body() {
        assert_eq!(kinds("// café\nfn"), vec![TokenKind::Fn, TokenKind::Eof]);
    }

    #[test]
    fn line_comment_continues_through_cr_before_lf() {
        assert_eq!(
            kinds("// comment\r\nfn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn bare_cr_does_not_end_line_comment() {
        assert_eq!(kinds("// a\rb\nfn"), vec![TokenKind::Fn, TokenKind::Eof]);
    }

    #[test]
    fn rejects_outer_line_doc_comment_for_now() {
        let error =
            lex(&SourceFile::new("test.rs", "/// docs\nfn")).expect_err("doc comment unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'/'));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn rejects_inner_line_doc_comment_for_now() {
        let error =
            lex(&SourceFile::new("test.rs", "//! docs\nfn")).expect_err("doc comment unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'/'));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn preserves_token_spans() {
        let source = SourceFile::new("test.rs", "fn main");
        let tokens = lex(&source).expect("input should lex");
        assert_eq!(tokens[0].span.lo().get(), 0);
        assert_eq!(tokens[0].span.hi().get(), 2);
        assert_eq!(tokens[1].span.lo().get(), 3);
        assert_eq!(tokens[1].span.hi().get(), 7);
        assert_eq!(tokens[2].span.lo().get(), 7);
        assert_eq!(tokens[2].span.hi().get(), 7);
    }

    #[test]
    fn preserves_token_spans_after_line_comment() {
        let source = SourceFile::new("test.rs", "// x\nfn");
        let tokens = lex(&source).expect("line comment followed by token should lex");
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[0].span.lo().get(), 5);
        assert_eq!(tokens[0].span.hi().get(), 7);
        assert_eq!(tokens[1].kind, TokenKind::Eof);
        assert_eq!(tokens[1].span.lo().get(), 7);
        assert_eq!(tokens[1].span.hi().get(), 7);
    }

    #[test]
    fn rejects_bare_underscore_for_now() {
        let error = lex(&SourceFile::new("test.rs", "_")).expect_err("underscore is unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'_'));
    }

    #[test]
    fn rejects_unsupported_byte() {
        let error = lex(&SourceFile::new("test.rs", "(")).expect_err("punctuation is unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'('));
    }
}
