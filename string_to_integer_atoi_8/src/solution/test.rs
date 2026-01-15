#[cfg(test)]

use super::solution::*;

#[test]
fn string_to_integer_atoi_8() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let s = caso_teste[0].clone()
                .as_string()
                .map_err(|x| format!("Expected string, got {:?}", x))?;

            let expected = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let result = Solution::my_atoi(s.clone());
            assert_eq!(result, expected, "Expected result to be {}, but got {} for input \"{}\"", expected, result, s);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
