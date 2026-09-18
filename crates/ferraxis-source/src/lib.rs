//! UTF-8 source-file storage and span-based slicing.

use ferraxis_span::Span;

/// One UTF-8 source file owned by the compiler session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    name: String,
    text: String,
}

impl SourceFile {
    /// Creates a source file.
    #[must_use]
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
    }

    /// Returns the logical source-file name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the complete UTF-8 source text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the source byte length if it fits the Phase 0 `u32` position model.
    #[must_use]
    pub fn byte_len(&self) -> Option<u32> {
        u32::try_from(self.text.len()).ok()
    }

    /// Returns the text covered by `span` when the span is in bounds and on UTF-8 boundaries.
    #[must_use]
    pub fn slice(&self, span: Span) -> Option<&str> {
        let lo = usize::try_from(span.lo().get()).ok()?;
        let hi = usize::try_from(span.hi().get()).ok()?;
        self.text.get(lo..hi)
    }
}

#[cfg(test)]
mod tests {
    use ferraxis_span::{BytePos, Span};

    use super::SourceFile;

    #[test]
    fn slices_valid_utf8_boundaries() {
        let file = SourceFile::new("test.rs", "fn main");
        let span = Span::new(BytePos::new(3), BytePos::new(7));
        assert_eq!(file.slice(span), Some("main"));
    }

    #[test]
    fn rejects_non_boundary_slices() {
        let file = SourceFile::new("test.rs", "é");
        let span = Span::new(BytePos::new(0), BytePos::new(1));
        assert_eq!(file.slice(span), None);
    }
}
