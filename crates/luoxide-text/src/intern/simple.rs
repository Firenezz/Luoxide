//! HashMap intern table.

use std::rc::Rc;

use rustc_hash::FxHashMap;

use super::{Atom, InternBackend};

/// `HashMap` plus a dense `Vec` of spellings. Same [`Atom`] numbering as
/// [`super::Buffered`].
#[derive(Debug, Default)]
pub struct Simple {
    map: FxHashMap<Rc<str>, Atom>,
    items: Vec<Rc<str>>,
}

impl InternBackend for Simple {
    fn intern(&mut self, text: &str) -> Atom {
        if let Some(&atom) = self.map.get(text) {
            return atom;
        }
        let atom = Atom::from_index(self.items.len()).expect("intern table overflowed u32");
        let entry: Rc<str> = Rc::from(text);
        self.items.push(Rc::clone(&entry));
        self.map.insert(entry, atom);
        atom
    }

    fn lookup(&self, text: &str) -> Option<Atom> {
        self.map.get(text).copied()
    }

    fn resolve(&self, atom: Atom) -> Option<&str> {
        self.items.get(atom.index()).map(|s| &**s)
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn interned(&self) -> impl Iterator<Item = (Atom, &str)> + '_ {
        self.items
            .iter()
            .enumerate()
            .map(|(i, s)| (Atom::from_index(i).expect("dense intern ids"), &**s))
    }
}
