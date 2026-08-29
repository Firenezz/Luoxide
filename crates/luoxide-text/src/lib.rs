//! Source spans and session-scoped string interning.

#[macro_use]
mod macros;

pub mod intern;
pub mod range;
pub mod size;
pub mod source;
pub mod traits;

pub use intern::{Atom, InternBackend, Interner, Name, Simple, SimpleInterner, Str};
