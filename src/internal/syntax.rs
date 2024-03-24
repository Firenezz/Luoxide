use core::fmt::Display;

use crate::error::SpannedError;

pub mod ast;
pub mod lexer;
pub mod parser;

#[derive(Debug)]
#[allow(dead_code)] // TODO: allow until parser is done
pub struct SyntaxError {
    errors: Vec<SpannedError>,
}

impl SyntaxError {
    fn new(errors: Vec<SpannedError>) -> Self {
        Self { errors }
    }

    pub fn errors(&self) -> &[SpannedError] {
        &self.errors
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("SyntaxError::fmt");
        //write!(f, "{}", self.errors.iter().join("\n"))
    }
}
