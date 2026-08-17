//! Contiguous intern table (`string-interner`).

use string_interner::StringInterner;
use string_interner::backend::StringBackend;
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

/// One string buffer plus a map. Default [`super::Interner`] backend.
#[derive(Debug)]
pub struct Buffered {
    inner: StringInterner<StringBackend<Atom>, rustc_hash::FxBuildHasher>,
}

impl Default for Buffered {
    fn default() -> Self {
        Self {
            inner: StringInterner::with_hasher(rustc_hash::FxBuildHasher),
        }
    }
}

impl InternBackend for Buffered {
    fn intern(&mut self, text: &str) -> Atom {
        self.inner.get_or_intern(text)
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
