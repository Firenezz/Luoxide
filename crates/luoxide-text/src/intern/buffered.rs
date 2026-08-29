//! Default intern backend (`string-interner` [`BucketBackend`]).
//!
//! Resolve is O(1). `'static` intern may skip a copy.

use ahash::RandomState;
use string_interner::StringInterner;
use string_interner::backend::BucketBackend;
use string_interner::symbol::Symbol;

use super::{Atom, InternBackend};

impl Symbol for Atom {
    fn try_from_usize(index: usize) -> Option<Self> {
        Self::from_index(index)
    }

    fn to_usize(self) -> usize {
        self.index()
    }
}

type Backend = BucketBackend<Atom>;

/// [`super::Interner`] backend using `string-interner` buckets.
#[derive(Debug)]
pub struct Buffered {
    inner: StringInterner<Backend, RandomState>,
}

impl Default for Buffered {
    fn default() -> Self {
        Self {
            inner: StringInterner::with_hasher(RandomState::new()),
        }
    }
}

impl InternBackend for Buffered {
    fn intern(&mut self, text: &str) -> Atom {
        self.inner.get_or_intern(text)
    }

    fn intern_static(&mut self, text: &'static str) -> Atom {
        self.inner.get_or_intern_static(text)
    }

    fn lookup(&self, text: &str) -> Option<Atom> {
        self.inner.get(text)
    }

    fn resolve(&self, atom: Atom) -> Option<&str> {
        self.inner.resolve(atom)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn interned(&self) -> impl Iterator<Item = (Atom, &str)> + '_ {
        self.inner.iter()
    }
}
