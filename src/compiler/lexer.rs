
use std::{fmt, string, io::{Read, Cursor}, iter::Peekable, rc::Rc};

use crate::intern::StringInterner;

pub struct LexerState {
    pub line: usize,
    pub column: usize,
    pub source: Rc<[u8]>,
    pub peek_buffer: Rc<[u8]>,
    pub string_buffer: Rc<[u8]>
}

pub struct FiniteStateMachine {
    pub state: LexerState,
    //pub peekable: Peekable<Vec<u8>>
}

pub struct Lexer<R, S> {
    source: R,
    interner: S,
    line: usize,
    string_buffer: Rc<[u8]>,
    peekBuffer: Rc<[u8]>
}

impl<R, S> Lexer<R, S>
    where
    R: Read,
    S: StringInterner
{
    pub fn new(source: R, string_interner: S) -> Self {
        Lexer {
            source: source,
            interner: string_interner,
            line: 0,
            string_buffer: Vec::new().into(),
            peekBuffer: Vec::new().into(),
        }
    }

    /// Current line number of the source file, 0-indexed
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn peek(&self, n: usize) -> Option<u8> {
        self.peekBuffer.get(n).copied()
    }

    pub fn peek_next(&self) -> Option<u8> {
        self.peekBuffer.get(1).copied()
    }
}