//! Session-scoped string interning.
//!
//! One [`Interner`] belongs to one compile session (later: one machine). An
//! [`Atom`] is interned **spelling** in that table — not a scoped binding.
//! [`Name`] and [`Str`] keep identifier spelling and string values distinct.
//!
//! Two backends share this API:
//! - [`Buffered`] (default): `string-interner` contiguous storage
//! - [`Simple`]: `HashMap` + `Vec`

mod buffered;
mod simple;

use std::fmt;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::rc::Rc;

pub use buffered::Buffered;
pub use simple::Simple;

/// Handle to an interned string.
///
/// `Copy`, one word, with a niche so `Option<Atom>` is the same size. Only
/// meaningful together with the [`Interner`] that produced it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Atom(NonZeroU32);

impl Atom {
    /// Index into side tables (0-based, dense in intern order).
    #[inline]
    #[must_use]
    pub const fn index(self) -> usize {
        self.0.get() as usize - 1
    }

    /// Builds an [`Atom`] from a 0-based intern index.
    #[inline]
    #[must_use]
    pub(crate) fn from_index(index: usize) -> Option<Self> {
        let id = u32::try_from(index).ok()?.checked_add(1)?;
        NonZeroU32::new(id).map(Self)
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Atom({})", self.0.get())
    }
}

/// Interned identifier spelling (variables, fields, labels).
///
/// Not a Lua binding and not a string value. Use [`Str`] for literals.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Name(Atom);

impl Name {
    #[inline]
    #[must_use]
    pub const fn from_atom(atom: Atom) -> Self {
        Self(atom)
    }

    #[inline]
    #[must_use]
    pub const fn atom(self) -> Atom {
        self.0
    }
}

impl From<Name> for Atom {
    #[inline]
    fn from(name: Name) -> Self {
        name.0
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Interned string value (literals, later proto constants).
///
/// Distinct from [`Name`] so `"foo"` and the identifier `foo` cannot share a
/// handle even when they share spelling.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Str(Atom);

impl Str {
    #[inline]
    #[must_use]
    pub const fn from_atom(atom: Atom) -> Self {
        Self(atom)
    }

    #[inline]
    #[must_use]
    pub const fn atom(self) -> Atom {
        self.0
    }
}

impl From<Str> for Atom {
    #[inline]
    fn from(string: Str) -> Self {
        string.0
    }
}

impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Storage for an [`Interner`].
///
/// Implementations must assign dense, stable ids: the first interned string
/// is `Atom(1)`, then `Atom(2)`, and so on. Ids are never reused.
pub trait InternBackend {
    fn intern(&mut self, text: &str) -> Atom;
    fn lookup(&self, text: &str) -> Option<Atom>;
    fn resolve(&self, atom: Atom) -> Option<&str>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn interned(&self) -> impl Iterator<Item = (Atom, &str)> + '_;
}

/// Intern table: full-string deduplication, `&str` in, [`Atom`] out.
///
/// Defaults to [`Buffered`] (`string-interner`). Use [`SimpleInterner`] for a
/// HashMap table. `intern` takes `&mut self`. The type is `!Sync` / `!Send`
/// so every machine/session owns its own table instead of sharing one.
pub struct Interner<B: InternBackend = Buffered> {
    backend: B,
    _not_threaded: PhantomData<Rc<()>>,
}

/// HashMap intern — same [`Atom`] / [`Name`] / [`Str`] API as [`Interner`].
pub type SimpleInterner = Interner<Simple>;

impl<B: InternBackend + Default> Default for Interner<B> {
    fn default() -> Self {
        Self {
            backend: B::default(),
            _not_threaded: PhantomData,
        }
    }
}

impl<B: InternBackend + Default> Interner<B> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<B: InternBackend> fmt::Debug for Interner<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interner")
            .field("len", &self.len())
            .finish()
    }
}

impl<B: InternBackend> Interner<B> {
    /// Get-or-insert: returns the existing [`Atom`] for `text` or copies the
    /// bytes into the table.
    pub fn intern(&mut self, text: &str) -> Atom {
        self.backend.intern(text)
    }

    /// Intern as an identifier spelling.
    pub fn intern_name(&mut self, text: &str) -> Name {
        Name::from_atom(self.intern(text))
    }

    /// Intern as a string value.
    pub fn intern_str(&mut self, text: &str) -> Str {
        Str::from_atom(self.intern(text))
    }

    /// Returns the [`Atom`] for `text` without inserting it.
    #[must_use]
    pub fn lookup(&self, text: &str) -> Option<Atom> {
        self.backend.lookup(text)
    }

    /// Resolves an [`Atom`] (or a [`Name`] / [`Str`]) back to its spelling.
    ///
    /// Returns `None` for a handle from a different interner whose id is out
    /// of range (ids from other interners that happen to be in range resolve
    /// to the wrong name — do not mix interners).
    #[must_use]
    pub fn get(&self, atom: impl Into<Atom>) -> Option<&str> {
        self.backend.resolve(atom.into())
    }

    /// Number of uniquely interned strings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.backend.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.backend.is_empty()
    }

    /// Rewrites `Atom(id)` placeholders in a [`Debug`] string into
    /// `Atom("spelling")` using this table. Replaces highest ids first so
    /// `Atom(10)` is not corrupted when resolving `Atom(1)`.
    #[must_use]
    pub fn annotate_debug_atoms(&self, debug: &str) -> String {
        let mut ids: Vec<(u32, &str)> = self
            .backend
            .interned()
            .map(|(atom, name)| (atom.0.get(), name))
            .collect();
        ids.sort_by_key(|(id, _)| std::cmp::Reverse(*id));
        let mut out = debug.to_string();
        for (id, name) in ids {
            out = out.replace(&format!("Atom({id})"), &format!("Atom({name:?})"));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dedupes_and_resolves<B: InternBackend + Default>() {
        let mut intern = Interner::<B>::new();
        let a = intern.intern("foo");
        let b = intern.intern("bar");
        let c = intern.intern("foo");

        assert_eq!(a, c);
        assert_ne!(a, b);
        assert_eq!(intern.get(a), Some("foo"));
        assert_eq!(intern.get(b), Some("bar"));
        assert_eq!(intern.len(), 2);
    }

    fn lookup_does_not_insert<B: InternBackend + Default>() {
        let mut intern = Interner::<B>::new();
        assert_eq!(intern.lookup("x"), None);
        let atom = intern.intern("x");
        assert_eq!(intern.lookup("x"), Some(atom));
        assert_eq!(intern.len(), 1);
    }

    fn names_and_strs_share_spelling<B: InternBackend + Default>() {
        let mut intern = Interner::<B>::new();
        let name = intern.intern_name("foo");
        let string = intern.intern_str("foo");
        assert_eq!(name.atom(), string.atom());
        assert_eq!(intern.get(name), Some("foo"));
        assert_eq!(intern.get(string), Some("foo"));
    }

    fn annotate_debug_atoms_resolves_spellings<B: InternBackend + Default>() {
        let mut intern = Interner::<B>::new();
        let a = intern.intern("simple");
        let b = intern.intern("add");
        let raw = format!("Identifier {{ name: {a:?}, span: 0..1 }}");
        assert_eq!(
            intern.annotate_debug_atoms(&raw),
            r#"Identifier { name: Atom("simple"), span: 0..1 }"#
        );
        let raw = format!("{a:?} then {b:?}");
        assert_eq!(
            intern.annotate_debug_atoms(&raw),
            r#"Atom("simple") then Atom("add")"#
        );
    }

    #[test]
    fn buffered_dedupes_and_resolves() {
        dedupes_and_resolves::<Buffered>();
    }

    #[test]
    fn simple_dedupes_and_resolves() {
        dedupes_and_resolves::<Simple>();
    }

    #[test]
    fn buffered_lookup_does_not_insert() {
        lookup_does_not_insert::<Buffered>();
    }

    #[test]
    fn simple_lookup_does_not_insert() {
        lookup_does_not_insert::<Simple>();
    }

    #[test]
    fn buffered_names_and_strs() {
        names_and_strs_share_spelling::<Buffered>();
    }

    #[test]
    fn simple_names_and_strs() {
        names_and_strs_share_spelling::<Simple>();
    }

    #[test]
    fn buffered_annotate_debug() {
        annotate_debug_atoms_resolves_spellings::<Buffered>();
    }

    #[test]
    fn simple_annotate_debug() {
        annotate_debug_atoms_resolves_spellings::<Simple>();
    }

    #[test]
    fn option_atom_is_one_word() {
        assert_eq!(size_of::<Option<Atom>>(), size_of::<Atom>());
        assert_eq!(size_of::<Option<Name>>(), size_of::<Name>());
        assert_eq!(size_of::<Option<Str>>(), size_of::<Str>());
    }
}
