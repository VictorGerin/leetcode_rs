#[cfg(test)]

use super::solution::*;


#[test]
fn sum_two_numbers() -> Result<(), String> {
    use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};

    let chars = read_input("input.txt")?;
    
    let values =ValueIterator::new(chars)
        .collect::<Result<Vec<Value>, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let a = caso_teste[0].clone()
                            .as_int()
                            .map_err(|x| format!("Invalid input for a: {:?}", x))?;
            let b = caso_teste[1].clone()
                            .as_int()
                            .map_err(|x| format!("Invalid input for b: {:?}", x))?;
            let expected = caso_teste[2].clone()
                                    .as_int()
                                    .map_err(|x| format!("Invalid expected sum: {:?}", x))?;

            let sum = Solution::sum_two_numbers(a, b);
            assert_eq!(sum, expected, "Expected sum of {} + {} to be {}, but got {}", a, b, expected, sum);

            Ok(())
        })?;

        println!("All test cases passed!");

    Ok(())
}