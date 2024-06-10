use super::text::TextSize;

use priv_in_pub::Sealed;
mod priv_in_pub {
    pub trait Sealed {}
}

pub trait TextLength: Copy + Sealed {
    fn text_length(self) -> TextSize;
}

impl Sealed for &String {}
impl TextLength for &String {
    #[inline]
    fn text_length(self) -> TextSize {
        self.as_str().text_length()
    }
}

impl Sealed for &str {}
impl TextLength for &str {
    #[inline]
    fn text_length(self) -> TextSize {
        (self.len()).into()
    }
}

impl Sealed for char {}
impl TextLength for char {
    #[inline]
    fn text_length(self) -> TextSize {
        (self.len_utf8()).into()
    }
}
