
use std::{fmt, string, io::Read};

use crate::intern::StringInterner;

pub struct Lexer<R, S> {
    source: Option<R>,
    interner: S,
    line: usize,
    string_buffer: Vec<u8>,
    peekBuffer: Vec<u8>
}

impl<R, S> Lexer<R, S>
    where
    R: Read + 'static,
    S: StringInterner
{
    pub fn new(source: R, string_interner: S) -> Self {
        Lexer {
            source: Some(source),
            interner: string_interner,
            line: 0,
            string_buffer: Vec::new(),
            peekBuffer: Vec::new()
        }
    }

    /// Current line number of the source file, 0-indexed
    pub fn line(&self) -> usize {
        self.line
    }
}