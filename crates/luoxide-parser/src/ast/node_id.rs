//! `NodeId`: a stable identity for AST nodes.
//!
//! The parser assigns each node a unique id at construction time. Later stages
//! (name resolution, diagnostics, incremental caches) can key side tables by
//! `NodeId` instead of storing that data inside the tree itself, which keeps
//! nodes small and the stages decoupled.

use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u32);

impl NodeId {
    /// Placeholder id for nodes that have not been numbered (yet).
    ///
    /// Synthetic nodes (e.g. error-recovery placeholders) may keep this id.
    pub const DUMMY: NodeId = NodeId(u32::MAX);

    #[inline]
    pub const fn from_u32(id: u32) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::DUMMY {
            write!(f, "NodeId(DUMMY)")
        } else {
            write!(f, "NodeId({})", self.0)
        }
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Hands out sequential [`NodeId`]s. One generator per parse.
#[derive(Debug, Default)]
pub struct NodeIdGenerator {
    next: u32,
}

impl NodeIdGenerator {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn next_id(&mut self) -> NodeId {
        let id = self.next;
        assert!(id != u32::MAX, "exhausted NodeId space");
        self.next += 1;
        NodeId(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_is_sequential() {
        let mut gen = NodeIdGenerator::new();
        assert_eq!(gen.next_id(), NodeId::from_u32(0));
        assert_eq!(gen.next_id(), NodeId::from_u32(1));
    }

    #[test]
    fn dummy_is_distinct() {
        let mut gen = NodeIdGenerator::new();
        assert_ne!(gen.next_id(), NodeId::DUMMY);
    }
}
