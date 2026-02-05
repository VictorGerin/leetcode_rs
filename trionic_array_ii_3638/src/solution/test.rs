#[cfg(test)]

use super::solution::*;

#[test]
fn trionic_array_ii_3638() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};

    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let nums = caso_teste[0].clone()
                .as_vec_int()
                .map_err(|x| format!("Expected array of integers, got {:?}", x))?;

            let expected = caso_teste[1].clone()
                .as_long()
                .map_err(|x| format!("Expected array of integers, got {:?}", x))?;

            let result = Solution::max_sum_trionic(nums.clone());
            assert_eq!(result, expected, "Expected result to be {:?}, but got {:?} for input nums={:?}", expected, result, nums);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
