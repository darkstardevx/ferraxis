//! The Ferraxis lexer.
//!
//! The current slice recognizes ASCII whitespace, ordinary non-documentation line comments,
//! recursively nested ordinary non-documentation block comments, the `fn` keyword, an ASCII
//! identifier subset, non-delimiter punctuation, and EOF. Unsupported source produces structured
//! lexical errors instead of being guessed.

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
    /// One non-delimiter punctuation token.
    Punctuation(Punctuation),
    /// Explicit end-of-file marker used internally by Ferraxis.
    Eof,
}

/// One non-delimiter punctuation spelling recognized by the P1-M004 lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punctuation {
    /// The `...` punctuation token.
    Ellipsis,
    /// The `..=` punctuation token.
    DotDotEq,
    /// The `<<=` punctuation token.
    ShlEq,
    /// The `>>=` punctuation token.
    ShrEq,
    /// The `!=` punctuation token.
    BangEq,
    /// The `%=` punctuation token.
    PercentEq,
    /// The `&&` punctuation token.
    AmpAmp,
    /// The `&=` punctuation token.
    AmpEq,
    /// The `*=` punctuation token.
    StarEq,
    /// The `+=` punctuation token.
    PlusEq,
    /// The `-=` punctuation token.
    MinusEq,
    /// The `->` punctuation token.
    ThinArrow,
    /// The `..` punctuation token.
    DotDot,
    /// The `/=` punctuation token.
    SlashEq,
    /// The `::` punctuation token.
    ColonColon,
    /// The `<-` punctuation token.
    LeftArrow,
    /// The `<<` punctuation token.
    Shl,
    /// The `<=` punctuation token.
    LessEq,
    /// The `==` punctuation token.
    EqEq,
    /// The `=>` punctuation token.
    FatArrow,
    /// The `>=` punctuation token.
    GreaterEq,
    /// The `>>` punctuation token.
    Shr,
    /// The `^=` punctuation token.
    CaretEq,
    /// The `|=` punctuation token.
    PipeEq,
    /// The `||` punctuation token.
    PipePipe,
    /// The `!` punctuation token.
    Bang,
    /// The `#` punctuation token.
    Pound,
    /// The dollar-sign punctuation token.
    Dollar,
    /// The `%` punctuation token.
    Percent,
    /// The `&` punctuation token.
    Amp,
    /// The `*` punctuation token.
    Star,
    /// The `+` punctuation token.
    Plus,
    /// The `,` punctuation token.
    Comma,
    /// The `-` punctuation token.
    Minus,
    /// The `.` punctuation token.
    Dot,
    /// The `/` punctuation token.
    Slash,
    /// The `:` punctuation token.
    Colon,
    /// The `;` punctuation token.
    Semicolon,
    /// The `<` punctuation token.
    Less,
    /// The `=` punctuation token.
    Eq,
    /// The `>` punctuation token.
    Greater,
    /// The `?` punctuation token.
    Question,
    /// The `@` punctuation token.
    At,
    /// The `^` punctuation token.
    Caret,
    /// The `|` punctuation token.
    Pipe,
    /// The `~` punctuation token.
    Tilde,
}

impl Punctuation {
    /// Returns the exact source spelling for this punctuation token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ellipsis => "...",
            Self::DotDotEq => "..=",
            Self::ShlEq => "<<=",
            Self::ShrEq => ">>=",
            Self::BangEq => "!=",
            Self::PercentEq => "%=",
            Self::AmpAmp => "&&",
            Self::AmpEq => "&=",
            Self::StarEq => "*=",
            Self::PlusEq => "+=",
            Self::MinusEq => "-=",
            Self::ThinArrow => "->",
            Self::DotDot => "..",
            Self::SlashEq => "/=",
            Self::ColonColon => "::",
            Self::LeftArrow => "<-",
            Self::Shl => "<<",
            Self::LessEq => "<=",
            Self::EqEq => "==",
            Self::FatArrow => "=>",
            Self::GreaterEq => ">=",
            Self::Shr => ">>",
            Self::CaretEq => "^=",
            Self::PipeEq => "|=",
            Self::PipePipe => "||",
            Self::Bang => "!",
            Self::Pound => "#",
            Self::Dollar => "$",
            Self::Percent => "%",
            Self::Amp => "&",
            Self::Star => "*",
            Self::Plus => "+",
            Self::Comma => ",",
            Self::Minus => "-",
            Self::Dot => ".",
            Self::Slash => "/",
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::Less => "<",
            Self::Eq => "=",
            Self::Greater => ">",
            Self::Question => "?",
            Self::At => "@",
            Self::Caret => "^",
            Self::Pipe => "|",
            Self::Tilde => "~",
        }
    }
}

const PUNCTUATION_SPELLINGS: &[(&str, Punctuation)] = &[
    ("...", Punctuation::Ellipsis),
    ("..=", Punctuation::DotDotEq),
    ("<<=", Punctuation::ShlEq),
    (">>=", Punctuation::ShrEq),
    ("!=", Punctuation::BangEq),
    ("%=", Punctuation::PercentEq),
    ("&&", Punctuation::AmpAmp),
    ("&=", Punctuation::AmpEq),
    ("*=", Punctuation::StarEq),
    ("+=", Punctuation::PlusEq),
    ("-=", Punctuation::MinusEq),
    ("->", Punctuation::ThinArrow),
    ("..", Punctuation::DotDot),
    ("/=", Punctuation::SlashEq),
    ("::", Punctuation::ColonColon),
    ("<-", Punctuation::LeftArrow),
    ("<<", Punctuation::Shl),
    ("<=", Punctuation::LessEq),
    ("==", Punctuation::EqEq),
    ("=>", Punctuation::FatArrow),
    (">=", Punctuation::GreaterEq),
    (">>", Punctuation::Shr),
    ("^=", Punctuation::CaretEq),
    ("|=", Punctuation::PipeEq),
    ("||", Punctuation::PipePipe),
    ("!", Punctuation::Bang),
    ("#", Punctuation::Pound),
    ("$", Punctuation::Dollar),
    ("%", Punctuation::Percent),
    ("&", Punctuation::Amp),
    ("*", Punctuation::Star),
    ("+", Punctuation::Plus),
    (",", Punctuation::Comma),
    ("-", Punctuation::Minus),
    (".", Punctuation::Dot),
    ("/", Punctuation::Slash),
    (":", Punctuation::Colon),
    (";", Punctuation::Semicolon),
    ("<", Punctuation::Less),
    ("=", Punctuation::Eq),
    (">", Punctuation::Greater),
    ("?", Punctuation::Question),
    ("@", Punctuation::At),
    ("^", Punctuation::Caret),
    ("|", Punctuation::Pipe),
    ("~", Punctuation::Tilde),
];

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
/// Ordinary non-documentation line comments and recursively nested ordinary block comments are
/// treated as whitespace. Top-level documentation comments remain unsupported until their
/// attribute semantics are implemented.
///
/// # Errors
///
/// Returns [`LexError`] for unsupported source bytes, unterminated block comments, or files too
/// large for the current byte position representation.
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

        if is_non_doc_block_comment_start(bytes, cursor) {
            cursor = scan_block_comment(bytes, cursor)?;
            continue;
        }

        if is_comment_prefix(bytes, cursor) {
            return Err(unexpected(bytes[cursor], cursor));
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

            if bytes.get(cursor) == Some(&b'#') {
                return Err(unexpected(b'#', cursor));
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

        if bytes[cursor] == b'#' && bytes.get(cursor + 1) == Some(&b'#') {
            return Err(unexpected(bytes[cursor], cursor));
        }

        if let Some((punctuation, length)) = punctuation_at(bytes, cursor) {
            let start = cursor;
            cursor += length;
            tokens.push(Token {
                kind: TokenKind::Punctuation(punctuation),
                span: span(start, cursor),
            });
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

fn is_non_doc_block_comment_start(bytes: &[u8], cursor: usize) -> bool {
    if bytes.get(cursor) != Some(&b'/') || bytes.get(cursor + 1) != Some(&b'*') {
        return false;
    }

    match bytes.get(cursor + 2) {
        Some(&b'!') => false,
        Some(&b'*') => matches!(bytes.get(cursor + 3), Some(&b'*') | Some(&b'/')),
        _ => true,
    }
}

fn is_comment_prefix(bytes: &[u8], cursor: usize) -> bool {
    matches!(
        (bytes.get(cursor), bytes.get(cursor + 1)),
        (Some(&b'/'), Some(&b'/') | Some(&b'*'))
    )
}

fn punctuation_at(bytes: &[u8], cursor: usize) -> Option<(Punctuation, usize)> {
    let tail = &bytes[cursor..];

    PUNCTUATION_SPELLINGS
        .iter()
        .find_map(|&(spelling, punctuation)| {
            tail.starts_with(spelling.as_bytes())
                .then_some((punctuation, spelling.len()))
        })
}

fn scan_block_comment(bytes: &[u8], start: usize) -> Result<usize, LexError> {
    let mut cursor = start + 2;
    let mut depth = 1usize;

    while cursor < bytes.len() {
        if bytes.get(cursor) == Some(&b'/') && bytes.get(cursor + 1) == Some(&b'*') {
            depth += 1;
            cursor += 2;
            continue;
        }

        if bytes.get(cursor) == Some(&b'*') && bytes.get(cursor + 1) == Some(&b'/') {
            depth -= 1;
            cursor += 2;

            if depth == 0 {
                return Ok(cursor);
            }

            continue;
        }

        cursor += 1;
    }

    Err(unexpected(bytes[start], start))
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

    use super::{LexErrorKind, Punctuation, TokenKind, lex};

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
    fn skips_basic_block_comment() {
        assert_eq!(kinds("/* comment */"), vec![TokenKind::Eof]);
    }

    #[test]
    fn skips_inline_block_comment() {
        assert_eq!(
            kinds("fn/* comment */main"),
            vec![TokenKind::Fn, TokenKind::Identifier, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_multiline_block_comment() {
        assert_eq!(
            kinds("/* first\nsecond */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_empty_block_comment() {
        assert_eq!(kinds("/**/fn"), vec![TokenKind::Fn, TokenKind::Eof]);
    }

    #[test]
    fn skips_triple_star_block_comment() {
        assert_eq!(kinds("/***/fn"), vec![TokenKind::Fn, TokenKind::Eof]);
    }

    #[test]
    fn skips_longer_star_block_comment() {
        assert_eq!(
            kinds("/*** ordinary */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_unicode_block_comment_body() {
        assert_eq!(kinds("/* café */fn"), vec![TokenKind::Fn, TokenKind::Eof]);
    }

    #[test]
    fn line_comment_marker_is_text_inside_block_comment() {
        assert_eq!(
            kinds("/* // still block text */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn bare_cr_is_text_inside_block_comment() {
        assert_eq!(kinds("/* a\rb */fn"), vec![TokenKind::Fn, TokenKind::Eof]);
    }

    #[test]
    fn rejects_outer_block_doc_comment_for_now() {
        let error = lex(&SourceFile::new("test.rs", "/** docs */fn"))
            .expect_err("outer block doc comment unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'/'));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn rejects_inner_block_doc_comment_for_now() {
        let error = lex(&SourceFile::new("test.rs", "/*! docs */fn"))
            .expect_err("inner block doc comment unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'/'));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn rejects_inner_block_doc_comment_with_extra_bang_for_now() {
        let error = lex(&SourceFile::new("test.rs", "/*!! docs */fn"))
            .expect_err("inner block doc comment unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'/'));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn rejects_unterminated_block_comment() {
        let error = lex(&SourceFile::new("test.rs", "/* comment"))
            .expect_err("unterminated block comment should fail");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'/'));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn skips_nested_block_comment() {
        assert_eq!(
            kinds("/* outer /* inner */ outer */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_deeply_nested_block_comments() {
        assert_eq!(
            kinds("/* a /* b /* c */ b */ a */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_nested_empty_block_comment() {
        assert_eq!(
            kinds("/* a /**/ b */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_nested_triple_star_block_comment() {
        assert_eq!(
            kinds("/* a /***/ b */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_nested_outer_block_doc_form() {
        assert_eq!(
            kinds("/* outer /** docs */ outer */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_nested_inner_block_doc_form() {
        assert_eq!(
            kinds("/* outer /*! docs */ outer */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_mixed_nested_block_comment_forms() {
        assert_eq!(
            kinds("/* outer /* ordinary */ /** docs */ /*! inner */ outer */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn skips_utf8_cr_and_line_markers_in_nested_comments() {
        assert_eq!(
            kinds("/* café /* // inner\rπ */ outer */fn"),
            vec![TokenKind::Fn, TokenKind::Eof]
        );
    }

    #[test]
    fn rejects_unterminated_nested_block_comment_at_outer_opener() {
        let error = lex(&SourceFile::new("test.rs", "/* outer /* inner */"))
            .expect_err("unterminated nested block comment should fail");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'/'));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn preserves_token_spans_after_nested_block_comment() {
        let source = SourceFile::new("test.rs", "/* a /* b */ c */fn");
        let tokens = lex(&source).expect("nested block comment followed by token should lex");
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[0].span.lo().get(), 17);
        assert_eq!(tokens[0].span.hi().get(), 19);
        assert_eq!(tokens[1].kind, TokenKind::Eof);
        assert_eq!(tokens[1].span.lo().get(), 19);
        assert_eq!(tokens[1].span.hi().get(), 19);
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
    fn preserves_token_spans_after_block_comment() {
        let source = SourceFile::new("test.rs", "/* x */fn");
        let tokens = lex(&source).expect("block comment followed by token should lex");
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[0].span.lo().get(), 7);
        assert_eq!(tokens[0].span.hi().get(), 9);
        assert_eq!(tokens[1].kind, TokenKind::Eof);
        assert_eq!(tokens[1].span.lo().get(), 9);
        assert_eq!(tokens[1].span.hi().get(), 9);
    }

    #[test]
    fn recognizes_all_non_delimiter_punctuation_spellings() {
        let cases = [
            ("...", Punctuation::Ellipsis),
            ("..=", Punctuation::DotDotEq),
            ("<<=", Punctuation::ShlEq),
            (">>=", Punctuation::ShrEq),
            ("!=", Punctuation::BangEq),
            ("%=", Punctuation::PercentEq),
            ("&&", Punctuation::AmpAmp),
            ("&=", Punctuation::AmpEq),
            ("*=", Punctuation::StarEq),
            ("+=", Punctuation::PlusEq),
            ("-=", Punctuation::MinusEq),
            ("->", Punctuation::ThinArrow),
            ("..", Punctuation::DotDot),
            ("/=", Punctuation::SlashEq),
            ("::", Punctuation::ColonColon),
            ("<-", Punctuation::LeftArrow),
            ("<<", Punctuation::Shl),
            ("<=", Punctuation::LessEq),
            ("==", Punctuation::EqEq),
            ("=>", Punctuation::FatArrow),
            (">=", Punctuation::GreaterEq),
            (">>", Punctuation::Shr),
            ("^=", Punctuation::CaretEq),
            ("|=", Punctuation::PipeEq),
            ("||", Punctuation::PipePipe),
            ("!", Punctuation::Bang),
            ("#", Punctuation::Pound),
            ("$", Punctuation::Dollar),
            ("%", Punctuation::Percent),
            ("&", Punctuation::Amp),
            ("*", Punctuation::Star),
            ("+", Punctuation::Plus),
            (",", Punctuation::Comma),
            ("-", Punctuation::Minus),
            (".", Punctuation::Dot),
            ("/", Punctuation::Slash),
            (":", Punctuation::Colon),
            (";", Punctuation::Semicolon),
            ("<", Punctuation::Less),
            ("=", Punctuation::Eq),
            (">", Punctuation::Greater),
            ("?", Punctuation::Question),
            ("@", Punctuation::At),
            ("^", Punctuation::Caret),
            ("|", Punctuation::Pipe),
            ("~", Punctuation::Tilde),
        ];

        for (spelling, expected) in cases {
            let source = SourceFile::new("test.rs", spelling);
            let tokens = lex(&source).expect("punctuation should lex");
            assert_eq!(expected.as_str(), spelling);
            assert_eq!(tokens[0].kind, TokenKind::Punctuation(expected));
            assert_eq!(tokens[0].span.lo().get(), 0);
            assert_eq!(tokens[0].span.hi().get(), spelling.len() as u32);
            assert_eq!(tokens[1].kind, TokenKind::Eof);
        }
    }

    #[test]
    fn punctuation_prefers_longest_spelling() {
        assert_eq!(
            kinds("... ..= .. . <<= << <= <- < >>= >> >= > :: : -> - => == = && &= & || |= |"),
            vec![
                TokenKind::Punctuation(Punctuation::Ellipsis),
                TokenKind::Punctuation(Punctuation::DotDotEq),
                TokenKind::Punctuation(Punctuation::DotDot),
                TokenKind::Punctuation(Punctuation::Dot),
                TokenKind::Punctuation(Punctuation::ShlEq),
                TokenKind::Punctuation(Punctuation::Shl),
                TokenKind::Punctuation(Punctuation::LessEq),
                TokenKind::Punctuation(Punctuation::LeftArrow),
                TokenKind::Punctuation(Punctuation::Less),
                TokenKind::Punctuation(Punctuation::ShrEq),
                TokenKind::Punctuation(Punctuation::Shr),
                TokenKind::Punctuation(Punctuation::GreaterEq),
                TokenKind::Punctuation(Punctuation::Greater),
                TokenKind::Punctuation(Punctuation::ColonColon),
                TokenKind::Punctuation(Punctuation::Colon),
                TokenKind::Punctuation(Punctuation::ThinArrow),
                TokenKind::Punctuation(Punctuation::Minus),
                TokenKind::Punctuation(Punctuation::FatArrow),
                TokenKind::Punctuation(Punctuation::EqEq),
                TokenKind::Punctuation(Punctuation::Eq),
                TokenKind::Punctuation(Punctuation::AmpAmp),
                TokenKind::Punctuation(Punctuation::AmpEq),
                TokenKind::Punctuation(Punctuation::Amp),
                TokenKind::Punctuation(Punctuation::PipePipe),
                TokenKind::Punctuation(Punctuation::PipeEq),
                TokenKind::Punctuation(Punctuation::Pipe),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn comments_keep_priority_over_punctuation() {
        assert_eq!(
            kinds("/ /= * *= // line\n/* outer /* nested */ outer */ +"),
            vec![
                TokenKind::Punctuation(Punctuation::Slash),
                TokenKind::Punctuation(Punctuation::SlashEq),
                TokenKind::Punctuation(Punctuation::Star),
                TokenKind::Punctuation(Punctuation::StarEq),
                TokenKind::Punctuation(Punctuation::Plus),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn rejects_reserved_multi_pound_forms() {
        for input in ["##", "###"] {
            let error = lex(&SourceFile::new("test.rs", input))
                .expect_err("reserved multi-pound form should fail");
            assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'#'));
            assert_eq!(error.span.lo().get(), 0);
            assert_eq!(error.span.hi().get(), 1);
        }

        assert_eq!(
            kinds("# #"),
            vec![
                TokenKind::Punctuation(Punctuation::Pound),
                TokenKind::Punctuation(Punctuation::Pound),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn rejects_identifier_adjacent_pound_prefix() {
        let error = lex(&SourceFile::new("test.rs", "name#foo"))
            .expect_err("reserved identifier pound prefix should fail");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'#'));
        assert_eq!(error.span.lo().get(), 4);
        assert_eq!(error.span.hi().get(), 5);
    }

    #[test]
    fn keeps_raw_identifier_as_future_gap() {
        let error = lex(&SourceFile::new("test.rs", "r#name"))
            .expect_err("raw identifiers are reserved for P1-M011");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'#'));
        assert_eq!(error.span.lo().get(), 1);
        assert_eq!(error.span.hi().get(), 2);
    }

    #[test]
    fn keeps_delimiters_as_future_gap() {
        let error =
            lex(&SourceFile::new("test.rs", "()")).expect_err("delimiters are reserved for P1-M005");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'('));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn keeps_lifetimes_as_future_gap() {
        let error =
            lex(&SourceFile::new("test.rs", "'a")).expect_err("lifetimes are reserved for P1-M012");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'\''));
        assert_eq!(error.span.lo().get(), 0);
        assert_eq!(error.span.hi().get(), 1);
    }

    #[test]
    fn preserves_punctuation_spans_between_tokens() {
        let source = SourceFile::new("test.rs", "fn->main");
        let tokens = lex(&source).expect("supported punctuation should lex");
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens[0].span.lo().get(), 0);
        assert_eq!(tokens[0].span.hi().get(), 2);
        assert_eq!(
            tokens[1].kind,
            TokenKind::Punctuation(Punctuation::ThinArrow)
        );
        assert_eq!(tokens[1].span.lo().get(), 2);
        assert_eq!(tokens[1].span.hi().get(), 4);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].span.lo().get(), 4);
        assert_eq!(tokens[2].span.hi().get(), 8);
        assert_eq!(tokens[3].kind, TokenKind::Eof);
        assert_eq!(tokens[3].span.lo().get(), 8);
        assert_eq!(tokens[3].span.hi().get(), 8);
    }

    #[test]
    fn rejects_bare_underscore_for_now() {
        let error = lex(&SourceFile::new("test.rs", "_")).expect_err("underscore is unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'_'));
    }

    #[test]
    fn rejects_delimiter_for_now() {
        let error = lex(&SourceFile::new("test.rs", "(")).expect_err("delimiter is unsupported");
        assert_eq!(error.kind, LexErrorKind::UnexpectedByte(b'('));
    }
}
