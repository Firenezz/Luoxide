use std::{
    cmp::Ordering,
    ops::{Bound, Index, IndexMut, Range, RangeBounds},
};

use crate::size::TextSize;

/// A range of text
/// 
/// ## Examples
/// 
/// ```rust
/// use luoxide_text::traits::Ranged;
/// use luoxide_text::range::TextRange;
/// use luoxide_text::size::TextSize;
/// 
/// let range = TextRange::new(0.into(), 20.into());
/// assert_eq!(range, TextRange { start: 0.into(), end: 20.into() });
/// ```
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextRange {
    pub start: TextSize,
    pub end: TextSize,
}

impl std::fmt::Debug for TextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}..{:#?}", self.start, self.end)
    }
}

// Creation implementations
impl TextRange {
    /// Create a new [`TextRange`]
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
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(0.into(), 20.into());
    /// assert_eq!(range, TextRange { start: 0.into(), end: 20.into() });
    /// ```
    #[inline]
    pub const fn new(start: TextSize, end: TextSize) -> Self {
        assert!(start.raw <= end.raw);
        Self { start, end }
    }

    /// Create a new [`TextRange`] from an offset and a length
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
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::at(5.into(), 15.into());
    /// assert_eq!(range, TextRange { start: 5.into(), end: 20.into() });
    /// ```
    #[inline]
    pub fn at(offset: TextSize, len: TextSize) -> TextRange {
        TextRange::new(offset, offset + len)
    }

    /// Create an empty [`TextRange`] at the given offset
    /// 
    /// ## Arguments
    /// 
    /// * `offset` - The offset of the range
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::empty(5.into());
    /// assert_eq!(range, TextRange { start: 5.into(), end: 5.into() });
    /// assert!(range.is_empty());
    /// ```
    #[inline]
    pub const fn empty(offset: TextSize) -> TextRange {
        TextRange {
            start: offset,
            end: offset,
        }
    }

    /// Create an up-to [`TextRange`] at the given offset
    /// 
    /// ## Arguments
    /// 
    /// * `end` - The end of the range
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::up_to(5.into());
    /// assert_eq!(range, TextRange { start: 0.into(), end: 5.into() });
    /// ```
    #[inline]
    pub const fn up_to(end: TextSize) -> TextRange {
        TextRange {
            start: TextSize::new(0),
            end,
        }
    }
}

impl From<Range<TextSize>> for TextRange {
    fn from(value: Range<TextSize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl<T: From<TextSize>> From<TextRange> for Range<T> {
    fn from(value: TextRange) -> Self {
        value.start.into()..value.end.into()
    }
}

// Operations implementations
impl TextRange {

    /// Get the length of the [`TextRange`]
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(0.into(), 20.into());
    /// assert_eq!(range.len(), 20.into());
    /// ```
    #[inline]
    pub const fn len(self) -> TextSize {
        TextSize::new(self.end.raw - self.start.raw)
    }

    /// Check if the [`TextRange`] is empty
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::empty(5.into());
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
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let larger_range = TextRange::new(0.into(), 20.into());
    /// let smaller_range = TextRange::new(10.into(), 15.into());
    /// 
    /// assert!(larger_range.contains_range(smaller_range));
    /// assert!(!smaller_range.contains_range(larger_range));
    /// 
    /// assert!(larger_range.contains_range(larger_range));
    /// assert!(smaller_range.contains_range(smaller_range));
    /// ```
    #[inline]
    pub fn contains_range(&self, range: TextRange) -> bool {
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
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(0.into(), 20.into());
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
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(0.into(), 20.into());
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
    /// If one is found, returns the intersection as a [`TextRange`]
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range1 = TextRange::new(0.into(), 20.into());
    /// let range2 = TextRange::new(10.into(), 30.into());
    /// 
    /// let no_intersect = TextRange::new(50.into(), 70.into());
    /// 
    /// assert_eq!(range1.intersect(range2), Some(TextRange::new(10.into(), 20.into())));
    /// assert_eq!(range1.intersect(no_intersect), None);
    /// ```
    #[inline]
    pub fn intersect(&self, other: TextRange) -> Option<TextRange> {
        let start = core::cmp::max(self.start, other.start);
        let end = core::cmp::min(self.end, other.end);

        if end < start {
            // No intersection
            None
        } else {
            // Intersection
            Some(TextRange::new(start, end))
        }
    }

    /// Compare two ranges
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// use std::cmp::Ordering;
    /// 
    /// let range1 = TextRange::new(0.into(), 20.into());
    /// let range2 = TextRange::new(10.into(), 30.into());
    /// 
    /// assert_eq!(range1.ordering(range2), Ordering::Less);
    /// assert_eq!(range2.ordering(range1), Ordering::Greater);
    /// assert_eq!(range1.ordering(range1), Ordering::Equal);
    /// ```
    #[inline]
    pub const fn ordering(self, other: TextRange) -> Ordering {
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
impl TextRange {

    /// Creates a new [`TextRange`] that covers both given ranges
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range1 = TextRange::new(0.into(), 20.into());
    /// let range2 = TextRange::new(10.into(), 30.into());
    /// 
    /// let expected = TextRange::new(0.into(), 30.into());
    /// 
    /// assert_eq!(range1.cover(range2), expected);
    /// ```
    #[inline]
    pub fn cover(self, other: TextRange) -> Self {
        TextRange::new(
            std::cmp::min(self.start, other.start),
            std::cmp::max(self.end, other.end),
        )
    }

    /// Creates a new [`TextRange`] that covers the given offset
    /// 
    /// ## Examples
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(0.into(), 20.into());
    /// 
    /// let expected = TextRange::new(0.into(), 30.into());
    /// 
    /// assert_eq!(range.cover_offset(30.into()), expected);
    /// ```
    #[inline]
    pub fn cover_offset(self, offset: TextSize) -> Self {
        self.cover(TextRange::empty(offset))
    }

    #[inline]
    pub fn checked_add(self, offset: TextSize) -> Option<TextRange> {
        Some(TextRange {
            start: self.start.checked_add(offset)?,
            end: self.end.checked_add(offset)?,
        })
    }

    #[inline]
    pub fn checked_sub(self, offset: TextSize) -> Option<TextRange> {
        Some(TextRange {
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
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(TextSize::from(5), TextSize::from(10));
    /// assert_eq!(range
    ///     .add_start(
    ///         TextSize::from(2)),
    ///         TextRange::new(
    ///             TextSize::from(7),
    ///             TextSize::from(10)
    /// ));
    /// ```
    #[inline]
    pub fn add_start(&self, amount: TextSize) -> TextRange {
        TextRange::new(self.start + amount, self.end)
    }

    /// Moves the end of the range by adding the amount to self.end
    /// 
    /// `self.end + amount`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(TextSize::from(5), TextSize::from(10));
    /// assert_eq!(range
    ///     .add_end(
    ///         TextSize::from(2)),
    ///         TextRange::new(
    ///             TextSize::from(5),
    ///             TextSize::from(12)
    /// ));
    /// ```
    #[inline]
    pub fn add_end(&self, amount: TextSize) -> TextRange {
        TextRange::new(self.start, self.end + amount)
    }

    /// Moves the start of the range by subtracting the amount to self.start
    /// 
    /// `self.start - amount`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(TextSize::from(5), TextSize::from(10));
    /// assert_eq!(range
    ///     .sub_start(
    ///         TextSize::from(2)),
    ///         TextRange::new(
    ///             TextSize::from(3),
    ///             TextSize::from(10)
    /// ));
    /// ```
    #[inline]
    pub fn sub_start(&self, amount: TextSize) -> TextRange {
        TextRange::new(self.start - amount, self.end)
    }

    /// Moves the end of the range by subtracting the amount to self.end
    /// 
    /// `self.end - amount`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use luoxide_text::traits::Ranged;
    /// use luoxide_text::range::TextRange;
    /// use luoxide_text::size::TextSize;
    /// 
    /// let range = TextRange::new(TextSize::from(5), TextSize::from(10));
    /// assert_eq!(range
    ///     .sub_end(
    ///         TextSize::from(2)),
    ///         TextRange::new(
    ///             TextSize::from(5),
    ///             TextSize::from(8)
    /// ));
    /// ```
    #[inline]
    pub fn sub_end(&self, amount: TextSize) -> TextRange {
        TextRange::new(self.start, self.end - amount)
    }
}

impl Index<TextRange> for str {
    type Output = str;
    #[inline]
    fn index(&self, index: TextRange) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

impl Index<TextRange> for String {
    type Output = str;
    #[inline]
    fn index(&self, index: TextRange) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

impl IndexMut<TextRange> for str {
    #[inline]
    fn index_mut(&mut self, index: TextRange) -> &mut Self::Output {
        &mut self[Range::<usize>::from(index)]
    }
}

impl IndexMut<TextRange> for String {
    #[inline]
    fn index_mut(&mut self, index: TextRange) -> &mut Self::Output {
        &mut self[Range::<usize>::from(index)]
    }
}

impl RangeBounds<TextSize> for TextRange {
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
    use std::ops::{Add, AddAssign, Sub, SubAssign};
    use super::*;
    operator!(impl Add for TextRange by fn add = + start,end);
    operator!(impl Sub for TextRange by fn sub = - start,end);

    impl Add<TextSize> for TextRange {
        type Output = TextRange;
    
        fn add(self, rhs: TextSize) -> Self::Output {
            self.checked_add(rhs).expect("TextRange + offset overflowed")
        }
    }

    impl Sub<TextSize> for TextRange {
        type Output = TextRange;
    
        fn sub(self, rhs: TextSize) -> Self::Output {
            self.checked_sub(rhs).expect("TextRange - offset overflowed")
        }
    }

    impl<A> AddAssign<A> for TextRange
    where 
        TextRange: Add<A, Output = TextRange>
    {
        #[inline]
        fn add_assign(&mut self, rhs: A) {
            *self = *self + rhs;
        }
    }

    impl<A> SubAssign<A> for TextRange
    where 
        TextRange: Sub<A, Output = TextRange>
    {
        #[inline]
        fn sub_assign(&mut self, rhs: A) {
            *self = *self - rhs;
        }
    }
}

