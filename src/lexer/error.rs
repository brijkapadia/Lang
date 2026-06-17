#[derive(Debug)]
pub enum LexerError {
    UnexpectedEOF,
    TokenExpectedBeforEOF(String),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_string = match self {
            Self::UnexpectedEOF => "unexpected EOF".to_string(),
            Self::TokenExpectedBeforEOF(token) => format!("{token} expected before EOF"),
        };
        write!(f, "{}", display_string)
    }
}

impl std::error::Error for LexerError {}
