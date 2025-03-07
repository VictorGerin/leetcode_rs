use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProcessInputError {
    InvalidInput,
    EmptyInput,
    UnexpectedError(String),
    InvalidEscapeCharacter(String),
    InvalidKeyWord(String)
}

impl Display for ProcessInputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessInputError::InvalidInput => write!(f, "Invalid input"),
            ProcessInputError::EmptyInput => write!(f, "empty input"),
            ProcessInputError::UnexpectedError(msg) => write!(f, "Unknow Error: {}", msg),
            ProcessInputError::InvalidEscapeCharacter(msg) => write!(f, "Invalid escape character: {}", msg),
            ProcessInputError::InvalidKeyWord(msg) => write!(f, "Invalid key word: {}", msg),
        }
    }
}

impl Error for ProcessInputError {} 