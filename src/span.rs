mod tests;

use core::{
    fmt::Display,
    ops::{Deref, DerefMut, Index},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A span of source code.
///
/// This type represents a range of source code, from `start` to `end` (exclusive).
///
/// Spans are used throughout the compiler to associate AST nodes with the source
/// code that they came from. They are also used for error reporting.
pub struct Span {
    // start is the index of the first character in the span
    /// The index of the first character in the span.
    pub start: usize,
    // end is the index of the character AFTER the last character in the span
    /// The index of the character AFTER the last character in the span.
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn join(&self, other: Self) -> Self {
        assert!(self.end >= other.start);
        assert!(self.start <= other.end);
        assert!(self.end <= other.end);
        Self {
            start: self.start,
            end: other.end,
        }
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn slice<'a>(&'a self, s: &'a [u8]) -> &[u8] {
        &s[self.start..self.end]
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl From<Span> for std::ops::Range<usize> {
    fn from(span: Span) -> Self {
        span.range()
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Index<Span> for [u8] {
    type Output = [u8];

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.range()]
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.range()]
    }
}

/// A wrapper type that combines a value with a span of source code.
///
/// This type is used throughout the lexer and parser to associate a value
/// with the part of source code that it represents. The span is used for
/// error reporting and can be used to retrieve the source code that the
/// value came from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spanned<T> {
    /// The span of source code that the wrapped value represents.
    ///
    /// This span is used for error reporting and can be used to retrieve
    /// the source code that the wrapped value came from.
    pub span: Span,
    /// The wrapped value.
    ///
    /// This is the value that this `Spanned` wraps.
    pub value: T,
}

impl<T> Spanned<T> {
    pub fn new(span: impl Into<Span>, value: T) -> Spanned<T> {
        Spanned {
            span: span.into(),
            value,
        }
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> AsRef<Span> for Spanned<T> {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl<T> AsMut<Span> for Spanned<T> {
    fn as_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}
