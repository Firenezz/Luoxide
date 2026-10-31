//! Function prototype: owned compiler artifact (PUC `Proto`).
//!
//! [`compile_chunk`] borrows the session intern; the returned proto does not.

mod func;
mod labels;
mod values;

pub use func::{ActiveLocal, FuncState};
pub use labels::{GotoEntry, LabelEntry, LabelManager};

use luoxide_parser::ast::{Chunk, Statement};
use luoxide_text::Interner;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Byte cutoff matching PUC `LUAI_MAXSHORTLEN`.
pub const MAX_SHORT_LEN: usize = 40;

/// Compiled function. Nested functions live in [`p`](Self::p), not in [`k`](Self::k).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionPrototype {
    /// Debug name (`main` for a chunk, or a declaration name).
    pub name: Option<ProtoString>,
    pub linedefined: u32,
    pub lastlinedefined: u32,
    pub numparams: u8,
    pub is_vararg: bool,
    /// Register count.
    pub maxstacksize: u8,
    pub locvars: Vec<LocVar>,
    pub upvalues: Vec<UpvalDesc>,
    pub k: Vec<Constant>,
    pub p: Vec<Self>,
    pub code: Vec<u8>,
}

impl FunctionPrototype {
    /// Empty chunk proto (`maxstacksize` 2, no params).
    #[must_use]
    pub const fn chunk() -> Self {
        Self {
            name: None,
            linedefined: 0,
            lastlinedefined: 0,
            numparams: 0,
            is_vararg: true,
            maxstacksize: 2,
            locvars: Vec::new(),
            upvalues: Vec::new(),
            k: Vec::new(),
            p: Vec::new(),
            code: Vec::new(),
        }
    }
}

/// Local slot plus debug extent in `code` (instruction indices).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LocVar {
    pub name: ProtoString,
    pub slot: u8,
    pub start_pc: u32,
    pub end_pc: u32,
}

/// Link to a local or upvalue of the enclosing proto.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpvalDesc {
    /// Debug name (`_ENV` for the chunk environment).
    pub name: Option<ProtoString>,
    /// `true`: `idx` is a local slot of the parent. `false`: parent's upvalue `idx`.
    pub in_stack: bool,
    pub idx: u8,
}

/// Entries of [`FunctionPrototype::k`].
///
/// Tables and closures are not constants: they are built at runtime.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Constant {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(ProtoString),
}

/// String payload stored on a proto (copied out of the session intern).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProtoString {
    Short(String),
    Long(Box<str>),
}

#[cfg(feature = "serde")]
impl Serialize for ProtoString {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ProtoString {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Ok(Self::from_text(&text))
    }
}

impl ProtoString {
    /// Classifies `text` with [`MAX_SHORT_LEN`].
    #[must_use]
    pub fn from_text(text: &str) -> Self {
        if text.len() <= MAX_SHORT_LEN {
            Self::Short(text.to_owned())
        } else {
            Self::Long(Box::from(text))
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Short(s) => s,
            Self::Long(s) => s,
        }
    }
}

impl From<&str> for ProtoString {
    fn from(text: &str) -> Self {
        Self::from_text(text)
    }
}

/// Builds proto 0 for `chunk`. Locals, `k`, and `code` are not filled yet.
#[must_use]
pub fn compile_chunk(intern: &mut Interner, chunk: &Chunk) -> FunctionPrototype {
    let mut fs = FuncState::chunk(intern);
    for statement in chunk.block.iter() {
        compile_statement(&mut fs, statement);
    }
    fs.finish()
}

#[allow(clippy::missing_const_for_fn)]
fn compile_statement(_fs: &mut FuncState, _statement: &Statement) {}
