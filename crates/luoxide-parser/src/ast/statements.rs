//! Statement, block and chunk nodes.

use luoxide_text::range::TextSpan;

use super::expressions::{Expression, FunctionBody};
use super::{Identifier, NodeList, P};

/// The root of a parsed source file.
#[derive(Clone, Debug, PartialEq)]
pub struct Chunk {
    pub block: Block,
}

/// A sequence of statements.
///
/// The "a `return` must be the last statement" rule is enforced by the parser,
/// not by this type.
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: NodeList<Statement>,
    pub span: TextSpan,
}

impl Block {
    #[inline]
    pub fn new(statements: NodeList<Statement>, span: TextSpan) -> Block {
        Block { statements, span }
    }

    #[inline]
    pub fn empty(span: TextSpan) -> Block {
        Block {
            statements: NodeList::new(),
            span,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: TextSpan,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementKind {
    /// A function or method call used as a statement: `f(x)`.
    Expression(Expression),
    /// `a, b = 1, 2`
    Assign(P<Assign>),
    /// `local a <const>, b = 1, 2`
    Local(P<Local>),
    /// `if c then ... elseif c2 then ... else ... end`
    If(P<IfStatement>),
    /// `while c do ... end`
    While(P<While>),
    /// `repeat ... until c`
    Repeat(P<Repeat>),
    /// `for i = start, stop [, step] do ... end`
    NumericFor(P<NumericFor>),
    /// `for a, b in exprs do ... end`
    GenericFor(P<GenericFor>),
    /// `do ... end`
    Do(P<Block>),
    /// `function a.b.c:m() ... end`
    FunctionDecl(P<FunctionDecl>),
    /// `local function f() ... end`
    LocalFunction(P<LocalFunction>),
    /// `return exprs`
    Return(NodeList<Expression>),
    /// `break`
    Break,
    /// `goto label`
    Goto(Identifier),
    /// `::label::`
    Label(Identifier),
    /// Placeholder produced by error recovery; diagnostics carry the details.
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assign {
    pub targets: NodeList<Expression>,
    pub values: NodeList<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Local {
    pub names: NodeList<AttributedName>,
    pub values: NodeList<Expression>,
}

/// `name <attrib>` in a local declaration (Lua 5.4 `<const>` / `<close>`).
#[derive(Clone, Debug, PartialEq)]
pub struct AttributedName {
    pub name: Identifier,
    pub attribute: Option<Identifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
    /// The `if` arm followed by any `elseif` arms; always at least one.
    pub arms: NodeList<IfArm>,
    pub else_block: Option<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfArm {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct While {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Repeat {
    pub block: Block,
    /// The `until` condition; it can see locals declared in the block.
    pub condition: Expression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumericFor {
    pub variable: Identifier,
    pub start: Expression,
    pub stop: Expression,
    pub step: Option<Expression>,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenericFor {
    pub names: NodeList<Identifier>,
    pub exprs: NodeList<Expression>,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: FunctionName,
    pub body: FunctionBody,
}

/// The dotted path of a function declaration: `a.b.c` or `a.b:m`.
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionName {
    pub base: Identifier,
    pub path: NodeList<Identifier>,
    /// Present for `function a.b:m()`; implies an implicit `self` parameter.
    pub method: Option<Identifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalFunction {
    pub name: Identifier,
    pub body: FunctionBody,
}

impl Statement {
    #[inline]
    pub fn new(kind: StatementKind, span: TextSpan) -> Statement {
        Statement { kind, span }
    }

    /// Error-recovery placeholder covering the skipped source range.
    #[inline]
    pub fn error(span: TextSpan) -> Statement {
        Statement {
            kind: StatementKind::Error,
            span,
        }
    }
}

// Statements box their payloads except `Expression` (the most common kind,
// kept inline to avoid an indirection on every call statement).
const _: () = assert!(size_of::<Statement>() <= 64);
