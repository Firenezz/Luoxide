pub mod lexer;
pub mod parser;

#[allow(dead_code)] // TODO: allow until parser is done
pub struct SyntaxError {
    errors: Vec<String>,
}
