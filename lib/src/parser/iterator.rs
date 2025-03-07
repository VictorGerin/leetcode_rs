use std::iter::Iterator;
use super::{Value, ProcessInputError, process_input};

pub struct ValueIterator<I>
where
    I: Iterator<Item = char>,
{
    iter: I,
}

impl<I> ValueIterator<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Iterator for ValueIterator<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Value, ProcessInputError>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = process_input(self.iter.by_ref());
        match val {
            Ok(value) => Some(Ok(value)),
            Err(ProcessInputError::EmptyInput) => None,
            Err(err) => Some(Err(err))
        }
    }
} 