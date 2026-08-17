use core::{
    cmp::Ordering,
    ops::{Bound, Index, IndexMut, Range, RangeBounds},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::size::TextSize;

/// A range of text
///
/// ## Examples
///
/// ```rust
/// use luoxide_text::traits::Ranged;
/// use luoxide_text::range::TextSpan;
/// use luoxide_text::size::TextSize;
///
/// let range = TextSpan::new(0.into(), 20.into());
/// assert_eq!(range, TextSpan { start: 0.into(), end: 20.into() });
/// ```
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextSpan {
    pub start: TextSize,
    pub end: TextSize,
}

impl core::fmt::Display for TextSpan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl core::fmt::Debug for TextSpan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#?}..{:#?}", self.start, self.end)
    }
}

// Creation implementations
impl TextSpan {
    /// Create a new [`TextSpan`]
    ///
    /// ## Arguments
    ///
    /// * `start` - The start of the range
    /// * `end` - The end of the range
    ///
    /// ## Panics
    ///
    /// This function will panic if `start` is greater than `end`
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(0.into(), 20.into());
    /// assert_eq!(range, TextSpan { start: 0.into(), end: 20.into() });
    /// ```
    #[inline]
    pub const fn new(start: TextSize, end: TextSize) -> Self {
        assert!(start.raw <= end.raw);
        Self { start, end }
    }

    /// Create a new [`TextSpan`] from an offset and a length
    ///
    /// ## Arguments
    ///
    /// * `offset` - The offset of the range
    /// * `len` - The length of the range
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::at(5.into(), 15.into());
    /// assert_eq!(range, TextSpan { start: 5.into(), end: 20.into() });
    /// ```
    #[inline]
    pub fn at(offset: TextSize, len: TextSize) -> TextSpan {
        TextSpan::new(offset, offset + len)
    }

    /// Create an empty [`TextSpan`] at the given offset
    ///
    /// ## Arguments
    ///
    /// * `offset` - The offset of the range
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::empty(5.into());
    /// assert_eq!(range, TextSpan { start: 5.into(), end: 5.into() });
    /// assert!(range.is_empty());
    /// ```
    #[inline]
    pub const fn empty(offset: TextSize) -> TextSpan {
        TextSpan {
            start: offset,
            end: offset,
        }
    }

    /// Create an up-to [`TextSpan`] at the given offset
    ///
    /// ## Arguments
    ///
    /// * `end` - The end of the range
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::up_to(5.into());
    /// assert_eq!(range, TextSpan { start: 0.into(), end: 5.into() });
    /// ```
    #[inline]
    pub const fn up_to(end: TextSize) -> TextSpan {
        TextSpan {
            start: TextSize::new(0),
            end,
        }
    }

    /// Creates a [`Range<usize>`] from a [`TextSpan`]
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(0.into(), 20.into());
    /// assert_eq!(range.to_range(), 0..20);
    /// ```
    #[inline]
    pub fn to_range(self) -> Range<usize> {
        self.start.into()..self.end.into()
    }

    /// Creates a new [`TextSpan`] so that the new [`TextSpan`] covers both [`TextSpan`]
    ///
    /// ## Returns
    ///
    /// A [`TextSpan`] that covers both spans
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range1 = TextSpan::new(0.into(), 20.into());
    /// let range2 = TextSpan::new(30.into(), 40.into());
    ///
    /// assert_eq!(range1.merge(range2), TextSpan::new(0.into(), 40.into()));
    /// ```
    #[inline]
    pub fn merge(self, other: TextSpan) -> TextSpan {
        TextSpan::new(
            std::cmp::min(self.start, other.start),
            std::cmp::max(self.end, other.end),
        )
    }
}

impl<T: Into<TextSize>> From<Range<T>> for TextSpan {
    fn from(value: Range<T>) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl<T: From<TextSize>> From<TextSpan> for Range<T> {
    fn from(value: TextSpan) -> Self {
        value.start.into()..value.end.into()
    }
}

// Operations implementations
impl TextSpan {
    /// Get the length of the [`TextSpan`]
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(0.into(), 20.into());
    /// assert_eq!(range.len(), 20.into());
    /// ```
    #[inline]
    pub const fn len(self) -> TextSize {
        TextSize::new(self.end.raw - self.start.raw)
    }

    /// Check if the [`TextSpan`] is empty
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::empty(5.into());
    ///
    /// assert!(range.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.start.raw == self.end.raw
    }

    /// Check if the range completely contains another range
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let larger_range = TextSpan::new(0.into(), 20.into());
    /// let smaller_range = TextSpan::new(10.into(), 15.into());
    ///
    /// assert!(larger_range.contains_range(smaller_range));
    /// assert!(!smaller_range.contains_range(larger_range));
    ///
    /// assert!(larger_range.contains_range(larger_range));
    /// assert!(smaller_range.contains_range(smaller_range));
    /// ```
    #[inline]
    pub fn contains_range(&self, range: TextSpan) -> bool {
        self.start <= range.start && range.end <= self.end
    }

    /// Check if the offset is with the range exlusively
    ///
    /// ``self.start <= offset && offset < self.end``
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(0.into(), 20.into());
    ///
    /// assert!(range.contains_offset(10.into()));
    /// assert!(!range.contains_offset(20.into()));
    /// assert!(!range.contains_offset(30.into()));
    /// ```
    #[inline]
    pub fn contains_offset(&self, offset: TextSize) -> bool {
        self.start <= offset && offset < self.end
    }

    /// Check if the offset is with the range inclusively
    ///
    /// ``self.start <= offset && offset <= self.end``
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(0.into(), 20.into());
    ///
    /// assert!(range.contains_offset_inclusive(10.into()));
    /// assert!(range.contains_offset_inclusive(20.into()));
    /// assert!(!range.contains_offset_inclusive(30.into()));
    /// ```
    #[inline]
    pub fn contains_offset_inclusive(&self, offset: TextSize) -> bool {
        self.start <= offset && offset <= self.end
    }

    /// Check if both given range intersect
    ///
    /// ## Returns
    ///
    /// If no intersection is detected returns [`None`]
    ///
    /// If one is found, returns the intersection as a [`TextSpan`]
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range1 = TextSpan::new(0.into(), 20.into());
    /// let range2 = TextSpan::new(10.into(), 30.into());
    ///
    /// let no_intersect = TextSpan::new(50.into(), 70.into());
    ///
    /// assert_eq!(range1.intersect(range2), Some(TextSpan::new(10.into(), 20.into())));
    /// assert_eq!(range1.intersect(no_intersect), None);
    /// ```
    #[inline]
    pub fn intersect(&self, other: TextSpan) -> Option<TextSpan> {
        let start = core::cmp::max(self.start, other.start);
        let end = core::cmp::min(self.end, other.end);

        if end < start {
            // No intersection
            None
        } else {
            // Intersection
            Some(TextSpan::new(start, end))
        }
    }

    /// Compare two ranges
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    /// use std::cmp::Ordering;
    ///
    /// let range1 = TextSpan::new(0.into(), 5.into());
    /// let range2 = TextSpan::new(10.into(), 30.into());
    ///
    /// assert_eq!(range1.ordering(range2), Ordering::Less);
    /// assert_eq!(range2.ordering(range1), Ordering::Greater);
    /// assert_eq!(range1.ordering(range1), Ordering::Equal);
    /// ```
    #[inline]
    pub const fn ordering(self, other: TextSpan) -> Ordering {
        if self.end.raw <= other.start.raw {
            Ordering::Less
        } else if self.start.raw >= other.end.raw {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

// Manipulation functions
impl TextSpan {
    /// Creates a new [`TextSpan`] that covers both given ranges
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range1 = TextSpan::new(0.into(), 20.into());
    /// let range2 = TextSpan::new(10.into(), 30.into());
    ///
    /// let expected = TextSpan::new(0.into(), 30.into());
    ///
    /// assert_eq!(range1.cover(range2), expected);
    /// ```
    #[inline]
    pub fn cover(self, other: TextSpan) -> Self {
        TextSpan::new(
            std::cmp::min(self.start, other.start),
            std::cmp::max(self.end, other.end),
        )
    }

    /// Creates a new [`TextSpan`] that covers the given offset
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(0.into(), 20.into());
    ///
    /// let expected = TextSpan::new(0.into(), 30.into());
    ///
    /// assert_eq!(range.cover_offset(30.into()), expected);
    /// ```
    #[inline]
    pub fn cover_offset(self, offset: TextSize) -> Self {
        self.cover(TextSpan::empty(offset))
    }

    #[inline]
    pub fn checked_add(self, offset: TextSize) -> Option<TextSpan> {
        Some(TextSpan {
            start: self.start.checked_add(offset)?,
            end: self.end.checked_add(offset)?,
        })
    }

    #[inline]
    pub fn checked_sub(self, offset: TextSize) -> Option<TextSpan> {
        Some(TextSpan {
            start: self.start.checked_sub(offset)?,
            end: self.end.checked_sub(offset)?,
        })
    }

    /// Moves the start of the range by adding the amount to self.start
    ///
    /// `self.start + amount`
    ///
    /// # Example
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(TextSize::from(5), TextSize::from(10));
    /// assert_eq!(range
    ///     .add_start(
    ///         TextSize::from(2)),
    ///         TextSpan::new(
    ///             TextSize::from(7),
    ///             TextSize::from(10)
    /// ));
    /// ```
    #[inline]
    pub fn add_start(&self, amount: TextSize) -> TextSpan {
        TextSpan::new(self.start + amount, self.end)
    }

    /// Moves the end of the range by adding the amount to self.end
    ///
    /// `self.end + amount`
    ///
    /// # Example
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(TextSize::from(5), TextSize::from(10));
    /// assert_eq!(range
    ///     .add_end(
    ///         TextSize::from(2)),
    ///         TextSpan::new(
    ///             TextSize::from(5),
    ///             TextSize::from(12)
    /// ));
    /// ```
    #[inline]
    pub fn add_end(&self, amount: TextSize) -> TextSpan {
        TextSpan::new(self.start, self.end + amount)
    }

    /// Moves the start of the range by subtracting the amount to self.start
    ///
    /// `self.start - amount`
    ///
    /// # Example
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(TextSize::from(5), TextSize::from(10));
    /// assert_eq!(range
    ///     .sub_start(
    ///         TextSize::from(2)),
    ///         TextSpan::new(
    ///             TextSize::from(3),
    ///             TextSize::from(10)
    /// ));
    /// ```
    #[inline]
    pub fn sub_start(&self, amount: TextSize) -> TextSpan {
        TextSpan::new(self.start - amount, self.end)
    }

    /// Moves the end of the range by subtracting the amount to self.end
    ///
    /// `self.end - amount`
    ///
    /// # Example
    ///
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextSpan;
    /// use luoxide_text::size::TextSize;
    ///
    /// let range = TextSpan::new(TextSize::from(5), TextSize::from(10));
    /// assert_eq!(range
    ///     .sub_end(
    ///         TextSize::from(2)),
    ///         TextSpan::new(
    ///             TextSize::from(5),
    ///             TextSize::from(8)
    /// ));
    /// ```
    #[inline]
    pub fn sub_end(&self, amount: TextSize) -> TextSpan {
        TextSpan::new(self.start, self.end - amount)
    }
}

impl Index<TextSpan> for str {
    type Output = str;
    #[inline]
    fn index(&self, index: TextSpan) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

impl Index<TextSpan> for String {
    type Output = str;
    #[inline]
    fn index(&self, index: TextSpan) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

impl IndexMut<TextSpan> for str {
    #[inline]
    fn index_mut(&mut self, index: TextSpan) -> &mut Self::Output {
        &mut self[Range::<usize>::from(index)]
    }
}

impl IndexMut<TextSpan> for String {
    #[inline]
    fn index_mut(&mut self, index: TextSpan) -> &mut Self::Output {
        &mut self[Range::<usize>::from(index)]
    }
}

impl RangeBounds<TextSize> for TextSpan {
    #[inline]
    fn start_bound(&self) -> std::ops::Bound<&TextSize> {
        Bound::Included(&self.start)
    }

    #[inline]
    fn end_bound(&self) -> std::ops::Bound<&TextSize> {
        Bound::Excluded(&self.end)
    }
}

mod operators {
    use super::*;
    use core::ops::{Add, AddAssign, Sub, SubAssign};
    operator!(impl Add for TextSpan by fn add = + start,end);
    operator!(impl Sub for TextSpan by fn sub = - start,end);

    impl Add<TextSize> for TextSpan {
        type Output = TextSpan;

        fn add(self, rhs: TextSize) -> Self::Output {
            self.checked_add(rhs).expect("TextSpan + offset overflowed")
        }
    }

    impl Sub<TextSize> for TextSpan {
        type Output = TextSpan;

        fn sub(self, rhs: TextSize) -> Self::Output {
            self.checked_sub(rhs).expect("TextSpan - offset overflowed")
        }
    }

    impl<A> AddAssign<A> for TextSpan
    where
        TextSpan: Add<A, Output = TextSpan>,
    {
        #[inline]
        fn add_assign(&mut self, rhs: A) {
            *self = *self + rhs;
        }
    }

    impl<A> SubAssign<A> for TextSpan
    where
        TextSpan: Sub<A, Output = TextSpan>,
    {
        #[inline]
        fn sub_assign(&mut self, rhs: A) {
            *self = *self - rhs;
        }
    }
}
