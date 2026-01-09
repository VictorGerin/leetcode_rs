#[cfg(test)]

use super::solution::*;

#[test]
fn ways_to_express_integer_2787() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let n = caso_teste[0].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let x = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let expected = caso_teste[2].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let result = Solution::number_of_ways(n, x);
            assert_eq!(result, expected, "Expected result to be {}, but got {} for input n={}, x={}", expected, result, n, x);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
