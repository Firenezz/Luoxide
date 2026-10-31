//! JSON dump of an AST node with the session string table.
//!
//! [`Name`](luoxide_text::Name) / [`Str`](luoxide_text::Str) / [`Atom`](luoxide_text::Atom)
//! serialize as 1-based intern ids. Spelling for id `n` is `strings[n - 1]`.

use luoxide_text::Interner;
use serde::Serialize;

/// AST plus the intern table needed to resolve atom ids in JSON.
#[derive(Serialize)]
pub struct JsonAst<'a, T: ?Sized> {
    /// Intern spellings in id order (`strings[atom.index()]`).
    pub strings: Vec<&'a str>,
    pub ast: &'a T,
}

impl<'a, T: ?Sized> JsonAst<'a, T> {
    #[must_use]
    pub fn new(node: &'a T, intern: &'a Interner) -> Self {
        let strings = intern.interned().map(|(_, text)| text).collect();
        Self { strings, ast: node }
    }
}
