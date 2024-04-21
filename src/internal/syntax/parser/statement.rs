use super::*;

impl<'source> Parser<'source> {
    pub fn statement(&mut self) -> Result<ast::Statement, SpannedError> {
        //self.simple_stmt()
        todo!("statement");
    }

    /// Check if the current token is a statement anchor
    ///
    /// This function returns true if the token indicates a possible statement start
    /// This function is used to synchronize the parser after a ParserError
    pub fn statement_anchor_token(&mut self, token: &TokenKind) -> bool {
        // First flow control keywords and keywords are a good start
        match token {
            TokenKind::Kw_Break
            | TokenKind::Kw_Do
            | TokenKind::Kw_Goto
            | TokenKind::Kw_If
            | TokenKind::Kw_ElseIf
            | TokenKind::Kw_Else => return true,
            TokenKind::Kw_End
            | TokenKind::Kw_While
            | TokenKind::Kw_For
            | TokenKind::Kw_Function
            | TokenKind::Kw_Local
            | TokenKind::Kw_Return => return true,
            TokenKind::Tok_SemiColon => return true,
            _ => (),
        };
        false
        // TODO: add assignment operators and labels
    }
}
