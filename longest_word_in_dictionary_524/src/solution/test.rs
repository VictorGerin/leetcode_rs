#[cfg(test)]

use super::solution::*;

#[test]
fn longest_word_in_dictionary() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};

    let chars = read_input("input.txt")?;
    
    let values = ValIter::new(chars)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let s = caso_teste[0].clone()
                .as_string()
                .map_err(|x| format!("Invalid input for s: {:?}", x))?;

            let dictionary = caso_teste[1].clone()
                .as_vec()
                .map_err(|x| format!("Expected array, got {:?}", x))?
                .into_iter()
                .map(|x| -> Result<String, String> {
                    Ok(x.as_string()
                    .map_err(|x| format!("dictionary as_string: {:?}", x))?)
                }).collect::<Result<Vec<String>, String>>()?;

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
