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
                .ok_or(format!("Invalid input for s: {:?}", caso_teste[0]))?;

            let dictionary = caso_teste[1].clone()
                .as_vec::<_, _, Result<Vec<String>, String>>(|v| {
                    v.as_string().ok_or(format!("Invalid dictionary"))
                }).ok_or(format!("Expected vector for dictionary, got: {:?}", caso_teste[1]))??;

            let expected = caso_teste[2].clone()
                .as_string()
                .ok_or(format!("Invalid expected result: {:?}", caso_teste[2]))?;

            let result = Solution::find_longest_word(s, dictionary);
            assert_eq!(result, expected, "Expected result to be '{}', but got '{}'", expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
