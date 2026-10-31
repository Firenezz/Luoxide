//! Working state for one function. The chunk is proto 0 (`previous` is [`None`]).

use luoxide_text::{Interner, Name};

use super::{FunctionPrototype, LabelManager, ProtoString, UpvalDesc};

/// Local that is in scope (slot is an index into [`FunctionPrototype::locvars`] later).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveLocal {
    pub name: Name,
    pub slot: u8,
}

/// PUC-style `FuncState`: proto under construction plus scope / labels.
///
/// Nested functions box the parent in [`previous`](Self::previous). The intern
/// is not stored here (`&mut Interner` cannot be shared along that chain).
#[derive(Debug)]
pub struct FuncState {
    pub proto: FunctionPrototype,
    pub previous: Option<Box<Self>>,
    pub locals: Vec<ActiveLocal>,
    /// 5.5 `global` names. Meaningful on proto 0; children walk [`previous`](Self::previous).
    pub globals: Vec<Name>,
    pub labels: LabelManager,
    pub freereg: u8,
}

impl FuncState {
    /// Main chunk: vararg proto, `_ENV` as upvalue 0.
    pub fn chunk(intern: &mut Interner) -> Self {
        let env = intern.intern_name("_ENV");
        let mut proto = FunctionPrototype::chunk();
        proto.name = Some(ProtoString::from_text("__chunk"));
        proto.upvalues.push(UpvalDesc {
            name: intern.get(env).map(ProtoString::from_text),
            in_stack: false,
            idx: 0,
        });
        Self {
            proto,
            previous: None,
            locals: Vec::new(),
            globals: Vec::new(),
            labels: LabelManager::new(),
            freereg: 0,
        }
    }

    /// Nested function; `parent` becomes [`previous`](Self::previous).
    #[must_use]
    pub fn function(parent: Self, numparams: u8, is_vararg: bool) -> Self {
        let mut proto = FunctionPrototype::chunk();
        proto.numparams = numparams;
        proto.is_vararg = is_vararg;
        proto.name = None;
        Self {
            proto,
            previous: Some(Box::new(parent)),
            locals: Vec::new(),
            globals: Vec::new(),
            labels: LabelManager::new(),
            freereg: numparams,
        }
    }

    /// Finished proto (chunk, or after the caller has used [`close_function`](Self::close_function)).
    #[must_use]
    pub fn finish(self) -> FunctionPrototype {
        self.proto
    }

    /// Append this proto to the parent’s `p` and return the parent.
    #[must_use]
    pub fn close_function(self) -> Option<Self> {
        let proto = self.proto;
        let mut parent = *self.previous?;
        parent.proto.p.push(proto);
        Some(parent)
    }
}

#[cfg(test)]
mod tests {
    use luoxide_text::Interner;

    use super::FuncState;

    #[test]
    fn chunk_is_proto_zero_with_env_upvalue() {
        let mut intern = Interner::new();
        let fs = FuncState::chunk(&mut intern);
        assert!(fs.previous.is_none());
        assert!(fs.proto.is_vararg);
        assert!(matches!(
            fs.proto.upvalues.as_slice(),
            [uv] if uv.name.as_ref().map(super::ProtoString::as_str) == Some("_ENV")
                && !uv.in_stack
        ));
    }

    #[test]
    fn nested_function_returns_to_parent() {
        let mut intern = Interner::new();
        let parent = FuncState::chunk(&mut intern);
        let child = FuncState::function(parent, 1, false);
        assert!(child.previous.is_some());
        assert_eq!(child.proto.numparams, 1);
        assert!(!child.proto.is_vararg);
        assert_eq!(
            child.close_function().map(|parent| parent
                .proto
                .p
                .first()
                .map(|nested| nested.numparams)),
            Some(Some(1))
        );
    }
}
