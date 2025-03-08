use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
use crate::solution::Solution;

mod solution;

fn main() -> Result<(), String> {
    let chars = read_input("integer_to_roman_12/src/input.txt")?;

    ValueIterator::new(chars)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let num = caso_teste[0].clone().as_int().ok_or(format!("Invalid input number: {:?}", caso_teste[0]))?;
            let expected = caso_teste[1].clone().as_string().ok_or_else(|| format!("Expected string value, got {:?}", caso_teste[1]))?;

            let result = Solution::int_to_roman(num);
            assert_eq!(result, expected, "Expected {} to be converted to {}, but got {}", num, expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
} 