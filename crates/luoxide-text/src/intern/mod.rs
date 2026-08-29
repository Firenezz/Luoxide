//! Session-scoped string intern.
//!
//! [`Interner`] maps `&str` to compact [`Atom`] handles. [`Name`] and [`Str`]
//! wrap [`Atom`] so identifier spellings and string values are distinct types.
//!
//! Handles are valid only for the intern that produced them. [`Interner`] is
//! neither `Send` nor `Sync`.
//!
//! Backends: [`Buffered`] (default, `string-interner` buckets) and [`Simple`]
//! (`HashMap`).

mod buffered;
mod simple;

use std::fmt;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::rc::Rc;

pub use buffered::Buffered;
pub use simple::Simple;

/// Compact handle to a string stored in an [`Interner`].
///
/// `Copy`, one word, with a niche (`Option<Atom>` is the same size). Valid only
/// for the intern that created it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Atom(NonZeroU32);

impl Atom {
    /// Zero-based index, dense in intern order.
    #[inline]
    #[must_use]
    pub const fn index(self) -> usize {
        self.0.get() as usize - 1
    }

    /// [`Atom`] for a zero-based intern index.
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
/// Distinct from [`Str`]. Equality is by intern id.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Name(Atom);

impl Name {
    /// Wraps an interned spelling as a [`Name`].
    #[inline]
    #[must_use]
    pub const fn from_atom(atom: Atom) -> Self {
        Self(atom)
    }

    /// Underlying intern handle.
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

/// Interned string value (literals and similar).
///
/// Distinct from [`Name`]. The identifier `foo` and the literal `"foo"` use
/// different wrappers even when they share spelling.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Str(Atom);

impl Str {
    /// Wraps an interned spelling as a [`Str`].
    #[inline]
    #[must_use]
    pub const fn from_atom(atom: Atom) -> Self {
        Self(atom)
    }

    /// Underlying intern handle.
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

/// Storage backend for an [`Interner`].
///
/// Ids are dense and stable: first intern is `Atom(1)`, then `Atom(2)`, and so
/// on. Ids are never reused.
pub trait InternBackend {
    /// Get-or-insert `text`.
    fn intern(&mut self, text: &str) -> Atom;
    /// Get-or-insert `'static` text.
    ///
    /// [`Buffered`] may store the pointer without copying. Default: [`intern`](Self::intern).
    fn intern_static(&mut self, text: &'static str) -> Atom {
        self.intern(text)
    }
    /// Existing [`Atom`] for `text`, if already interned.
    fn lookup(&self, text: &str) -> Option<Atom>;
    /// Spelling for `atom`, if it belongs to this table.
    fn resolve(&self, atom: Atom) -> Option<&str>;
    /// Number of unique interned strings.
    fn len(&self) -> usize;
    /// Whether [`len`](Self::len) is zero.
    fn is_empty(&self) -> bool;
    /// All interned pairs, in intern order.
    fn interned(&self) -> impl Iterator<Item = (Atom, &str)> + '_;
}

/// Deduplicating string table (`&str` in, [`Atom`] out).
///
/// Default backend is [`Buffered`]. Use [`SimpleInterner`] for a `HashMap`
/// table. Neither `Send` nor `Sync`.
pub struct Interner<B: InternBackend = Buffered> {
    backend: B,
    _not_threaded: PhantomData<Rc<()>>,
}

/// [`Interner`] with the [`Simple`] backend.
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
    /// Empty intern table.
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
    /// Returns the existing [`Atom`] for `text`, or inserts a copy.
    pub fn intern(&mut self, text: &str) -> Atom {
        self.backend.intern(text)
    }

    /// [`intern`](Self::intern) for `'static` text. [`Buffered`] may avoid a copy.
    pub fn intern_static(&mut self, text: &'static str) -> Atom {
        self.backend.intern_static(text)
    }

    /// Interns `text` as a [`Name`].
    pub fn intern_name(&mut self, text: &str) -> Name {
        Name::from_atom(self.intern(text))
    }

    /// Interns `'static` text as a [`Name`].
    pub fn intern_name_static(&mut self, text: &'static str) -> Name {
        Name::from_atom(self.intern_static(text))
    }

    /// Interns `text` as a [`Str`].
    pub fn intern_str(&mut self, text: &str) -> Str {
        Str::from_atom(self.intern(text))
    }

    /// Interns `'static` text as a [`Str`].
    pub fn intern_str_static(&mut self, text: &'static str) -> Str {
        Str::from_atom(self.intern_static(text))
    }

    /// [`Atom`] for `text` if already interned.
    #[must_use]
    pub fn lookup(&self, text: &str) -> Option<Atom> {
        self.backend.lookup(text)
    }

    /// Spelling for `atom` (or a [`Name`] / [`Str`]) in this table.
    ///
    /// `None` if the handle is out of range for this intern.
    #[must_use]
    pub fn get(&self, atom: impl Into<Atom>) -> Option<&str> {
        self.backend.resolve(atom.into())
    }

    /// Number of unique interned strings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.backend.len()
    }

    /// Whether no strings have been interned.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.backend.is_empty()
    }

    /// Rewrites `Atom(id)` in a `Debug` string to `Atom("spelling")`.
    ///
    /// Highest ids first so `Atom(10)` is not treated as a prefix of `Atom(1)`.
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

    #[test]
    fn buffered_static_intern_matches_dynamic() {
        let mut intern = Interner::<Buffered>::new();
        let static_atom = intern.intern_static("print");
        let copied = intern.intern("print");
        assert_eq!(static_atom, copied);
        assert_eq!(intern.get(static_atom), Some("print"));
        assert_eq!(intern.len(), 1);
    }
}
