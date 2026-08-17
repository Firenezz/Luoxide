//! Human-readable Lua for AST nodes, analogous to the lexer's `DisplayToken`.
//!
//! The tree itself stays a data type (`Debug` dumps structure). Wrap a node in
//! [`DisplayLua`] to print the code the parser understood, or [`DebugAst`] to
//! dump structure with identifier spellings resolved through an [`Intern`].

use core::fmt::{self, Write};

use luoxide_text::Interner;
use luoxide_text::range::TextSpan;

use crate::ast::FunctionScope;

use super::{
    Assign, AttributedName, Block, Chunk, Expression, ExpressionKind, Field, FieldKind,
    FunctionBody, FunctionDecl, GenericFor, Global, Identifier, IfArm, IfStatement, Literal, Local,
    NodeList, NumericFor, Param, Repeat, Statement, StatementKind, UnaryOp, While,
};

/// Lua source rendering of an AST node.
///
/// Same idea as `DisplayToken(token, lexeme)`: the node does not implement
/// `Display` itself. Identifier names are [`Atom`](luoxide_text::Atom)s, so
/// rendering needs the [`Intern`] that produced the tree. The optional source
/// is only used for [`ExpressionKind::Error`] / [`StatementKind::Error`] so
/// the skipped snippet can be shown; names and literals always come from the
/// tree and the intern.
pub struct DisplayLua<'a, T: ?Sized> {
    pub node: &'a T,
    pub intern: &'a Interner,
    pub source: Option<&'a str>,
}

impl<'a, T: ?Sized> DisplayLua<'a, T> {
    pub fn new(node: &'a T, intern: &'a Interner) -> Self {
        Self {
            node,
            intern,
            source: None,
        }
    }

    pub fn with_source(node: &'a T, intern: &'a Interner, source: &'a str) -> Self {
        Self {
            node,
            intern,
            source: Some(source),
        }
    }
}

/// Debug rendering of an AST node with [`Atom`](luoxide_text::Atom) spellings
/// resolved through an [`Intern`].
///
/// Wrap `{:#?}` on the raw tree and rewrite `Atom(1)` into `Atom("name")`.
pub struct DebugAst<'a, T: ?Sized> {
    pub node: &'a T,
    pub intern: &'a Interner,
}

impl<'a, T: ?Sized> DebugAst<'a, T> {
    pub fn new(node: &'a T, intern: &'a Interner) -> Self {
        Self { node, intern }
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for DebugAst<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let raw = if f.alternate() {
            format!("{:#?}", self.node)
        } else {
            format!("{:?}", self.node)
        };
        f.write_str(&self.intern.annotate_debug_atoms(&raw))
    }
}

impl<'a, T: ?Sized> fmt::Debug for DisplayLua<'a, T>
where
    Self: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for DisplayLua<'_, Chunk> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Printer::new(f, self.intern, self.source).write_block(&self.node.block, 0)?;
        Ok(())
    }
}

impl fmt::Display for DisplayLua<'_, Block> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Printer::new(f, self.intern, self.source).write_block(self.node, 0)
    }
}

impl fmt::Display for DisplayLua<'_, Statement> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Printer::new(f, self.intern, self.source).write_statement(self.node, 0)
    }
}

impl fmt::Display for DisplayLua<'_, Expression> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Printer::new(f, self.intern, self.source).write_expr(self.node, 0, 0)
    }
}

struct Printer<'a, 'b> {
    f: &'a mut fmt::Formatter<'b>,
    intern: &'a Interner,
    source: Option<&'a str>,
}

impl<'a, 'b> Printer<'a, 'b> {
    fn new(f: &'a mut fmt::Formatter<'b>, intern: &'a Interner, source: Option<&'a str>) -> Self {
        Self { f, intern, source }
    }

    /// Spelling of `name` in this printer's intern; an atom from another
    /// intern renders as a placeholder instead of panicking mid-format.
    fn name(&self, name: &Identifier) -> &'a str {
        self.intern.get(name.name).unwrap_or("<unknown atom>")
    }

    fn write_indent(&mut self, level: usize) -> fmt::Result {
        for _ in 0..level {
            self.f.write_str("    ")?;
        }
        Ok(())
    }

    fn newline_indent(&mut self, level: usize) -> fmt::Result {
        self.f.write_char('\n')?;
        self.write_indent(level)
    }

    fn error_text(&self, span: TextSpan) -> String {
        if let Some(source) = self.source {
            let range = span.to_range();
            if range.end <= source.len() {
                let snippet = source[range].trim();
                if !snippet.is_empty() {
                    return format!("<error: {snippet}>");
                }
            }
        }
        "<error>".to_string()
    }

    fn write_block(&mut self, block: &Block, level: usize) -> fmt::Result {
        for (i, statement) in block.statements.iter().enumerate() {
            if i > 0 {
                self.newline_indent(level)?;
            } else {
                self.write_indent(level)?;
            }
            self.write_statement(statement, level)?;
        }
        Ok(())
    }

    fn write_statement(&mut self, statement: &Statement, level: usize) -> fmt::Result {
        match &statement.kind {
            StatementKind::Expression(expr) => self.write_expr(expr, 0, level),
            StatementKind::Assign(assign) => self.write_assign(assign, level),
            StatementKind::Local(local) => self.write_local(local, level),
            StatementKind::Global(global) => self.write_global(global, level),
            StatementKind::If(if_stmt) => self.write_if(if_stmt, level),
            StatementKind::While(while_stmt) => self.write_while(while_stmt, level),
            StatementKind::Repeat(repeat) => self.write_repeat(repeat, level),
            StatementKind::NumericFor(for_stmt) => self.write_numeric_for(for_stmt, level),
            StatementKind::GenericFor(for_stmt) => self.write_generic_for(for_stmt, level),
            StatementKind::Do(block) => {
                self.f.write_str("do")?;
                self.write_inner_block(block, level)?;
                self.newline_indent(level)?;
                self.f.write_str("end")
            }
            StatementKind::FunctionDecl(decl) => self.write_function_decl(decl, level),
            StatementKind::Return(values) => {
                self.f.write_str("return")?;
                if !values.is_empty() {
                    self.f.write_char(' ')?;
                    self.write_expr_list(values, level)?;
                }
                Ok(())
            }
            StatementKind::Break => self.f.write_str("break"),
            StatementKind::Goto(label) => write!(self.f, "goto {}", self.name(label)),
            StatementKind::Label(label) => write!(self.f, "::{}::", self.name(label)),
            StatementKind::Error => self.f.write_str(&self.error_text(statement.span)),
        }
    }

    fn write_inner_block(&mut self, block: &Block, level: usize) -> fmt::Result {
        if block.statements.is_empty() {
            return Ok(());
        }
        self.newline_indent(level + 1)?;
        self.write_block(block, level + 1)
    }

    fn write_assign(&mut self, assign: &Assign, indent: usize) -> fmt::Result {
        self.write_expr_list(&assign.targets, indent)?;
        self.f.write_str(" = ")?;
        self.write_expr_list(&assign.values, indent)
    }

    fn write_local(&mut self, local: &Local, indent: usize) -> fmt::Result {
        self.f.write_str("local ")?;
        self.write_attnamelist(local.prefix.as_ref(), &local.names)?;
        if !local.values.is_empty() {
            self.f.write_str(" = ")?;
            self.write_expr_list(&local.values, indent)?;
        }
        Ok(())
    }

    fn write_global(&mut self, global: &Global, indent: usize) -> fmt::Result {
        self.f.write_str("global ")?;
        self.write_attnamelist(global.prefix.as_ref(), &global.names)?;
        if global.names.is_empty() {
            self.f.write_str("*")?;
        }
        if !global.values.is_empty() {
            self.f.write_str(" = ")?;
            self.write_expr_list(&global.values, indent)?;
        }
        Ok(())
    }

    fn write_attnamelist(
        &mut self,
        prefix: Option<&Identifier>,
        names: &NodeList<AttributedName>,
    ) -> fmt::Result {
        if let Some(attribute) = prefix {
            write!(self.f, "<{}> ", self.name(attribute))?;
        }
        for (i, name) in names.iter().enumerate() {
            if i > 0 {
                self.f.write_str(", ")?;
            }
            self.write_attributed_name(name)?;
        }
        Ok(())
    }

    fn write_attributed_name(&mut self, name: &AttributedName) -> fmt::Result {
        self.f.write_str(self.name(&name.name))?;
        if let Some(attribute) = &name.attribute {
            write!(self.f, " <{}>", self.name(attribute))?;
        }
        Ok(())
    }

    fn write_if(&mut self, if_stmt: &IfStatement, level: usize) -> fmt::Result {
        for (i, arm) in if_stmt.arms.iter().enumerate() {
            if i == 0 {
                self.f.write_str("if ")?;
            } else {
                self.newline_indent(level)?;
                self.f.write_str("elseif ")?;
            }
            self.write_if_arm(arm, level)?;
        }
        if let Some(else_block) = &if_stmt.else_block {
            self.newline_indent(level)?;
            self.f.write_str("else")?;
            self.write_inner_block(else_block, level)?;
        }
        self.newline_indent(level)?;
        self.f.write_str("end")
    }

    fn write_if_arm(&mut self, arm: &IfArm, level: usize) -> fmt::Result {
        self.write_expr(&arm.condition, 0, level)?;
        self.f.write_str(" then")?;
        self.write_inner_block(&arm.block, level)
    }

    fn write_while(&mut self, while_stmt: &While, level: usize) -> fmt::Result {
        self.f.write_str("while ")?;
        self.write_expr(&while_stmt.condition, 0, level)?;
        self.f.write_str(" do")?;
        self.write_inner_block(&while_stmt.block, level)?;
        self.newline_indent(level)?;
        self.f.write_str("end")
    }

    fn write_repeat(&mut self, repeat: &Repeat, level: usize) -> fmt::Result {
        self.f.write_str("repeat")?;
        self.write_inner_block(&repeat.block, level)?;
        self.newline_indent(level)?;
        self.f.write_str("until ")?;
        self.write_expr(&repeat.condition, 0, level)
    }

    fn write_numeric_for(&mut self, for_stmt: &NumericFor, level: usize) -> fmt::Result {
        write!(self.f, "for {} = ", self.name(&for_stmt.variable))?;
        self.write_expr(&for_stmt.start, 0, level)?;
        self.f.write_str(", ")?;
        self.write_expr(&for_stmt.stop, 0, level)?;
        if let Some(step) = &for_stmt.step {
            self.f.write_str(", ")?;
            self.write_expr(step, 0, level)?;
        }
        self.f.write_str(" do")?;
        self.write_inner_block(&for_stmt.block, level)?;
        self.newline_indent(level)?;
        self.f.write_str("end")
    }

    fn write_generic_for(&mut self, for_stmt: &GenericFor, level: usize) -> fmt::Result {
        self.f.write_str("for ")?;
        self.write_name_list(&for_stmt.names)?;
        self.f.write_str(" in ")?;
        self.write_expr_list(&for_stmt.exprs, level)?;
        self.f.write_str(" do")?;
        self.write_inner_block(&for_stmt.block, level)?;
        self.newline_indent(level)?;
        self.f.write_str("end")
    }

    fn write_function_decl(&mut self, decl: &FunctionDecl, level: usize) -> fmt::Result {
        match &decl.name {
            FunctionScope::Assign { name } => {
                write!(self.f, "function {}", self.name(&name.base))?;
                for segment in &name.path {
                    write!(self.f, ".{}", self.name(segment))?;
                }
                if let Some(method) = &name.method {
                    write!(self.f, ":{}", self.name(method))?;
                }
            }
            FunctionScope::Local { name } => write!(self.f, "local function {}", self.name(name))?,
            FunctionScope::Global { name } => {
                write!(self.f, "global function {}", self.name(name))?;
            }
        };
        self.write_function_body(&decl.body, level, false)
    }

    fn write_function_body(
        &mut self,
        body: &FunctionBody,
        level: usize,
        anonymous: bool,
    ) -> fmt::Result {
        if anonymous {
            self.f.write_str("function")?;
        }
        self.f.write_char('(')?;
        self.write_param_list(&body.params)?;
        self.f.write_char(')')?;
        self.write_inner_block(&body.body, level)?;
        self.newline_indent(level)?;
        self.f.write_str("end")
    }

    fn write_param_list(&mut self, params: &NodeList<Param>) -> fmt::Result {
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                self.f.write_str(", ")?;
            }
            match param {
                Param::Name(name) => self.f.write_str(self.name(name))?,
                Param::Varargs(varargs) => {
                    self.f.write_str("...")?;
                    if let Some(name) = &varargs.name {
                        self.f.write_str(self.name(name))?;
                    }
                }
            }
        }
        Ok(())
    }

    fn write_name_list(&mut self, names: &NodeList<Identifier>) -> fmt::Result {
        for (i, name) in names.iter().enumerate() {
            if i > 0 {
                self.f.write_str(", ")?;
            }
            self.f.write_str(self.name(name))?;
        }
        Ok(())
    }

    fn write_expr_list(&mut self, exprs: &NodeList<Expression>, indent: usize) -> fmt::Result {
        for (i, expr) in exprs.iter().enumerate() {
            if i > 0 {
                self.f.write_str(", ")?;
            }
            self.write_expr(expr, 0, indent)?;
        }
        Ok(())
    }

    fn write_expr(&mut self, expr: &Expression, min_bp: u8, indent: usize) -> fmt::Result {
        match &expr.kind {
            ExpressionKind::Literal(literal) => self.write_literal(literal),
            ExpressionKind::Identifier(name) => self.f.write_str(self.name(name)),
            ExpressionKind::Varargs => self.f.write_str("..."),
            ExpressionKind::Unary { op, operand } => {
                let wrap = UnaryOp::BINDING_POWER < min_bp;
                if wrap {
                    self.f.write_char('(')?;
                }
                self.f.write_str(op.as_str())?;
                if *op == UnaryOp::Not {
                    self.f.write_char(' ')?;
                }
                self.write_expr(operand, UnaryOp::BINDING_POWER, indent)?;
                if wrap {
                    self.f.write_char(')')?;
                }
                Ok(())
            }
            ExpressionKind::Binary { op, lhs, rhs } => {
                let (left_bp, right_bp) = op.binding_power();
                let wrap = left_bp < min_bp;
                if wrap {
                    self.f.write_char('(')?;
                }
                self.write_expr(lhs, left_bp, indent)?;
                write!(self.f, " {} ", op.as_str())?;
                self.write_expr(rhs, right_bp, indent)?;
                if wrap {
                    self.f.write_char(')')?;
                }
                Ok(())
            }
            ExpressionKind::Index { object, index } => {
                self.write_suffix_base(object, indent)?;
                self.f.write_char('[')?;
                self.write_expr(index, 0, indent)?;
                self.f.write_char(']')
            }
            ExpressionKind::Member { object, name } => {
                self.write_suffix_base(object, indent)?;
                write!(self.f, ".{}", self.name(name))
            }
            ExpressionKind::Call { callee, args } => {
                self.write_suffix_base(callee, indent)?;
                self.write_call_args(args, indent)
            }
            ExpressionKind::MethodCall(call) => {
                self.write_suffix_base(&call.receiver, indent)?;
                write!(self.f, ":{}", self.name(&call.name))?;
                self.write_call_args(&call.args, indent)
            }
            ExpressionKind::Function(body) => self.write_function_body(body, indent, true),
            ExpressionKind::Table(fields) => self.write_table(fields, indent),
            ExpressionKind::Grouped(inner) => {
                self.f.write_char('(')?;
                self.write_expr(inner, 0, indent)?;
                self.f.write_char(')')
            }
            ExpressionKind::Error => self.f.write_str(&self.error_text(expr.span)),
        }
    }

    fn write_suffix_base(&mut self, expr: &Expression, indent: usize) -> fmt::Result {
        if needs_suffix_parens(&expr.kind) {
            self.f.write_char('(')?;
            self.write_expr(expr, 0, indent)?;
            self.f.write_char(')')
        } else {
            self.write_expr(expr, 0, indent)
        }
    }

    fn write_call_args(&mut self, args: &NodeList<Expression>, indent: usize) -> fmt::Result {
        self.f.write_char('(')?;
        self.write_expr_list(args, indent)?;
        self.f.write_char(')')
    }

    fn write_table(&mut self, fields: &NodeList<Field>, indent: usize) -> fmt::Result {
        self.f.write_char('{')?;
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                self.f.write_str(", ")?;
            } else if !fields.is_empty() {
                self.f.write_char(' ')?;
            }
            match &field.kind {
                FieldKind::Positional(value) => self.write_expr(value, 0, indent)?,
                FieldKind::Named { name, value } => {
                    write!(self.f, "{} = ", self.name(name))?;
                    self.write_expr(value, 0, indent)?;
                }
                FieldKind::Indexed { key, value } => {
                    self.f.write_char('[')?;
                    self.write_expr(key, 0, indent)?;
                    self.f.write_str("] = ")?;
                    self.write_expr(value, 0, indent)?;
                }
            }
        }
        if !fields.is_empty() {
            self.f.write_char(' ')?;
        }
        self.f.write_char('}')
    }

    fn write_literal(&mut self, literal: &Literal) -> fmt::Result {
        match literal {
            Literal::Nil => self.f.write_str("nil"),
            Literal::Bool(true) => self.f.write_str("true"),
            Literal::Bool(false) => self.f.write_str("false"),
            Literal::Int(value) => write!(self.f, "{value}"),
            Literal::Float(value) if value.is_nan() => self.f.write_str("NaN"),
            Literal::Float(value) => write!(self.f, "{value}"),
            Literal::String(value) => write_lua_string(self.f, value),
        }
    }
}

fn needs_suffix_parens(kind: &ExpressionKind) -> bool {
    !matches!(
        kind,
        ExpressionKind::Identifier(_)
            | ExpressionKind::Member { .. }
            | ExpressionKind::Index { .. }
            | ExpressionKind::Call { .. }
            | ExpressionKind::MethodCall(_)
            | ExpressionKind::Grouped(_)
    )
}

fn write_lua_string(f: &mut fmt::Formatter<'_>, value: &str) -> fmt::Result {
    f.write_char('"')?;
    for c in value.chars() {
        match c {
            '\n' => f.write_str("\\n")?,
            '\r' => f.write_str("\\r")?,
            '\t' => f.write_str("\\t")?,
            '\\' => f.write_str("\\\\")?,
            '"' => f.write_str("\\\"")?,
            '\0' => f.write_str("\\0")?,
            c if c.is_control() => write!(f, "\\{e}", e = c as u32)?,
            c => f.write_char(c)?,
        }
    }
    f.write_char('"')
}

#[cfg(all(test, feature = "parse"))]
mod tests {
    use luoxide_text::Interner;

    use crate::outcome::Outcome;
    use crate::parser::{compile_chunk, compile_expression};

    use super::DisplayLua;

    fn expr(source: &str) -> String {
        let mut intern = Interner::new();
        match compile_expression(&mut intern, source) {
            Outcome::Ok(ast) | Outcome::PartialFailure(ast, _) => {
                DisplayLua::with_source(&ast, &intern, source).to_string()
            }
            Outcome::TotalFailure(errors) => panic!("{errors:?}"),
        }
    }

    fn chunk(source: &str) -> String {
        let mut intern = Interner::new();
        match compile_chunk(&mut intern, source) {
            Outcome::Ok(ast) | Outcome::PartialFailure(ast, _) => {
                DisplayLua::with_source(&ast, &intern, source).to_string()
            }
            Outcome::TotalFailure(errors) => panic!("{errors:?}"),
        }
    }

    #[test]
    fn displays_suffixed_call() {
        assert_eq!(expr("a.b.c()"), "a.b.c()");
    }

    #[test]
    fn displays_precedence() {
        assert_eq!(expr("1 + 2 * 3"), "1 + 2 * 3");
        assert_eq!(expr("(1 + 2) * 3"), "(1 + 2) * 3");
    }

    #[test]
    fn displays_local() {
        assert_eq!(chunk("local x, y = 1, 2"), "local x, y = 1, 2");
    }

    #[test]
    fn displays_prefix_and_postfix_attributes() {
        assert_eq!(
            chunk("local <const> a, b <close> = 1, 2"),
            "local <const> a, b <close> = 1, 2"
        );
        assert_eq!(
            chunk("global <const> PI = 3.14"),
            "global <const> PI = 3.14"
        );
    }

    #[test]
    fn displays_named_varargs() {
        assert_eq!(
            chunk("function f(...args) return args end"),
            "function f(...args)\n        return args\nend"
        );
        assert_eq!(
            chunk("local function g(a, ...rest) end"),
            "local function g(a, ...rest)\nend"
        );
    }
}
