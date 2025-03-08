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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_value_iterator() -> Result<(), String> {

        let input = "1 2 3 4 5".chars();

        let iter = ValueIterator::new(input)
            .collect::<Result<Vec<Value>, _>>()
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|x| x.as_int())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|x| format!("\"{:?}\" is not a int", x))?;
    
        assert_eq!(iter, vec![1, 2, 3, 4, 5]);
    
        Ok(())
    }

    #[test]
    fn test_value_iterator_with_comments() -> Result<(), String> {

        let input = r#"
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
        "#.chars();

        let iter = ValueIterator::new(input)
            .collect::<Result<Vec<Value>, _>>()
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|x| x.as_int())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|x| format!("\"{:?}\" is not a int", x))?;
    
        assert_eq!(iter, vec![1, 2, 3, 4, 5]);
    
        Ok(())
    }

    #[test]
    fn test_value_iterator_multple_types() -> Result<(), String> {

        let input = r#"
        "Ola" 123
        //comment
        3.14 false
        [10] null
        [[[1]]] //just for fun
        "#.chars();

        let mut iter = ValueIterator::new(input)
            .collect::<Result<Vec<Value>, _>>()
            .map_err(|e| e.to_string())?
            .into_iter();

        assert_eq!(iter.len(), 7);

        assert_eq!(iter.next(), Some(Value::Str("Ola".to_string())));
        assert_eq!(iter.next(), Some(Value::Int(123)));
        assert_eq!(iter.next(), Some(Value::Double(3.14)));
        assert_eq!(iter.next(), Some(Value::Bool(false)));
        assert_eq!(iter.next(), Some(Value::Vec(vec![Value::Int(10)])));
        assert_eq!(iter.next(), Some(Value::None));
        assert_eq!(iter.next(), Some(Value::Vec(vec![Value::Vec(vec![Value::Vec(vec![Value::Int(1)])])])));

        assert_eq!(iter.next(), None);
    
        Ok(())
    }

    #[test]
    fn test_value_iterator_empty_input() -> Result<(), String> {

        let input = "".chars();

        let iter = ValueIterator::new(input)
            .collect::<Result<Vec<Value>, _>>()
            .map_err(|e| e.to_string())?;
    
        assert_eq!(iter.len(), 0);
    
        Ok(())
    }

    #[test]
    fn test_value_iterator_empty_array() -> Result<(), String> {

        let input = "[]".chars();
        let iter = ValueIterator::new(input)
            .collect::<Result<Vec<Value>, _>>()
            .map_err(|e| e.to_string())?;

        println!("{:?}", iter);


        Ok(())
    }
}