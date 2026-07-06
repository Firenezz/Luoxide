use std::num::TryFromIntError;

use priv_in_pub::Sealed;

use crate::{range::TextSpan, size::TextSize};

mod priv_in_pub {
    pub trait Sealed {}
}

/// Primitives with a textual length that can be passed to [`TextSize::of`].
pub trait TextLen: Copy + Sealed {
    /// The textual length of this primitive.
    fn text_len(self) -> TextSize {
        self.try_text_len().unwrap()
    }
    /// Try to get the textual length of this primitive.
    fn try_text_len(self) -> Result<TextSize, TryFromIntError>;
}

impl Sealed for &'_ str {}
impl TextLen for &'_ str {
    #[inline]
    fn try_text_len(self) -> Result<TextSize, TryFromIntError> {
        self.len().try_into()
    }
}

impl Sealed for &'_ [u8] {}
impl TextLen for &'_ [u8] {
    #[inline]
    fn try_text_len(self) -> Result<TextSize, TryFromIntError> {
        self.len().try_into()
    }
}

impl Sealed for &'_ String {}
impl TextLen for &'_ String {
    #[inline]
    fn try_text_len(self) -> Result<TextSize, TryFromIntError> {
        self.as_str().try_text_len()
    }
}

impl Sealed for char {}
impl TextLen for char {
    #[inline]
    fn try_text_len(self) -> Result<TextSize, TryFromIntError> {
        Ok((self.len_utf8() as u32).into()) // Should always succeed because len is always 1..=4
    }
}

pub trait Ranged {
    /// The range of this item in the source text.
    fn range(&self) -> TextSpan;

    /// The start offset of this item in the source text.
    fn start(&self) -> TextSize {
        self.range().start
    }

    /// The end offset of this item in the source text.
    fn end(&self) -> TextSize {
        self.range().end
    }
}
