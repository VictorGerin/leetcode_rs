#[cfg(test)]

use super::solution::*;

#[test]
fn new_21_game_837() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(4)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let n = caso_teste[0].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let k = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let max_pts = caso_teste[2].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let expected = caso_teste[3].clone()
                .as_double()
                .map_err(|x| format!("Expected float, got {:?}", x))?;

            let result = Solution::new21_game(n, k, max_pts);
            let diff = (result - expected).abs();
            assert!(diff < 1e-5, "Expected result to be approximately {}, but got {} for input n={}, k={}, max_pts={}. Difference: {}", expected, result, n, k, max_pts, diff);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
