use crate::span::Span;

use super::text::TextSize;

pub struct TextRange {
    pub start: TextSize,
    pub end: TextSize,
}

impl TextRange {
    pub fn new(start: TextSize, end: TextSize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> TextSize {
        self.start
    }

    pub fn end(&self) -> TextSize {
        self.end
    }

    pub fn length(&self) -> TextSize {
        self.end - self.start
    }
}

impl From<Span> for TextRange {
    fn from(span: Span) -> Self {
        TextRange::new(span.start.into(), span.end.into())
    }
}

impl From<TextRange> for Span {
    fn from(range: TextRange) -> Self {
        Span::new(range.start.into(), range.end.into())
    }
}
