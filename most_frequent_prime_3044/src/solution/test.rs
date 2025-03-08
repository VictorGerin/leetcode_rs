#[cfg(test)]

use super::solution::*;

#[test]
fn test_most_frequent_prime() -> Result<(), String> {
    use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
    let chars = read_input("input.txt")?;
    
    let values = ValueIterator::new(chars)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let matrix = caso_teste[0].clone()
                .as_vec::<_, _, Result<Vec<Vec<i32>>, String>>(|v| {
                    v.as_vec::<_, _, Result<Vec<i32>, String>>(|v| {
                        v.as_int().ok_or(format!("Expected integer"))
                    }).ok_or(format!("Expected array"))?
                }).ok_or(format!("Expected array, got {:?}", caso_teste[0]))??;

            let expected = caso_teste[1].clone()
                .as_int()
                .ok_or(format!("Invalid expected result: {:?}", caso_teste[1]))?;

            let result = Solution::most_frequent_prime(matrix.clone());
            assert_eq!(result, expected, "Expected result to be {}, but got {}", expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}