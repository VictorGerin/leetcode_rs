#[cfg(test)]

use super::solution::*;

#[test]
fn maximum_69_number_1323() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let num = caso_teste[0].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let expected = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let result = Solution::maximum69_number(num);
            assert_eq!(result, expected, "Expected result to be {}, but got {} for input {}", expected, result, num);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
