use std::iter::Iterator;
use super::{Value, ProcessInputError, parser};

pub struct ValueIterator<I>
where
    I: Iterator<Item = char>,
{
    iter: std::iter::Peekable<I>,
}

impl<I> ValueIterator<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(iter: I) -> Self {
        let iter = iter.peekable();
        Self { iter }
    }
}

impl<I> Iterator for ValueIterator<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Value, ProcessInputError>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = parser(self.iter.by_ref());
        match val {
            Ok(value) => Some(Ok(value)),
            Err(ProcessInputError::EmptyInput) if self.iter.peek().is_none() => None,
            Err(ProcessInputError::EmptyInput) if self.iter.peek().is_some() => self.next(),
            Err(err) => Some(Err(err))
        }
    }
} 