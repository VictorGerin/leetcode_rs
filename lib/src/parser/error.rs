use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use super::Value;

#[derive(Debug, PartialEq, Eq)]
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
            ProcessInputError::EmptyInput => write!(f, "Empty input"),
            ProcessInputError::UnexpectedError(msg) => write!(f, "Unknow Error: {}", msg),
            ProcessInputError::InvalidEscapeCharacter(msg) => write!(f, "Invalid escape character: {}", msg),
            ProcessInputError::InvalidKeyWord(msg) => write!(f, "Invalid key word: {}", msg),
        }
    }
}

impl Error for ProcessInputError {} 

#[derive(PartialEq)]
pub enum ValueError {
    IsNotVec(Value),
    NotAllElementsIsIntOnVec(Value),
}

impl Display for ValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueError::IsNotVec(v) => write!(f, "Expected Vec, got {:?}", v),
            ValueError::NotAllElementsIsIntOnVec(v) => write!(f, "Expected all elements to be Int, got {:?}", v),
        }
    }
}

impl Debug for ValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueError::IsNotVec(v) => write!(f, "Expected Vec, got {:?}", v),
            ValueError::NotAllElementsIsIntOnVec(v) => write!(f, "Expected all elements to be Int, got {:?}", v),
        }
    }
}