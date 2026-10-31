//! AST to function prototypes.

pub mod codegen;

pub use codegen::{
    ActiveLocal, Constant, FuncState, FunctionPrototype, GotoEntry, LabelEntry, LabelManager,
    LocVar, MAX_SHORT_LEN, ProtoString, UpvalDesc, compile_chunk,
};
