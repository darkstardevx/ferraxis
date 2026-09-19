//! Integration tests for the Ferraxis token-dump command-line interface.
//!
//! These tests verify that the compiler driver exposes the initial lexer token
//! stream with stable token kinds, source spans, and source text where
//! applicable.

use std::{fs, process::Command};

#[test]
fn dumps_initial_token_stream() {
    let path = std::env::temp_dir().join(format!("ferraxis-token-dump-{}.rs", std::process::id()));

    fs::write(&path, "fn main\n").expect("write temporary source");

    let output = Command::new(env!("CARGO_BIN_EXE_ferraxis"))
        .arg("--emit=tokens")
        .arg(&path)
        .output()
        .expect("run ferraxis");

    let _ = fs::remove_file(&path);

    assert!(output.status.success());

    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        "Fn          0..2\nIdentifier  3..7  \"main\"\nEof         8..8\n"
    );
}

#[test]
fn dumps_punctuation_with_spans_and_spelling() {
    let path =
        std::env::temp_dir().join(format!("ferraxis-punctuation-dump-{}.rs", std::process::id()));

    fs::write(&path, "fn->main += value\n").expect("write temporary source");

    let output = Command::new(env!("CARGO_BIN_EXE_ferraxis"))
        .arg("--emit=tokens")
        .arg(&path)
        .output()
        .expect("run ferraxis");

    let _ = fs::remove_file(&path);

    assert!(output.status.success());

    assert_eq!(
        String::from_utf8(output.stdout).expect("utf8 stdout"),
        "Fn          0..2\n\
Punctuation 2..4  \"->\"\n\
Identifier  4..8  \"main\"\n\
Punctuation 9..11  \"+=\"\n\
Identifier  12..17  \"value\"\n\
Eof         18..18\n"
    );
}
