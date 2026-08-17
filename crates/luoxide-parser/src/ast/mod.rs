//! The Lua abstract syntax tree.
//!
//! Design notes:
//!
//! - Every recursive position uses [`P`] and every sequence uses [`NodeList`].
//!   Both wrap their backing storage so the allocation strategy (currently
//!   `Box`/`ThinVec`, later possibly an arena) stays an implementation detail.
//! - Every node carries a [`TextSpan`] so diagnostics can always point at
//!   source code.
//! - Node sizes are guarded by compile-time asserts in the submodules; growing
//!   a node past its budget is a deliberate decision, not an accident.

pub mod display;
pub mod expressions;
pub mod list;
pub mod node_id;
pub mod ptr;
pub mod statements;

use luoxide_text::Name;
use luoxide_text::range::TextSpan;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use display::{DebugAst, DisplayLua};
pub use expressions::{
    BinaryOp, Expression, ExpressionKind, Field, FieldKind, FunctionBody, Literal, MethodCall,
    Param, UnaryOp, VarargsParam,
};
pub use list::NodeList;
pub use node_id::{NodeId, NodeIdGenerator};
pub use ptr::P;
pub use statements::{
    Assign, AttributedName, Block, Chunk, FunctionDecl, FunctionName, FunctionScope, GenericFor,
    Global, IfArm, IfStatement, Local, NumericFor, Repeat, Statement, StatementKind, While,
};

/// A name in the source code, together with its location.
///
/// The name is a [`Name`]: interned identifier spelling in the session's
/// [`Interner`](luoxide_text::Interner). Resolving it back to text requires
/// that same intern (see [`DisplayLua`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier {
    pub name: Name,
    pub span: TextSpan,
}

impl Identifier {
    #[inline]
    pub fn new(name: Name, span: TextSpan) -> Self {
        Self { name, span }
    }
}
