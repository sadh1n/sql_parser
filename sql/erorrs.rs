use std::fmt;

#[derive(Debug)]
pub enum SQLError {
    TokenizerError(String),
    ParserError(String),
    ExpressionError(String),
}

impl fmt::Display for SQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SQLError::TokenizerError(msg) => write!(f, "Tokenizer error: {}", msg),
            SQLError::ParserError(msg) => write!(f, "Parser error: {}", msg),
            SQLError::ExpressionError(msg) => write!(f, "Expression error: {}", msg),
        }
    }
}

impl std::error::Error for SQLError {}

pub type Result<T> = std::result::Result<T, SQLError>;
