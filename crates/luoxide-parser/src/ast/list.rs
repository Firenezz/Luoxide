//! `NodeList<T>`: the sequence type used for every list position in the AST
//! (statements in a block, call arguments, table fields, ...).
//!
//! Like [`P`](super::ptr::P), this is a wrapper so the backing storage stays an
//! implementation detail. It is currently a [`ThinVec`] (a single pointer wide,
//! which keeps node enums small); it can later be swapped for arena-allocated
//! slices without touching node definitions or the parser.

use core::fmt;
use std::ops::{Deref, DerefMut};

use thin_vec::ThinVec;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NodeList<T> {
    items: ThinVec<T>,
}

impl<T> NodeList<T> {
    /// Creates an empty list without allocating.
    #[inline]
    pub fn new() -> Self {
        Self {
            items: ThinVec::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: ThinVec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

impl<T> Default for NodeList<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for NodeList<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        &self.items
    }
}

impl<T> DerefMut for NodeList<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.items
    }
}

impl<T> FromIterator<T> for NodeList<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            items: ThinVec::from_iter(iter),
        }
    }
}

impl<T> Extend<T> for NodeList<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter);
    }
}

impl<T> IntoIterator for NodeList<T> {
    type Item = T;
    type IntoIter = thin_vec::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a NodeList<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NodeList<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl<T> From<ThinVec<T>> for NodeList<T> {
    #[inline]
    fn from(items: ThinVec<T>) -> Self {
        Self { items }
    }
}

impl<T> From<Vec<T>> for NodeList<T> {
    #[inline]
    fn from(items: Vec<T>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl<T: Clone> Clone for NodeList<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for NodeList<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl<T: Eq> Eq for NodeList<T> {}

impl<T: fmt::Debug> fmt::Debug for NodeList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.items.iter()).finish()
    }
}

// An empty NodeList must stay a single pointer wide so node enums that embed
// one (e.g. a table constructor's field list) stay small.
const _: () = assert!(size_of::<NodeList<u64>>() == size_of::<usize>());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_and_iterate() {
        let list: NodeList<i32> = (0..3).collect();
        assert_eq!(list.len(), 3);
        assert_eq!(list.iter().sum::<i32>(), 3);
    }

    #[test]
    fn push_and_slice_access() {
        let mut list = NodeList::new();
        list.push("a");
        list.push("b");
        assert_eq!(&list[..], ["a", "b"]);
    }
}
