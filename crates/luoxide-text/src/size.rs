use std::num::TryFromIntError;

use crate::traits::TextLen;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextSize {
    pub(crate) raw: u32,
}

impl std::fmt::Debug for TextSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl TextSize {
    /// Creates a new instance of [`TextSize`] with the given offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - A u32 value representing the offset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luoxide_text::size::TextSize;
    /// 
    /// let size = TextSize::new(10);
    /// 
    /// assert_eq!(size.to_u32(), 10);
    /// ```
    #[inline]
    pub const fn new(offset: u32) -> Self {
        Self { raw: offset }
    }

    /// Creates a new instance of [`TextSize`] with the given offset.
    /// 
    /// # Arguments
    /// 
    /// * `text` - A string slice representing the text.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use luoxide_text::size::TextSize;
    /// 
    /// let size = TextSize::of("Hello, World!");
    /// 
    /// assert_eq!(size.to_u32(), 13);
    /// ```
    #[inline]
    pub fn of<T: TextLen>(text: T) -> TextSize {
        text.text_len()
    }

    /// Returns the raw value of the [`TextSize`].
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use luoxide_text::size::TextSize;
    /// 
    /// let size = TextSize::new(10);
    /// 
    /// assert_eq!(size.to_u32(), 10u32);
    /// ```
    #[inline]
    pub const fn to_u32(&self) -> u32 {
        self.raw
    }

    /// Returns the raw value of the [`TextSize`].
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use luoxide_text::size::TextSize;
    /// 
    /// let size = TextSize::new(10);
    /// 
    /// assert_eq!(size.to_usize(), 10usize);
    /// ```
    #[inline]
    pub const fn to_usize(&self) -> usize {
        self.raw as usize
    }
}

impl From<u32> for TextSize {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<TextSize> for u32 {
    fn from(value: TextSize) -> Self {
        value.to_u32()
    }
}

impl TryFrom<usize> for TextSize {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(u32::try_from(value)?.into())
    }
}

impl From<TextSize> for usize {
    fn from(value: TextSize) -> Self {
        value.to_usize()
    }
}

impl TextSize {
    /// Checked integer addition. Computes `self + rhs`, returning None if overflow occurred.
    pub fn checked_add(&self, rhs: Self) -> Option<Self> {
        self.raw
            .checked_add(rhs.raw)
            .map(TextSize::new)
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning None if overflow occurred.
    pub fn checked_sub(&self, rhs: Self) -> Option<Self> {
        self.raw
            .checked_sub(rhs.raw)
            .map(TextSize::new)
    }
}

mod operators {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
    use super::*;
    operator!(impl Add for TextSize by fn add = +);
    operator!(impl Sub for TextSize by fn sub = -);
    operator!(impl Mul for TextSize by fn mul = *);
    operator!(impl Div for TextSize by fn div = /);

    impl AddAssign for TextSize {
        fn add_assign(&mut self, rhs: Self) {
            *self = Self {
                raw: self.raw + rhs.raw,
            };
        }
    }

    impl SubAssign for TextSize {
        fn sub_assign(&mut self, rhs: Self) {
            *self = Self {
                raw: self.raw - rhs.raw,
            };
        }
    }

    impl<T> MulAssign<T> for TextSize
    where 
        TextSize: Mul<T, Output = TextSize>
    {
        fn mul_assign(&mut self, rhs: T) {
            *self = Self {
                raw: (*self * rhs).to_u32(),
            };
        }
    }

    impl<T> DivAssign<T> for TextSize
    where 
        TextSize: Div<T, Output = TextSize>
    {
        fn div_assign(&mut self, rhs: T) {
            *self = Self {
                raw: (*self / rhs).to_u32(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = "test";
    #[test]
    fn it_create_succesfully() {
        let result = TextSize::new(5);
        assert_eq!(result, TextSize { raw: 5 });
        assert_eq!(result.to_u32(), 5u32);
        assert_eq!(result.to_usize(), 5usize);
    }

    #[test]
    fn it_create_of_text_successfully() {
        let result = TextSize::of(TEST_STRING);
        assert_eq!(result, TextSize { raw: 4 });
        assert_eq!(result.to_u32(), 4u32);
        assert_eq!(result.to_usize(), 4usize);
    }

    mod operations {
        use super::*;
        #[test]
        fn it_checked_add_successfully() {
            let result = TextSize::of("test")
                .checked_add(5u32.into())
                .expect("Addition should have been successful");

            assert_eq!(result.raw, 4 + 5);
        }
        
        #[test]
        fn it_checked_sub_successfully() {
            let result = TextSize::of("test")
                .checked_sub(2u32.into());

            assert_eq!(result, Some(TextSize{ raw: 4 - 2 }), "testing {} - {}", 4, 2);
        }

        #[test]
        fn it_checked_add_return_none() {
            let result = TextSize::of(TEST_STRING)
                .checked_add(u32::MAX.into());

            assert_eq!(result, None, "testing \"{}\".len() + u32::MAX", TEST_STRING);
        }
        
        #[test]
        fn it_checked_sub_return_none() {
            let result = TextSize::new(0)
                .checked_sub(2u32.into());

            assert_eq!(result, None, "testing 0 - 5");
        }

        #[test]
        fn it_add_successfully() {

        }
    }
}
