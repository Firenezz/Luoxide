//! Statement, block and chunk parsing.
//!
//! Blocks are the error-recovery boundary: when a statement fails to parse,
//! the error is recorded, the parser synchronizes to the next plausible
//! statement start, and an `Error` statement takes the failed statement's
//! place. Callers therefore always receive a complete tree.

use tracing::{event, Level};

use crate::ast::statements::Global;
use crate::ast::{
    self, Assign, AttributedName, Block, Chunk, Expression, ExpressionKind, FunctionDecl,
    FunctionName, GenericFor, IfArm, IfStatement, Local, LocalFunction, NodeList, NumericFor,
    Repeat, Statement, StatementKind, While,
};
use crate::error::Result;

use super::Parser;

impl Parser<'_> {
    /// Parses a whole chunk (source file). Never fails; all errors are
    /// recorded in the [`ErrorContext`](super::error::ErrorContext).
    pub fn parse_chunk(&mut self) -> Chunk {
        let block = self.parse_block();

        if !self.is_at_end() {
            // parse_block only stops early on a block terminator (`end`,
            // `else`, ...) which has no opener here.
            let current = *self.current_token();
            let error = self.unexpected_token([token!(EOF)], &current.kind, Some(current.span));
            self.error_context.add_error(error);
        }

        Chunk { block }
    }

    /// ```BNF
    /// block ::= {statement} [return_statement]
    /// ```
    ///
    /// Stops at (without consuming) a block terminator: `end`, `else`,
    /// `elseif`, `until` or end of file.
    pub fn parse_block(&mut self) -> Block {
        let start = self.current_token().span;
        let mut statements: NodeList<Statement> = NodeList::new();

        loop {
            // Skip empty statements (`;`).
            while self.maybe(token!(";")).is_some() {}

            if Self::is_block_terminator(self.current_kind()) {
                break;
            }

            let is_return = self.current_is(token!(return));
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    let recovery_start = self.current_token().span;
                    self.recover_statement(error);
                    let recovery_span = recovery_start.merge(self.previous_span());
                    statements.push(Statement::error(recovery_span));
                    continue;
                }
            }

            // `return` must be the last statement of a block.
            if is_return {
                self.maybe(token!(";"));
                break;
            }
        }

        let span = if let (Some(first), Some(last)) = (statements.first(), statements.last()) {
            first.span.merge(last.span)
        } else {
            luoxide_text::range::TextSpan::empty(start.start)
        };

        Block::new(statements, span)
    }

    /// ```BNF
    /// statement ::= ';' | if | while | do | for | repeat | function
    ///     | local | global | return | break | goto | label | assignment | call
    /// ```
    pub fn parse_statement(&mut self) -> Result<Statement> {
        event!(Level::TRACE, "parsing statement");
        let at = self.current_token().span;
        self.with_depth(at, |parser| parser.parse_statement_inner())
    }

    fn parse_statement_inner(&mut self) -> Result<Statement> {
        let start = *self.current_token();

        let kind = match start.kind {
            token!(global) => self.parse_global()?,
            token!(if) => self.parse_if()?,
            token!(while) => self.parse_while()?,
            token!(do) => {
                self.bump();
                let block = self.parse_block();
                self.expect(token!(end));
                StatementKind::Do(ast::P(block))
            }
            token!(for) => self.parse_for()?,
            token!(repeat) => self.parse_repeat()?,
            token!(function) => self.parse_function_declaration()?,
            token!(local) => self.parse_local()?,
            token!(return) => {
                self.bump();
                let mut values: NodeList<Expression> = NodeList::new();
                if !Self::is_block_terminator(self.current_kind())
                    && self.current_is_not(token!(";"))
                {
                    loop {
                        values.push(self.parse_expression()?);
                        if self.maybe(token!(",")).is_none() {
                            break;
                        }
                    }
                }
                StatementKind::Return(values)
            }
            token!(break) => {
                self.bump();
                StatementKind::Break
            }
            token!(goto) => {
                self.bump();
                let label = self.require_identifier()?;
                StatementKind::Goto(label)
            }
            token!("::") => {
                self.bump();
                let label = self.require_identifier()?;
                self.expect(token!("::"));
                StatementKind::Label(label)
            }
            token!(EOF) => return Err(self.unexpected_eof(Some(start.span))),
            kind if kind.is_reserved() => {
                self.bump();
                return Err(self.reserved_keyword(Some(start.span)));
            }
            // Anything else must be an assignment or a call statement.
            _ => self.parse_expression_statement()?,
        };

        let span = start.span.merge(self.previous_span());
        Ok(Statement::new(kind, span))
    }

    /// ```BNF
    /// if_statement ::= if expression then block
    ///     {elseif expression then block} [else block] end
    /// ```
    fn parse_if(&mut self) -> Result<StatementKind> {
        debug_assert!(self.current_is(token!(if)));

        let mut arms: NodeList<IfArm> = NodeList::new();
        // First `if` arm, then any number of `elseif` arms.
        loop {
            self.bump(); // `if` / `elseif`
            let condition = self.parse_expression()?;
            self.expect(token!(then));
            let block = self.parse_block();
            arms.push(IfArm { condition, block });

            if !self.current_is(token!(else if)) {
                break;
            }
        }

        let else_block = if self.maybe(token!(else)).is_some() {
            Some(self.parse_block())
        } else {
            None
        };

        self.expect(token!(end));
        Ok(StatementKind::If(ast::P(IfStatement { arms, else_block })))
    }

    /// ```BNF
    /// while_statement ::= while expression do block end
    /// ```
    fn parse_while(&mut self) -> Result<StatementKind> {
        debug_assert!(self.current_is(token!(while)));
        self.bump();

        let condition = self.parse_expression()?;
        self.expect(token!(do));
        let block = self.parse_block();
        self.expect(token!(end));

        Ok(StatementKind::While(ast::P(While { condition, block })))
    }

    /// ```BNF
    /// repeat_statement ::= repeat block until expression
    /// ```
    fn parse_repeat(&mut self) -> Result<StatementKind> {
        debug_assert!(self.current_is(token!(repeat)));
        self.bump();

        let block = self.parse_block();
        self.expect(token!(until));
        let condition = self.parse_expression()?;

        Ok(StatementKind::Repeat(ast::P(Repeat { block, condition })))
    }

    /// ```BNF
    /// for_statement ::= for Name '=' expression ',' expression [',' expression] do block end
    ///     | for name_list in expression_list do block end
    /// ```
    fn parse_for(&mut self) -> Result<StatementKind> {
        debug_assert!(self.current_is(token!(for)));
        self.bump();

        let first_name = self.require_identifier()?;

        // Numeric form: `for i = start, stop [, step]`
        if self.maybe(token!("=")).is_some() {
            let start = self.parse_expression()?;
            self.expect(token!(","));
            let stop = self.parse_expression()?;
            let step = if self.maybe(token!(",")).is_some() {
                Some(self.parse_expression()?)
            } else {
                None
            };

            self.expect(token!(do));
            let block = self.parse_block();
            self.expect(token!(end));

            return Ok(StatementKind::NumericFor(ast::P(NumericFor {
                variable: first_name,
                start,
                stop,
                step,
                block,
            })));
        }

        // Generic form: `for a, b in exprs`
        let mut names: NodeList<ast::Identifier> = NodeList::new();
        names.push(first_name);
        while self.maybe(token!(",")).is_some() {
            names.push(self.require_identifier()?);
        }

        self.expect(token!(in));

        let mut exprs: NodeList<Expression> = NodeList::new();
        loop {
            exprs.push(self.parse_expression()?);
            if self.maybe(token!(",")).is_none() {
                break;
            }
        }

        self.expect(token!(do));
        let block = self.parse_block();
        self.expect(token!(end));

        Ok(StatementKind::GenericFor(ast::P(GenericFor {
            names,
            exprs,
            block,
        })))
    }

    /// ```BNF
    /// function_declaration ::= function funcname function_body
    /// funcname ::= Name {'.' Name} [':' Name]
    /// ```
    fn parse_function_declaration(&mut self) -> Result<StatementKind> {
        debug_assert!(self.current_is(token!(function)));
        let start = self.current_token().span;
        self.bump();

        let base = self.require_identifier()?;
        let mut path: NodeList<ast::Identifier> = NodeList::new();
        while self.maybe(token!(".")).is_some() {
            path.push(self.require_identifier()?);
        }
        let method = if self.maybe(token!(":")).is_some() {
            Some(self.require_identifier()?)
        } else {
            None
        };

        let (body, _span) = self.parse_function_body(start)?;

        Ok(StatementKind::FunctionDecl(ast::P(FunctionDecl {
            name: FunctionName { base, path, method },
            body,
        })))
    }

    /// ```BNF
    /// local_statement ::= local function Name function_body
    ///     | local attnamelist ['=' explist]
    /// global_statement ::= global attnamelist ['=' explist]
    /// attnamelist ::= [attrib] Name [attrib] {',' Name [attrib]}
    /// attrib ::= '<' Name '>'
    /// ```
    fn parse_local(&mut self) -> Result<StatementKind> {
        debug_assert!(self.current_is(token!(local)));
        let start = self.current_token().span;
        self.bump();

        // `local function f() ... end`
        if self.maybe(token!(function)).is_some() {
            let name = self.require_identifier()?;
            let (body, _span) = self.parse_function_body(start)?;
            return Ok(StatementKind::LocalFunction(ast::P(LocalFunction {
                name,
                body,
            })));
        }

        let (prefix, names) = self.parse_attnamelist()?;
        let values = self.parse_optional_explist()?;
        Ok(StatementKind::Local(ast::P(Local {
            prefix,
            names,
            values,
        })))
    }

    /// ```BNF
    /// global_statement ::=
    ///     | global attnamelist ['=' explist]
    ///     | global [attrib] '*'
    /// ```
    fn parse_global(&mut self) -> Result<StatementKind> {
        debug_assert!(self.current_is(token!(global)));
        self.bump();

        if self.maybe(token!("*")).is_some() {
            return Ok(StatementKind::Global(ast::P(Global {
                prefix: None,
                names: NodeList::new(),
                values: NodeList::new(),
            })));
        }

        let (prefix, names) = self.parse_attnamelist()?;
        let values = self.parse_optional_explist()?;
        Ok(StatementKind::Global(ast::P(Global {
            prefix,
            names,
            values,
        })))
    }

    /// ```BNF
    /// attrib ::= '<' Name '>'
    /// ```
    fn parse_attrib(&mut self) -> Result<Option<ast::Identifier>> {
        if self.maybe(token!("<")).is_none() {
            return Ok(None);
        }
        let attribute = self.require_identifier()?;
        self.expect(token!(">"));
        Ok(Some(attribute))
    }

    /// ```BNF
    /// attnamelist ::= [attrib] Name [attrib] {',' Name [attrib]}
    /// ```
    fn parse_attnamelist(&mut self) -> Result<(Option<ast::Identifier>, NodeList<AttributedName>)> {
        let prefix = self.parse_attrib()?;
        let mut names: NodeList<AttributedName> = NodeList::new();
        loop {
            let name = self.require_identifier()?;
            let attribute = self.parse_attrib()?;
            names.push(AttributedName { name, attribute });
            if self.maybe(token!(",")).is_none() {
                break;
            }
        }
        Ok((prefix, names))
    }

    /// ```BNF
    /// explist ::= exp {',' exp}
    /// ```
    ///
    /// Optional: `['=' explist]`
    fn parse_optional_explist(&mut self) -> Result<NodeList<Expression>> {
        if self.maybe(token!("=")).is_none() {
            return Ok(NodeList::new());
        }
        self.parse_list(token!(","), Self::parse_expression)
    }

    /// Disambiguates assignments from call statements: parse a suffixed
    /// expression first, then decide based on the next token (`=` or `,`
    /// means assignment). No backtracking needed.
    ///
    /// ```BNF
    /// expression_statement ::= call | var_list '=' expression_list
    /// ```
    fn parse_expression_statement(&mut self) -> Result<StatementKind> {
        let first = self.parse_suffixed_expression()?;

        // Plain call statement.
        if !self.current_is(token!(",")) && !self.current_is(token!("=")) {
            if !first.is_call() {
                let error = self.non_call_expression_statement(Some(first.span));
                self.error_context.add_error(error);
            }
            return Ok(StatementKind::Expression(first));
        }

        // Assignment: collect remaining targets, then values.
        let mut targets: NodeList<Expression> = NodeList::new();
        self.check_assignment_target(&first);
        targets.push(first);
        while self.maybe(token!(",")).is_some() {
            let target = self.parse_suffixed_expression()?;
            self.check_assignment_target(&target);
            targets.push(target);
        }

        self.expect(token!("="));

        let mut values: NodeList<Expression> = NodeList::new();
        loop {
            values.push(self.parse_expression()?);
            if self.maybe(token!(",")).is_none() {
                break;
            }
        }

        Ok(StatementKind::Assign(ast::P(Assign { targets, values })))
    }

    /// Records an error when `target` cannot be assigned to. Recoverable: the
    /// tree keeps the invalid target so tooling still sees it.
    fn check_assignment_target(&mut self, target: &Expression) {
        let is_assignable = matches!(
            target.kind,
            ExpressionKind::Identifier(..)
                | ExpressionKind::Member { .. }
                | ExpressionKind::Index { .. }
        );
        if !is_assignable {
            let error = self.invalid_assignment_target(Some(target.span));
            self.error_context.add_error(error);
        }
    }
}
