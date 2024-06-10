

impl SyntaxErrorKind::UnexpectedToken {
    pub fn new(token: &Token) -> Self {
        Self {
            token: token.clone(),
        }
    }
}