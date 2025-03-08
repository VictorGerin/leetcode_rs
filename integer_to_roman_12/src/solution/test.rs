#[cfg(test)]

use super::solution::*;

#[test]
fn test_sort_array() -> Result<(), String> {
    use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
    
    ValueIterator::new(read_input("input.txt")?)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let nums = caso_teste[0].clone()
                .as_int()
                .map_err(|x| format!("Expected array, got {:?}", x))?;

            let expected = caso_teste[1].clone()
                .as_string()
                .map_err(|x| format!("Expected array, got {:?}", x))?;

            let result = Solution::int_to_roman(nums.clone());
            assert_eq!(result, expected, "Expected result to be {:?}, but got {:?}", expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
