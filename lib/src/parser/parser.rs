use super::{Val, ProcessInputError};

///
/// parse a JSON-like string defined by LeetCode
/// https://support.leetcode.com/hc/en-us/articles/32442719377939-How-to-create-test-cases-on-LeetCode
/// 
/// This function is a alias to leetcode_lib::parser::parser function as 
/// sometimes there is no need to process varius times the same input until the end
/// 
/// # Arguments
/// str: A string slice
///
/// # Returns
/// A Result<Value, ProcessInputError> where Value is the parsed value and ProcessInputError is an error enum
/// 
/// # Example
/// ```
/// use leetcode_lib::parser::{parser_str, Val};
/// 
/// let input = r#"
///     "Hello, \"World!\""
/// "#;
/// let result = parser_str(input);
/// 
/// assert_eq!(result, Ok(Val::Str(r#"Hello, "World!""#.to_string())));
/// ```
/// 
pub fn parser_str(str: &str) -> Result<Val, ProcessInputError>
{
    return parser(str.chars());
}

///
/// Parse a JSON-like string defined by LeetCode
/// https://support.leetcode.com/hc/en-us/articles/32442719377939-How-to-create-test-cases-on-LeetCode
/// 
/// # Arguments
/// iter: An iterator of char
/// 
/// # Returns
/// A Result<Value, ProcessInputError> where Value is the parsed value and ProcessInputError is an error enum
/// 
/// # Example
/// ```
/// use leetcode_lib::parser::{parser, Val};
/// 
/// let input = r#"
///     "Hello, \"World!\""
/// "#;
/// let result = parser(input.chars());
/// 
/// assert_eq!(result, Ok(Val::Str(r#"Hello, "World!""#.to_string())));
/// ```
/// 
/// 
pub fn parser<I>(iter: I) -> Result<Val, ProcessInputError>
where 
    I: Iterator<Item = char>
{
    #[derive(Debug, Clone, Copy)]
    enum State {
        StartStateMachine,
        List,
        String,
        Number,
        Boolean,
        LineComment,
        MultLineComment,
        End
    }

    let mut last_state: State = State::End;
    let mut state: State = State::StartStateMachine;
    let mut current_val: Option<Val> = None;
    let mut stack_lst: Vec<Vec<Val>> = vec![];
    let mut current_text = String::new();

    let mut iter = iter.chain(std::iter::once(' ')).peekable();

    while let Some(c) = iter.peek().copied() {
        match state {
            State::StartStateMachine => {
                match c {
                    ' ' | '\n' | '\r' | '\t' => {
                        iter.next();
                    },
                    '[' => {
                        state = State::List;
                        stack_lst.push(vec![]);
                        iter.next();
                    },
                    '"' => {
                        state = State::String;
                        iter.next();
                    },
                    '0'..='9' | '-' | '+' => {
                        state = State::Number;
                        current_text.push(c);
                        iter.next();
                    },
                    'f' | 't' | 'F' | 'T' | 'n' | 'N' => {
                        state = State::Boolean;
                        current_text.push(c);
                        iter.next();
                    },
                    '/' => {
                        iter.next();

                        let Some(next_char) = iter.peek()
                        else {
                            return Err(ProcessInputError::InvalidInput);
                        };
                        
                        match next_char {
                            '/' => {
                                state = State::LineComment;
                                iter.next();
                            },
                            '*' => {
                                state = State::MultLineComment;
                                iter.next();
                            },
                            _ => {
                                return Err(ProcessInputError::InvalidInput);
                            }
                        }
                    },
                    _ => {
                        return Err(ProcessInputError::InvalidInput);
                    }
                }
            },
            State::List => {
                match c {
                    ' ' | '\n' | '\r' | '\t' => {
                        iter.next();
                    },
                    '[' => {
                        stack_lst.push(vec![]);
                        iter.next();
                    },
                    ',' =>  {
                        let Some(top) = stack_lst.last_mut() else {
                            return Err(ProcessInputError::UnexpectedError("Empty stack".to_string()));
                        };

                        if let Some(val) = current_val.take() {
                            top.push(val);
                        }
                        iter.next();
                    },
                    ']' => {
                        let Some(mut top) = stack_lst.pop() else {
                            return Err(ProcessInputError::UnexpectedError("Empty stack".to_string()));
                        };

                        if let Some(val) = current_val.take() {
                            top.push(val);
                        }

                        current_val = Some(Val::Vec(top));
                        
                        if stack_lst.len() == 0 {
                            state = State::End;
                        }
                        
                        iter.next();
                    },
                    '"' => {
                        last_state = State::List;
                        state = State::String;
                        iter.next();
                    }
                    '0'..='9' | '-' | '+' => {
                        last_state = State::List;
                        state = State::Number;
                        current_text.push(c);
                        iter.next();
                    },
                    'f' | 't' | 'F' | 'T' | 'n' | 'N' => {
                        last_state = State::List;
                        state = State::Boolean;
                        current_text.push(c);
                        iter.next();
                    },
                    _ => {
                        return Err(ProcessInputError::InvalidInput);
                    }
                }
            },
            State::String => {
                match c {
                    '"' => {
                        state = last_state;
                        current_val = Some(Val::Str(current_text.clone()));
                        current_text.clear();
                        iter.next();
                    },
                    '\\' => {
                        iter.next();
                        if let Some(next_char) = iter.next() {
                            match next_char {
                                'n' => current_text.push('\n'),
                                '\\' => current_text.push('\\'),
                                '"' => current_text.push('"'),
                                _ => return Err(ProcessInputError::InvalidEscapeCharacter(format!("\"\\{}\" is not a valid escape character", next_char)))
                            }
                        }
                    },
                    _ => {
                        current_text.push(c);
                        iter.next();
                    }
                }
            },
            State::Number => {
                match c {
                    '0'..='9' | '-' | '+' | '.' => {
                        current_text.push(c);
                        iter.next();
                    },
                    _ => {
                        state = last_state;
                        if current_text.contains(".") {
                            current_val = Some(
                                Val::Double(
                                    current_text.parse::<f64>()
                                    .map_err(|_| ProcessInputError::UnexpectedError(format!("\"{}\" is not a valid double", current_text)))?
                                )
                            );
                        } else {
                            current_val = Some(
                                Val::Int(
                                    current_text.parse::<i128>()
                                    .map_err(|_| ProcessInputError::UnexpectedError(format!("\"{}\" is not a valid integer", current_text)))?
                                )
                            );
                        }
                        current_text.clear();
                    }
                }
            },
            State::Boolean => {
                match c {
                    'a'..='z' | 'A'..='Z' => {
                        current_text.push(c);
                        iter.next();
                    },
                    _ => {
                        state = last_state;
                        current_val = Some(match current_text.to_lowercase().as_str() {
                            "true" => Some(Val::Bool(true)),
                            "false" => Some(Val::Bool(false)),
                            "null" => Some(Val::None),
                            _ => None  
                        }.ok_or_else(|| ProcessInputError::InvalidKeyWord(format!("\"{}\" is not a key word: true, false ou null", current_text)))?);
                        current_text.clear();
                    }
                }
            },
            State::LineComment => {
                match c {
                    '\n' | '\r' => {
                        state = last_state;
                        iter.next();
                    },
                    _ => {
                        iter.next();
                    }
                }
            },
            State::MultLineComment => {
                match c {
                    '*' => {
                        iter.next();
                        if let Some('/') = iter.peek() {
                            state = last_state;
                            iter.next();
                        }
                    },
                    _ => {
                        iter.next();
                    }
                }
            },
            State::End => {
                break;
            },
        }
    }

    Ok(current_val.ok_or_else(|| ProcessInputError::EmptyInput)?)
}

///
/// Test and examples for parser function
/// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_list() {
        let input = r#"
        [
            "Hello, World!",
            42,
            3.14,
            true,
            false,
            null,
            [
                "Nested list"
            ]
        ]
        "#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::Vec(vec![
                Val::Str("Hello, World!".to_string()),
                Val::Int(42),
                Val::Double(3.14),
                Val::Bool(true),
                Val::Bool(false),
                Val::None,
                Val::Vec(vec![
                    Val::Str("Nested list".to_string())
                ])
            ]))
        );
    }

    #[test]
    fn empty_list() {
        let input = r#"[]"#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::Vec(vec![]))
        );
    }

    #[test]
    fn empty_list2() {
        let input = r#"[,]"#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::Vec(vec![]))
        );
    }

    #[test]
    fn parser_string() {
        let input = r#""Hello, \"World!\" \\ <- this is a bar""#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::Str(r#"Hello, "World!" \ <- this is a bar"#.to_string()))
        );
    }

    #[test]
    fn parser_number() {
        let input = r#"42"#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::Int(42))
        );
    }

    #[test]
    fn parser_double() {
        let input = r#"3.14"#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::Double(3.14))
        );
    }

    #[test]
    fn parser_boolean() {
        let input = r#"true"#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::Bool(true))
        );

        let input = r#"false"#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::Bool(false))
        );

        let input = r#"null"#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Ok(Val::None)
        );
    }

    #[test]
    fn parser_invalid_input() {
        let input = r#"{"#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Err(ProcessInputError::InvalidInput)
        );
    }

    #[test]
    fn parser_comment() {
        let input = r#"
        //this is a comment
        "#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Err(ProcessInputError::EmptyInput)
        );

        let input = r#"
        /*
            This is a mult line comment
        */
        "#;

        let result = parser_str(input);

        assert_eq!(
            result,
            Err(ProcessInputError::EmptyInput)
        );
    }

    #[test]
    fn parser_n_values() {

        let mut input = r#"
        "Hello, World!"
        42
        3.14
        true
        "#.chars();

        let result = parser(input.by_ref());
        assert_eq!(
            result,
            Ok(Val::Str("Hello, World!".to_string()))
        );

        let result = parser(input.by_ref());
        assert_eq!(
            result,
            Ok(Val::Int(42))
        );

        let result = parser(input.by_ref());
        assert_eq!(
            result,
            Ok(Val::Double(3.14))
        );

        let result = parser(input.by_ref());
        assert_eq!(
            result,
            Ok(Val::Bool(true))
        );


        let result = parser(input.by_ref());
        assert_eq!(
            result,
            Err(ProcessInputError::EmptyInput)
        );

    }
}