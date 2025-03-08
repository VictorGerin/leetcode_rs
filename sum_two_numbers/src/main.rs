use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
mod solution;
use solution::Solution;

fn main() -> Result<(), String> {
    let chars = read_input("sum_two_numbers/src/input.txt")?;
    
    ValueIterator::new(chars)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let a = caso_teste[0].clone().as_int().ok_or(format!("Invalid input for a: {:?}", caso_teste[0]))?;
            let b = caso_teste[1].clone().as_int().ok_or(format!("Invalid input for b: {:?}", caso_teste[1]))?;
            let expected = caso_teste[2].clone().as_int().ok_or(format!("Invalid expected sum: {:?}", caso_teste[2]))?;

            let sum = Solution::sum_two_numbers(a, b);
            assert_eq!(sum, expected, "Expected sum of {} + {} to be {}, but got {}", a, b, expected, sum);

            Ok(())
        })?;

        println!("All test cases passed!");

    Ok(())
} 