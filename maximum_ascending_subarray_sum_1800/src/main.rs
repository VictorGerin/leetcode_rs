use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
use crate::solution::Solution;

mod solution;

fn main() -> Result<(), String> {
    let chars = read_input("maximum_ascending_subarray_sum_1800/src/input.txt")?;

    let values: Vec<Value> = ValueIterator::new(chars)
        .collect::<Result<_, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let nums = match &caso_teste[0] {
                Value::Vec(v) => v.iter()
                    .map(|val| val.as_int().ok_or_else(|| format!("Expected integer, got {:?}", val)))
                    .collect::<Result<Vec<i32>, String>>()?,
                _ => return Err(format!("Expected array, got {:?}", caso_teste[0]))
            };

            let expected = caso_teste[1].as_int()
                .ok_or_else(|| format!("Expected integer, got {:?}", caso_teste[1]))?;

            let result = Solution::max_ascending_sum(nums.clone());
            assert_eq!(result, expected, "For input {:?}, expected {}, but got {}", nums, expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
} 