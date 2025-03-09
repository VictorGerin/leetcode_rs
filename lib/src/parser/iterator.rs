use std::iter::Iterator;
use super::{Val, ProcessInputError, parser};

pub struct ValIter<I>
where
    I: Iterator<Item = char>,
{
    iter: std::iter::Peekable<I>,
}

impl<I> ValIter<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(iter: I) -> Self {
        let iter = iter.peekable();
        Self { iter }
    }
}

impl<I> Iterator for ValIter<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Val, ProcessInputError>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = parser(self.iter.by_ref());
        match val {
            // if has value return it
            Ok(value) => Some(Ok(value)),
            Err(ProcessInputError::EmptyInput) if self.iter.peek().is_some() => self.next(),
            Err(ProcessInputError::EmptyInput) if self.iter.peek().is_none() => None,
            Err(err) => Some(Err(err))
        }
    }
}

impl<'a> From<&'a str> for ValIter<std::str::Chars<'a>> 
{
    fn from(input: &'a str) -> Self {
        let iter = input.chars();
        Self::new(iter)
    }
}

impl std::str::FromStr for ValIter<std::vec::IntoIter<char>>
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.chars().collect::<Vec<char>>().into_iter()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_value_return() {

        let mut iter = Into::<ValIter<_>>::into(r#"/*
        Test a input that only contains comment until the end of the input
        */"#).into_iter();
        
        assert_eq!(iter.next(), None);
    }


    #[test]
    fn value_iterator() -> Result<(), String> {

        let iter = Into::<ValIter<_>>::into("1 2 3 4 5")
            .collect::<Result<Vec<Val>, _>>()
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|x| x.as_int())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|x| format!("\"{:?}\" is not a int", x))?;
    
        assert_eq!(iter, vec![1, 2, 3, 4, 5]);
    
        Ok(())
    }

    #[test]
    fn value_iterator_with_comments() -> Result<(), String> {

        let input: ValIter<_> = r#"
        // first obj is a comment
        1
        // this is a comment
        2
        3
        /*
                6
                7
                8
                9
        */
        4 // this is a inline comment
        5
        "#.parse().unwrap();

        let iter = input
            .collect::<Result<Vec<Val>, _>>()
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|x| x.as_int())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|x| format!("\"{:?}\" is not a int", x))?;
    
        assert_eq!(iter, vec![1, 2, 3, 4, 5]);
    
        Ok(())
    }

    #[test]
    fn value_iterator_multple_types() -> Result<(), String> {

        let input = r#"
        "Ola" 123
        //comment
        3.14 false
        [10] null
        [[[]]] //just for fun
        "#.chars();

        let mut iter = ValIter::new(input)
            .collect::<Result<Vec<Val>, _>>()
            .map_err(|e| e.to_string())?
            .into_iter();

        assert_eq!(iter.len(), 7);

        assert_eq!(iter.next(), Some(Val::Str("Ola".to_string())));
        assert_eq!(iter.next(), Some(Val::Int(123)));
        assert_eq!(iter.next(), Some(Val::Double(3.14)));
        assert_eq!(iter.next(), Some(Val::Bool(false)));
        assert_eq!(iter.next(), Some(Val::Vec(vec![Val::Int(10)])));
        assert_eq!(iter.next(), Some(Val::None));
        assert_eq!(iter.next(), Some(Val::Vec(vec![Val::Vec(vec![Val::Vec(vec![])])])));

        assert_eq!(iter.next(), None);
    
        Ok(())
    }

    #[test]
    fn value_iterator_empty_input() -> Result<(), String> {

        let input = "".chars();

        let iter = ValIter::new(input)
            .collect::<Result<Vec<Val>, _>>()
            .map_err(|e| e.to_string())?;
    
        assert_eq!(iter.len(), 0);
    
        Ok(())
    }

    #[test]
    fn value_iterator_empty_array() -> Result<(), String> {

        let input = "[]".chars();
        let vec = ValIter::new(input)
            .collect::<Result<Vec<Val>, _>>()
            .map_err(|e| e.to_string());

        assert_eq!(vec, Ok(vec![Val::Vec(vec![])]));

        Ok(())
    }
}