//! The compile session: one per machine/compilation unit.

use luoxide_parser::ast::{Chunk, DebugAst, DisplayLua, Expression};
use luoxide_parser::error::ParseError;
use luoxide_parser::outcome::Outcome;
use luoxide_parser::parser;
use luoxide_text::Interner;

/// Owns everything scoped to one Lua machine or compilation unit — today
/// that is the [`Interner`] table; later passes hang their state here too.
///
/// Sessions are independent: [`Atom`](luoxide_text::Atom)s from one session
/// are meaningless in another, and the type is `!Sync`, so parallel machines
/// in the same process each own their own `Session` instead of sharing one
/// behind a lock.
#[derive(Debug, Default)]
pub struct Session {
    intern: Interner,
}

impl Session {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn intern(&self) -> &Interner {
        &self.intern
    }

    /// Host/FFI ingress: `session.intern_mut().intern("print")` — bytes in,
    /// [`Atom`](luoxide_text::Atom) only inside this session.
    pub fn intern_mut(&mut self) -> &mut Interner {
        &mut self.intern
    }

    /// Parses a whole source file into this session.
    pub fn parse_chunk(&mut self, text: &str) -> Outcome<Chunk, Vec<ParseError>> {
        parser::compile_chunk(&mut self.intern, text)
    }

    /// Parses a single expression (mostly useful for tests and tooling).
    pub fn parse_expression(&mut self, text: &str) -> Outcome<Expression, Vec<ParseError>> {
        parser::compile_expression(&mut self.intern, text)
    }

    /// Renders `node` as Lua source using this session's intern. `source` is
    /// only consulted for error-node snippets.
    pub fn display<'a, T: ?Sized>(&'a self, node: &'a T, source: &'a str) -> DisplayLua<'a, T> {
        DisplayLua::with_source(node, &self.intern, source)
    }

    /// Pretty-prints `node` with identifier [`Atom`](luoxide_text::Atom)s
    /// resolved to their spellings in this session's intern.
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
