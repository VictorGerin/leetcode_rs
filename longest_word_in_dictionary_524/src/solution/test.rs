#[cfg(test)]

use super::solution::*;

#[test]
fn test_find_longest_word() -> Result<(), String> {
    use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};

    let chars = read_input("input.txt")?;
    
    let values = ValueIterator::new(chars)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let s = caso_teste[0].clone()
                .as_string()
                .map_err(|x| format!("Invalid input for s: {:?}", x))?;

            let dictionary = caso_teste[1].clone()
                .as_vec::<_, _, Result<Vec<String>, String>>(|v| {
                    v.as_string().map_err(|x| format!("Invalid dictionary, got: {:?}", x))
                }).map_err(|x| format!("Expected vector for dictionary, got: {:?}", x))??;

            let expected = caso_teste[2].clone()
                .as_string()
                .map_err(|x| format!("Invalid expected result: {:?}", x))?;

            let result = Solution::find_longest_word(s, dictionary);
            assert_eq!(result, expected, "Expected result to be '{}', but got '{}'", expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
