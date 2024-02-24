
use std::io::Cursor;

use log::trace;
use luoxidant::compiler::lexer::LexerState;

use log::debug;
use log::error;
use log::info;
use log::warn;
use luoxidant::intern;

fn main() {
    env_logger::init();

    let interner = intern::DefaultInterner::default();

    let source = Cursor::new("local a = 1");

    let mut lexer = LexerState::new(&source, &interner);

    while let Some(token) = lexer. {
        debug!("{:?}", token);
    }

    //let mut lexer = LexerState::new();
}