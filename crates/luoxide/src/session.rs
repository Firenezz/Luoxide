//! One compile session: intern table and parse entry points.

use luoxide_parser::ast::{Chunk, DebugAst, DisplayLua, Expression};
use luoxide_parser::error::ParseError;
use luoxide_parser::outcome::Outcome;
use luoxide_parser::parser;
use luoxide_text::Interner;

/// Per-session state for parsing Lua.
///
/// Owns the [`Interner`]. [`Atom`](luoxide_text::Atom)s are valid only for the
/// session that produced them. `Session` is neither `Send` nor `Sync`.
#[derive(Debug, Default)]
pub struct Session {
    intern: Interner,
}

impl Session {
    /// Empty session with a fresh intern.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Shared intern table for this session.
    #[must_use]
    pub fn intern(&self) -> &Interner {
        &self.intern
    }

    /// Mutable intern table (insert names or string values).
    pub fn intern_mut(&mut self) -> &mut Interner {
        &mut self.intern
    }

    /// Parses `text` as a Lua chunk.
    pub fn parse_chunk(&mut self, text: &str) -> Outcome<Chunk, Vec<ParseError>> {
        parser::compile_chunk(&mut self.intern, text)
    }

    /// Parses `text` as a single Lua expression.
    pub fn parse_expression(&mut self, text: &str) -> Outcome<Expression, Vec<ParseError>> {
        parser::compile_expression(&mut self.intern, text)
    }

    /// Reconstructs Lua source for `node`.
    ///
    /// `source` is used only for error-recovery snippets.
    pub fn display<'a, T: ?Sized>(&'a self, node: &'a T, source: &'a str) -> DisplayLua<'a, T> {
        DisplayLua::with_source(node, &self.intern, source)
    }

    /// `Debug` dump of `node` with interned names resolved to spellings.
    pub fn debug_ast<'a, T: ?Sized>(&'a self, node: &'a T) -> DebugAst<'a, T> {
        DebugAst::new(node, &self.intern)
    }
}

#[cfg(test)]
mod tests {
    use luoxide_parser::outcome::Outcome;

    use super::Session;

    #[test]
    fn parse_and_display_round_trip() {
        let mut session = Session::new();
        let source = "local x = 1 + 2";
        let Outcome::Ok(chunk) = session.parse_chunk(source) else {
            panic!("expected a clean parse");
        };
        assert_eq!(session.display(&chunk, source).to_string(), source);
    }

    #[test]
    fn sessions_intern_independently() {
        let mut a = Session::new();
        let mut b = Session::new();
        b.intern_mut().intern("offset");

        let atom_a = a.intern_mut().intern("print");
        let atom_b = b.intern_mut().intern("print");
        assert_ne!(atom_a, atom_b, "atoms are session-local ids");
    }
}
