//! `P<T>`: the owned pointer type used for every recursive position in the AST.
//!
//! The AST never uses `Box<T>` directly. Routing all node indirection through
//! `P<T>` keeps the allocation strategy private to this module, so it can later
//! be backed by an arena (e.g. bumpalo) or carry extra metadata without
//! touching node definitions or the parser.

use core::fmt;
use std::ops::{Deref, DerefMut};

pub struct P<T: ?Sized> {
    ptr: Box<T>,
}

/// Shorthand constructor: `P(expr)` instead of `P::new(expr)`.
#[allow(non_snake_case)]
#[inline]
pub fn P<T>(value: T) -> P<T> {
    P::new(value)
}

impl<T> P<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            ptr: Box::new(value),
        }
    }

    /// Moves the value out of the pointer, releasing the allocation.
    #[inline]
    pub fn into_inner(self) -> T {
        *self.ptr
    }

    /// Transforms the pointed-to value in place, reusing the allocation.
    #[inline]
    pub fn map<F: FnOnce(T) -> T>(mut self, f: F) -> Self {
        // Replace-with-read pattern: `f` must not panic between the read and
        // the write or the old value would be dropped twice. We guard by
        // aborting via a bomb if `f` unwinds.
        struct AbortOnPanic;
        impl Drop for AbortOnPanic {
            fn drop(&mut self) {
                std::process::abort();
            }
        }

        unsafe {
            let slot: *mut T = &mut *self.ptr;
            let bomb = AbortOnPanic;
            let value = std::ptr::read(slot);
            let value = f(value);
            std::ptr::write(slot, value);
            std::mem::forget(bomb);
        }
        self
    }
}

impl<T: ?Sized> Deref for P<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.ptr
    }
}

impl<T: ?Sized> DerefMut for P<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.ptr
    }
}

impl<T: ?Sized> AsRef<T> for P<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> From<T> for P<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Clone> Clone for P<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new((**self).clone())
    }
}

impl<T: ?Sized + PartialEq> PartialEq for P<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: ?Sized + Eq> Eq for P<T> {}

impl<T: ?Sized + fmt::Debug> fmt::Debug for P<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for P<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_and_into_inner() {
        let p = P(41);
        assert_eq!(*p, 41);
        assert_eq!(p.into_inner(), 41);
    }

    #[test]
    fn map_in_place() {
        let p = P(String::from("a")).map(|mut s| {
            s.push('b');
            s
        });
        assert_eq!(*p, "ab");
    }
}
