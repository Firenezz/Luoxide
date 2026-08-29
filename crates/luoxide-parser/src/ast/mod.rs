//! Lua abstract syntax tree.
//!
//! Recursive children are [`P`]; sequences are [`NodeList`]. Nodes carry
//! [`TextSpan`]. Identifier spellings are [`Name`]s, resolved through the
//! session intern.

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

/// Source identifier: interned [`Name`] plus [`TextSpan`].
///
/// Resolving `name` to text requires the session [`Interner`](luoxide_text::Interner)
/// (see [`DisplayLua`]).
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
