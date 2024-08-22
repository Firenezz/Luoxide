use luoxide_parser::lexer::Lexer;

fn main() {
    temp()
}

fn temp() {
    let lexer = Lexer::new("print('Hello World')");

    lexer.into_iter().for_each(|token| println!("{:?}", token));
}
