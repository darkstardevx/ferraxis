//! Ferraxis compiler driver.

use std::{env, fs, process};

use ferraxis_lexer::{LexErrorKind, TokenKind, lex};
use ferraxis_source::SourceFile;

fn main() {
    if let Err(message) = run() {
        eprintln!("ferraxis: {message}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(first) = args.next() else {
        return Err(usage());
    };

    if first == "--version" || first == "-V" {
        println!("ferraxis {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if first != "--emit=tokens" {
        return Err(format!("unsupported option `{first}`\n{}", usage()));
    }

    let path = args.next().ok_or_else(usage)?;
    if args.next().is_some() {
        return Err(format!("too many arguments\n{}", usage()));
    }

    let text =
        fs::read_to_string(&path).map_err(|error| format!("failed to read `{path}`: {error}"))?;
    let source = SourceFile::new(path.clone(), text);
    let tokens = lex(&source).map_err(|error| match error.kind {
        LexErrorKind::UnexpectedByte(byte) => format!(
            "unsupported byte 0x{byte:02x} at {}..{} in `{}`",
            error.span.lo().get(),
            error.span.hi().get(),
            source.name()
        ),
        LexErrorKind::SourceTooLarge => {
            format!(
                "source file `{}` exceeds Phase 0 size limits",
                source.name()
            )
        }
    })?;

    for token in tokens {
        let lo = token.span.lo().get();
        let hi = token.span.hi().get();
        match token.kind {
            TokenKind::Fn => println!("Fn          {lo}..{hi}"),
            TokenKind::Identifier => {
                let lexeme = source
                    .slice(token.span)
                    .ok_or_else(|| "internal error: invalid identifier span".to_owned())?;
                println!("Identifier  {lo}..{hi}  {lexeme:?}");
            }
            TokenKind::Punctuation(punctuation) => {
                println!("Punctuation {lo}..{hi}  {:?}", punctuation.as_str());
            }
            TokenKind::Delimiter(delimiter) => {
                println!("Delimiter   {lo}..{hi}  {:?}", delimiter.as_str());
            }
            TokenKind::Eof => println!("Eof         {lo}..{hi}"),
        }
    }

    Ok(())
}

fn usage() -> String {
    "usage: ferraxis --emit=tokens <source.rs>".to_owned()
}
