#[cfg(test)]

use super::solution::*;

#[test]
fn add_binary_67() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let a = caso_teste[0].clone()
                .as_string()
                .map_err(|x| format!("Expected string, got {:?}", x))?;

            let b = caso_teste[1].clone()
                .as_string()
                .map_err(|x| format!("Expected string, got {:?}", x))?;

            let expected = caso_teste[2].clone()
                .as_string()
                .map_err(|x| format!("Expected string, got {:?}", x))?;

            let result = Solution::add_binary(a.clone(), b.clone());
            assert_eq!(result, expected, "Expected result to be {:?}, but got {:?} for input a={:?}, b={:?}", expected, result, a, b);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}

