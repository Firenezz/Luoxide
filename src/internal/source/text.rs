use core::fmt;
use std::{
    iter::Sum,
    num::TryFromIntError,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use super::traits::TextLength;

/// A size and location in the source code.
///
/// It is used to represent the position in UTF-8 bytes offset in the source code.
///
///
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextSize {
    raw: usize,
}

impl fmt::Debug for TextSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl fmt::Display for TextSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl TextSize {
    pub const fn new(raw: usize) -> Self {
        Self { raw }
    }

    pub const fn from_offset(offset: usize) -> Self {
        Self { raw: offset }
    }

    pub const fn from_size(size: usize) -> Self {
        Self { raw: size }
    }

    pub fn of<T: TextLength>(text: T) -> TextSize {
        text.text_length()
    }

    pub fn to_u32(&self) -> Result<u32, TryFromIntError> {
        u32::try_from(self.raw)
    }

    pub fn to_u64(&self) -> Result<u64, TryFromIntError> {
        u64::try_from(self.raw)
    }

    pub const fn to_usize(&self) -> usize {
        self.raw
    }
}

impl From<usize> for TextSize {
    fn from(raw: usize) -> Self {
        Self { raw }
    }
}

impl From<TextSize> for usize {
    fn from(size: TextSize) -> Self {
        size.raw
    }
}

impl TryFrom<TextSize> for u32 {
    type Error = TryFromIntError;
    fn try_from(size: TextSize) -> Result<Self, Self::Error> {
        size.to_u32()
    }
}

impl TryFrom<TextSize> for u64 {
    type Error = TryFromIntError;
    fn try_from(size: TextSize) -> Result<Self, Self::Error> {
        size.to_u64()
    }
}

macro_rules! operations {
    (impl $trait:ident for TextSize with fn $function:ident = $op:tt) => {
        impl $trait<TextSize> for TextSize {
            type Output = TextSize;
            #[inline]
            fn $function(self, other: TextSize) -> TextSize {
                TextSize::new(self.raw $op other.raw)
            }
        }

        impl $trait<&TextSize> for TextSize {
            type Output = TextSize;
            #[inline]
            fn $function(self, other: &TextSize) -> TextSize {
                TextSize::new(self.raw $op other.raw)
            }
        }

        impl<TOther> $trait<TOther> for &TextSize
        where
            TextSize: $trait<TOther, Output = TextSize>
        {
            type Output = TextSize;
            #[inline]
            fn $function(self, other: TOther) -> TextSize {
                *self $op other
            }
        }
    };
}

use core::ops::{Add, Div, Mul, Sub};
operations!(impl Add for TextSize with fn add = +);
operations!(impl Sub for TextSize with fn sub = -);
operations!(impl Mul for TextSize with fn mul = *);
operations!(impl Div for TextSize with fn div = /);

impl<TOther> AddAssign<TOther> for TextSize
where
    TextSize: Add<TOther, Output = TextSize>,
{
    #[inline]
    fn add_assign(&mut self, other: TOther) {
        *self = *self + other
    }
}

impl<TOther> SubAssign<TOther> for TextSize
where
    TextSize: Sub<TOther, Output = TextSize>,
{
    #[inline]
    fn sub_assign(&mut self, other: TOther) {
        *self = *self - other
    }
}

impl<TOther> MulAssign<TOther> for TextSize
where
    TextSize: Mul<TOther, Output = TextSize>,
{
    #[inline]
    fn mul_assign(&mut self, other: TOther) {
        *self = *self * other
    }
}

impl<TOther> DivAssign<TOther> for TextSize
where
    TextSize: Div<TOther, Output = TextSize>,
{
    #[inline]
    fn div_assign(&mut self, other: TOther) {
        *self = *self / other
    }
}

impl Sum for TextSize {
    fn sum<I: Iterator<Item = TextSize>>(iter: I) -> TextSize {
        iter.fold(TextSize::new(0), |a, b| a + b)
    }
}
