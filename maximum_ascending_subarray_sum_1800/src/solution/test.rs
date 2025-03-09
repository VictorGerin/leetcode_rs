#[cfg(test)]

use super::solution::*;

#[test]
fn maximum_ascending_subarray_sum() -> Result<(), String> {
    use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};

    let chars = read_input("input.txt")?;
    
    let values = ValueIterator::new(chars)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let nums = caso_teste[0].clone()
                .as_vec_int()
                .map_err(|x| format!("Expected array of integer, got {:?}", x))?;

            let expected = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Invalid expected result: {:?}", x))?;

            let result = Solution::max_ascending_sum(nums.clone());
            assert_eq!(result, expected, "Expected result to be {}, but got {}", expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}