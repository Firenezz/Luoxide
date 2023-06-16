use std::io::Cursor;

use Luoxidant::lexer::Lexer;



fn main () {
    let source = "print('Hello World')";
    let mut lexer = Lexer::new(Cursor::new(source), Luoxidant::intern::DefaultInterner::default());

    println!("{:?}", lexer.line());
}