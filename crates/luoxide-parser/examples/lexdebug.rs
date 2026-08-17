use luoxide_parser::lexer::{Lexer, Tokens};

fn main() {
    let case = "[[alo\n123\"]]";
    let lexer = Lexer::new(case);
    for (lexeme, token) in Tokens(lexer) {
        println!("  {:?} {:?}", token.kind, lexeme);
    }
}
