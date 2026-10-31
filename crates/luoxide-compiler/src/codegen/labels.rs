//! Per-[`FuncState`](super::FuncState) labels and pending `goto`s.

use luoxide_text::Name;

/// Defined `::name::` and unresolved `goto` / `break` in one function.
#[derive(Debug, Default)]
pub struct LabelManager {
    pub labels: Vec<LabelEntry>,
    pub pending: Vec<GotoEntry>,
}

/// A label in this proto (`pc` is an instruction index).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LabelEntry {
    pub name: Name,
    pub pc: u32,
    /// Active locals at the label (jump-into-scope checks).
    pub nactvar: u8,
}

/// A `goto` or `break` not yet matched to a label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GotoEntry {
    pub name: Name,
    pub pc: u32,
    pub nactvar: u8,
}

impl LabelManager {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            labels: Vec::new(),
            pending: Vec::new(),
        }
    }
}
