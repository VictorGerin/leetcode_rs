use super::{Value, ProcessInputError};

pub fn process_input<I>(iter: I) -> Result<Value, ProcessInputError>
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
        End
    }

    let mut last_state: State = State::End;
    let mut state: State = State::StartStateMachine;
    let mut current_val: Option<Value> = None;
    let mut stack_lst: Vec<Vec<Value>> = vec![];
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
                        stack_lst.last_mut()
                        .ok_or(ProcessInputError::UnexpectedError("Empty stack".to_string()))?
                        .push(current_val.ok_or(ProcessInputError::UnexpectedError("Current value empty".to_string()))?.clone());
                        current_val = None;
                        iter.next();
                    },
                    ']' => {
                        stack_lst.last_mut()
                        .ok_or(ProcessInputError::UnexpectedError("Empty stack".to_string()))?
                        .push(current_val.ok_or(ProcessInputError::UnexpectedError("Current value empty".to_string()))?.clone());

                        current_val = Some(Value::Vec(stack_lst.pop().ok_or(ProcessInputError::UnexpectedError("Empty stack".to_string()))?));
                        
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
                        current_val = Some(Value::Str(current_text.clone()));
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
                                Value::Double(
                                    current_text.parse::<f64>()
                                    .map_err(|_| ProcessInputError::UnexpectedError(format!("\"{}\" is not a valid double", current_text)))?
                                )
                            );
                        } else {
                            current_val = Some(
                                Value::Int(
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
                            "true" => Some(Value::Bool(true)),
                            "false" => Some(Value::Bool(false)),
                            "null" => Some(Value::None),
                            _ => None  
                        }.ok_or(ProcessInputError::InvalidKeyWord(format!("\"{}\" is not a key word: true, false ou null", current_text)))?);
                        current_text.clear();
                    }
                }
            },
            State::End => {
                break;
            },
        }
    }

    Ok(current_val.ok_or(ProcessInputError::EmptyInput)?)
} 