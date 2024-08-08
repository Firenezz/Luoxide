
use priv_in_pub::Sealed;

use crate::{range::TextRange, size::TextSize};

mod priv_in_pub {
    pub trait Sealed {}
}

/// Primitives with a textual length that can be passed to [`TextSize::of`].
pub trait TextLen: Copy + Sealed {
    /// The textual length of this primitive.
    fn text_len(self) -> TextSize;
}

impl Sealed for &'_ str {}
impl TextLen for &'_ str {
    #[inline]
    fn text_len(self) -> TextSize {
        self.len().try_into().unwrap()
    }
}

impl Sealed for &'_ String {}
impl TextLen for &'_ String {
    #[inline]
    fn text_len(self) -> TextSize {
        self.as_str().text_len()
    }
}

impl Sealed for char {}
impl TextLen for char {
    #[inline]
    fn text_len(self) -> TextSize {
        (self.len_utf8() as u32).into()
    }
}

pub trait Ranged {
    /// The range of this item in the source text.
    fn range(&self) -> TextRange;

    /// The start offset of this item in the source text.
    fn start(&self) -> TextSize {
        self.range().start
    }

    /// The end offset of this item in the source text.
    fn end(&self) -> TextSize {
        self.range().end
    }
}
