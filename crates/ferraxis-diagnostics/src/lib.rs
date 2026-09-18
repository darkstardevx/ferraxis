//! Structured diagnostic data independent of terminal rendering.

use ferraxis_span::Span;

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    /// A compilation error.
    Error,
    /// A warning that does not by itself reject compilation.
    Warning,
    /// Informational diagnostic output.
    Note,
}

/// A structured compiler diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Diagnostic severity.
    pub level: DiagnosticLevel,
    /// Stable diagnostic code when one has been assigned.
    pub code: Option<String>,
    /// Human-readable message.
    pub message: String,
    /// Source location when meaningful.
    pub span: Option<Span>,
}

impl Diagnostic {
    /// Creates an error diagnostic.
    #[must_use]
    pub fn error(message: impl Into<String>, span: Option<Span>) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            code: None,
            message: message.into(),
            span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, DiagnosticLevel};

    #[test]
    fn creates_structured_error() {
        let diagnostic = Diagnostic::error("unexpected token", None);
        assert_eq!(diagnostic.level, DiagnosticLevel::Error);
        assert_eq!(diagnostic.message, "unexpected token");
    }
}
