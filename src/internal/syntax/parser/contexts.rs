use std::{ops::Range, rc::Rc};

use crate::internal::{
    source::{range::TextRange, text::TextSize},
    syntax::{
        lexer::{LineInfo, Token},
        SyntaxError,
    },
};

#[derive(Default)]
pub struct ErrorContext {
    defered_syntax_errors: Vec<SyntaxError>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn defer_error(&mut self, error: SyntaxError) {
        self.defered_syntax_errors.push(error);
    }

    pub fn errors() -> Rc<SyntaxError> {
        todo!("");
    }

    pub fn has_errors() -> bool {
        todo!("");
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Marker {
    pub range: TextRange,
    pub locations: Range<Location>,
}

impl Marker {
    pub fn from_token(token: &Token) -> Self {
        let line_info = token
            .last_line_info
            .or(Some(token.begin_line_info))
            .expect("Line must be set in the lexer, if this happens it's a bug");
        Self {
            range: token.span.into(),
            locations: Location {
                line: line_info.line.0.into(),
                column: (token.span.start - line_info.start_of_line).into(),
            }..Location {
                line: 0.into(),
                column: 0.into(),
            },
        }
    }

    pub fn create_from_current(token: &Token) -> Self {
        Self::from_token(token)
    }

    pub fn complete(&mut self, previous: &Token) {
        // TODO: add location and range options
        self.range.end = previous.span.end.into();

        let mut end_location = self.locations.end;

        let line_info = previous
            .last_line_info
            .or(Some(previous.begin_line_info))
            .expect("Line must be set in the lexer, if this happens it's a bug");

        end_location.line = line_info.line.0.into();
        end_location.column =
            (self.range.end.to_usize() - (previous.span.end - line_info.start_of_line)).into();
    }

    pub fn bless(self, node: &mut impl Marked) {
        node.mark(self);
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub line: TextSize,
    pub column: TextSize,
}

pub trait Marked {
    fn mark(&mut self, marker: Marker);
}
