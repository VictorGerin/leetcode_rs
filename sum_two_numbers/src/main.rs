use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
mod solution;
use solution::Solution;

fn main() -> Result<(), String> {
    let chars = read_input("sum_two_numbers/src/input.txt")?;

    let values: Vec<Value> = ValueIterator::new(chars)
        .collect::<Result<_, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let a = caso_teste[0].as_int().ok_or(format!("Invalid input for a: {:?}", caso_teste[0]))?;
            let b = caso_teste[1].as_int().ok_or(format!("Invalid input for b: {:?}", caso_teste[1]))?;
            let expected = caso_teste[2].as_int().ok_or(format!("Invalid expected sum: {:?}", caso_teste[2]))?;

            let sum = Solution::sum_two_numbers(a, b);
            assert_eq!(sum, expected, "Expected sum of {} + {} to be {}, but got {}", a, b, expected, sum);

            Ok(())
        })?;

        println!("All test cases passed!");

    Ok(())
} 