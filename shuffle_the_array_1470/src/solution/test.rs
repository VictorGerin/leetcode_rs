#[cfg(test)]

use super::solution::*;

#[test]
fn shuffle_the_array_1470() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let nums = caso_teste[0].clone()
                .as_vec_int()
                .map_err(|x| format!("Expected array, got {:?}", x))?;

            let n = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let expected = caso_teste[2].clone()
                .as_vec_int()
                .map_err(|x| format!("Expected array, got {:?}", x))?;

            let result = Solution::shuffle(nums.clone(), n);
            assert_eq!(result, expected, "Expected result to be {:?}, but got {:?} for input nums={:?}, n={}", expected, result, nums, n);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}


