use core::fmt;
use std::ops::Range;

use logos::{Lexer as LogosLexer, Logos};
use luoxide_text::traits::TextLen;

use crate::token::{Token, TokenKind};

pub struct Lexer<'source> {
    inner: LogosLexer<'source, TokenKind>,

    /// Current token. It is not yet consumed by the parser
    current: Token,
    /// Previous token. It is consumed by the parser and it should be part of the current parsing operation
    previous: Token,
    /// Token that marks the end of the file
    ///
    /// It is returned by the lexer when it hits the end of the file. It also has the size of the source code.
    end_of_file: Token,

    leading_trivias: Vec<Trivia>,
}

pub struct Trivia {
    pub token: Token,
}

impl<'source> Lexer<'source> {
    fn advance_token(&mut self, ignore_trivia: bool) -> Option<Token> {
        let inner_lexer = &mut self.inner;

        self.leading_trivias.clear();

        while let Some(kind) = inner_lexer.next() {
            let _lexeme = inner_lexer.slice();
            let span = inner_lexer.span();
            let span = ((span.start as u32)..(span.end as u32)).into();

            match kind {
                Ok(kind) => {
                    let token = Token { kind, span };
                    match kind {
                        TokenKind::_Tok_Comment
                        | TokenKind::_Tok_MultilineComment
                        | TokenKind::_Newline => {
                            if !ignore_trivia {
                                self.leading_trivias.push(Trivia { token });
                            }
                        }
                        _ => return Some(token),
                    }
                }
                // TODO: Make it pass the error
                Err(_) => {
                    let token = Token {
                        kind: TokenKind::Tok_Error,
                        span,
                    };
                    return Some(token);
                }
            }
        }

        None
    }

    pub fn new(source: &'source str) -> Self {
        let end = if let Ok(end) = source.try_text_len() {
            end
        } else {
            panic!("source is too large")
        };
        let eof = Token {
            kind: TokenKind::Tok_Eof,
            span: (end..end).into(),
        };
        let mut lex = Self {
            inner: TokenKind::lexer(source),
            current: eof,
            previous: eof,
            end_of_file: eof,
            leading_trivias: vec![],
        };

        lex.bump();
        lex.bump();

        lex
    }

    #[inline]
    #[allow(dead_code)]
    pub fn previous(&self) -> &Token {
        &self.previous
    }

    #[inline]
    pub fn current(&self) -> &Token {
        &self.current
    }

    #[inline]
    pub fn kind(&self) -> TokenKind {
        self.current.kind
    }

    #[inline]
    pub fn lexeme(&self, token: &Token) -> &'source str {
        &self.inner.source()[Range::from(token.span)]
    }

    pub fn bump(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);

        self.current = self.advance_token(true).unwrap_or(self.end_of_file);
    }

    /// Bumps the lexer and dont skip trivia
    pub fn bump_with_trivia(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);

        self.current = self.advance_token(false).unwrap_or(self.end_of_file);
    }
}

pub struct Tokens<'source>(pub Lexer<'source>);

impl<'source> Iterator for Tokens<'source> {
    type Item = (&'source str, Token);

    fn next(&mut self) -> Option<Self::Item> {
        let token = *self.0.current();
        self.0.bump();
        if !token.is(TokenKind::Tok_Eof) {
            Some((self.0.lexeme(&token), token))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub struct DisplayToken<'source>(pub Token, pub &'source str);

impl<'source> fmt::Debug for DisplayToken<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self.0.kind;
        let span = self.0.span;
        write!(f, "(>{kind:?} @{span:?})")
    }
}

impl<'source> fmt::Display for DisplayToken<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self.0.kind;
        let span = self.0.span;
        write!(f, "(>{kind} @{span})")
    }
}

pub struct TokenVec<'a>(pub Vec<DisplayToken<'a>>);

impl<'a> fmt::Debug for TokenVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl<'a> fmt::Display for TokenVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut comma_separated = String::new();

        for token in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(format!("{}", token).as_str());
            comma_separated.push_str(",\n");
        }

        comma_separated.push_str(format!("{}", &self.0[self.0.len() - 1]).as_str());
        write!(f, "{}", comma_separated)
    }
}

pub mod peekable;
#[cfg(test)]
mod tests;
