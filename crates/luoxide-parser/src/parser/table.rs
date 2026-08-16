//! Table constructor parsing.

use crate::ast::{Expression, Field, FieldKind, NodeList};
use crate::error::Result;

use super::Parser;

impl Parser<'_> {
    /// ```BNF
    /// table_constructor ::= '{' [field {field_separator field} [field_separator]] '}'
    /// field ::= '[' expression ']' '=' expression | Name '=' expression | expression
    /// field_separator ::= ',' | ';'
    /// ```
    pub fn parse_table_constructor(&mut self) -> Result<Expression> {
        let open = *self.current_token();
        debug_assert!(open.is(token!("{")));
        self.bump();

        let mut fields: NodeList<Field> = NodeList::new();

        while self.current_is_not(token!("}")) && !self.is_at_end() {
            // One bad field doesn't discard the whole table: record the
            // error, skip to the next anchor and continue with the next field.
            match self.parse_field() {
                Ok(field) => fields.push(field),
                Err(error) => {
                    let start = self.current_token().span;
                    self.recover_expression(error);
                    let span = start.merge(self.current_token().span);
                    fields.push(Field {
                        kind: FieldKind::Positional(Expression::error(span)),
                        span,
                    });
                }
            }

            // Field separator: mandatory between fields, optional before `}`.
            if self.expect(token!(",")).is_none() && self.expect(token!(";")).is_none() {
                break;
            }
        }

        self.expect_or_error(token!("}"));
        let span = open.span.merge(self.previous_span());
        Ok(Expression::table(fields, span))
    }

    fn parse_field(&mut self) -> Result<Field> {
        let start = self.current_token().span;

        let kind = match self.current_token().kind {
            // `[key] = value`
            token!("[") => {
                self.bump();
                let key = self.parse_expression()?;
                self.expect_or_error(token!("]"));
                self.expect_or_error(token!("="));
                let value = self.parse_expression()?;
                FieldKind::Indexed { key, value }
            }
            // `name = value` — needs lookahead: a lone identifier can also
            // start a positional expression like `x + 1`.
            token!(identifier) => {
                let name = self
                    .maybe_identifier()
                    .expect("current token is an identifier");
                if self.expect(token!("=")).is_some() {
                    let value = self.parse_expression()?;
                    FieldKind::Named { name, value }
                } else {
                    // Not an assignment: the identifier is the start of an
                    // expression; continue suffix and binary parsing from it.
                    let primary = Expression::identifier(name);
                    let suffixed = self.parse_suffixed_rest(primary)?;
                    let value = self.parse_binary_rest(suffixed, 0)?;
                    FieldKind::Positional(value)
                }
            }
            // `"key" = value` — needs lookahead: a lone string literal can also
            // start a positional expression like `"a" + "b"`.
            token!(string) => {
                let string = self.parse_string_literal();
                FieldKind::Positional(string)
            }
            // `value`
            _ => FieldKind::Positional(self.parse_expression()?),
        };

        let span = start.merge(self.previous_span());
        Ok(Field { kind, span })
    }
}
