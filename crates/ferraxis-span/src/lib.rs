//! Compact source byte positions and half-open spans.

/// A zero-based byte position in UTF-8 source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytePos(u32);

impl BytePos {
    /// Creates a byte position from a raw offset.
    #[must_use]
    pub const fn new(offset: u32) -> Self {
        Self(offset)
    }

    /// Returns the raw zero-based byte offset.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// A half-open source range `[lo, hi)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    lo: BytePos,
    hi: BytePos,
}

impl Span {
    /// Creates a span when `lo <= hi`.
    #[must_use]
    pub const fn new(lo: BytePos, hi: BytePos) -> Self {
        assert!(lo.get() <= hi.get(), "span start must not exceed span end");
        Self { lo, hi }
    }

    /// Creates a zero-width span at `pos`.
    #[must_use]
    pub const fn empty(pos: BytePos) -> Self {
        Self { lo: pos, hi: pos }
    }

    /// Returns the inclusive start byte position.
    #[must_use]
    pub const fn lo(self) -> BytePos {
        self.lo
    }

    /// Returns the exclusive end byte position.
    #[must_use]
    pub const fn hi(self) -> BytePos {
        self.hi
    }

    /// Returns the byte length of the span.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.hi.get() - self.lo.get()
    }

    /// Returns whether the span has zero length.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.lo.get() == self.hi.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{BytePos, Span};

    #[test]
    fn spans_are_half_open() {
        let span = Span::new(BytePos::new(3), BytePos::new(7));
        assert_eq!(span.lo().get(), 3);
        assert_eq!(span.hi().get(), 7);
        assert_eq!(span.len(), 4);
        assert!(!span.is_empty());
    }

    #[test]
    fn empty_spans_have_zero_length() {
        let span = Span::empty(BytePos::new(5));
        assert_eq!(span.len(), 0);
        assert!(span.is_empty());
    }
}
