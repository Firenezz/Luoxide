use core::ops::Range;

use crate::span::Span;

#[derive(Clone, Debug)]
pub struct SpannedError {
    pub span: Span,
    pub message: String,
}

pub trait MaybeSpan {
    fn into_span(self) -> Span;
}

impl MaybeSpan for Span {
    fn into_span(self) -> Span {
        self
    }
}

impl MaybeSpan for Range<usize> {
    fn into_span(self) -> Span {
        self.into()
    }
}

impl MaybeSpan for Option<Range<usize>> {
    fn into_span(self) -> Span {
        match self {
            Some(v) => v.into(),
            None => (0..0).into(),
        }
    }
}
