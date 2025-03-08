#[cfg(test)]

use super::solution::*;


#[test]
fn test_sum_two_numbers() -> Result<(), String> {
    use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};

    let chars = read_input("input.txt")?;
    
    let values =ValueIterator::new(chars)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    println!("Testing n test cases: {:?}", values.len());

    values.chunks_exact(3)
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