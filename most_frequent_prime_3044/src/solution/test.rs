#[cfg(test)]

use super::solution::*;

#[test]
fn most_frequent_prime() -> Result<(), String> {
    use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
    let chars = read_input("input.txt")?;
    
    ValueIterator::new(chars)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let matrix = caso_teste[0].clone()
                .as_vec::<_, _, Result<Vec<Vec<i32>>, String>>(|v| {
                    v.as_vec::<_, _, Result<Vec<i32>, String>>(|v| {
                        v.as_int().map_err(|x| format!("Expected integer, got {:?}", x))
                    }).map_err(|x| format!("Expected array, got {:?}", x))?
                }).map_err(|x| format!("Expected array, got {:?}", x))??;

            let expected = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Invalid expected result: {:?}", x))?;

            let result = Solution::most_frequent_prime(matrix.clone());
            assert_eq!(result, expected, "Expected result to be {}, but got {}", expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}