
use std::io::Cursor;

use luoxidant::intern;

fn main() {
    env_logger::init();

    let _interner = intern::DefaultInterner::default();

    let _source = Cursor::new("local a = 1");



    //let mut lexer = LexerState::new();
}

