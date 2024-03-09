pub mod lexer;
pub mod parser;

pub struct SyntaxError {
    errors: Vec<String>,
}
