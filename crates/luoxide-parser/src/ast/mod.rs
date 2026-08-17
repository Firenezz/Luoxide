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

use ecow::EcoString;
use luoxide_text::range::TextSpan;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use display::DisplayLua;
pub use expressions::{
    BinaryOp, Expression, ExpressionKind, Field, FieldKind, FunctionBody, Literal, MethodCall,
    UnaryOp,
};
pub use list::NodeList;
pub use node_id::{NodeId, NodeIdGenerator};
pub use ptr::P;
pub use statements::{
    Assign, AttributedName, Block, Chunk, FunctionDecl, FunctionName, GenericFor, Global, IfArm,
    IfStatement, Local, LocalFunction, NumericFor, Repeat, Statement, StatementKind, While,
};

/// A name in the source code, together with its location.
///
/// The name is stored as an [`EcoString`]: cloning is cheap and names of up to
/// 15 bytes are stored inline without a heap allocation. Once the interner
/// exists, this will shrink to a [`Symbol`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier {
    pub name: EcoString,
    pub span: TextSpan,
}

impl Identifier {
    #[inline]
    pub fn new(name: impl Into<EcoString>, span: TextSpan) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    #[inline]
    pub fn string(string: impl Into<EcoString>) -> Self {
        Self {
            name: string.into(),
            span: TextSpan::default(),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

/// Handle to an interned string.
///
/// Identifier names and string literals will be interned; a `Symbol` is the
/// cheap, `Copy` reference into the interner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub id: u32,
}
