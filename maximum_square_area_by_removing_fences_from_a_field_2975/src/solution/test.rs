#[cfg(test)]

use super::solution::*;

#[test]
fn maximize_square_area_2975() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(5)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let m = caso_teste[0].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let n = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let h_fences = caso_teste[2].clone()
                .as_vec_int()
                .map_err(|x| format!("Expected array of integers, got {:?}", x))?;

            let v_fences = caso_teste[3].clone()
                .as_vec_int()
                .map_err(|x| format!("Expected array of integers, got {:?}", x))?;

            let expected = caso_teste[4].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let result = Solution::maximize_square_area(m, n, h_fences.clone(), v_fences.clone());
            assert_eq!(result, expected, "Expected result to be {}, but got {} for input m={}, n={}, h_fences={:?}, v_fences={:?}", expected, result, m, n, h_fences, v_fences);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
