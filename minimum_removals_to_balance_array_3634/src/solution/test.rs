#[cfg(test)]
use super::solution::*;

#[cfg(test)]
use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};

#[test]
fn minimum_removals_to_balance_array_3634() -> Result<(), String> {
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let nums = caso_teste[0].clone()
                .as_vec_int()
                .map_err(|x| format!("Expected array of integers, got {:?}", x))?;
            let k = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected k as integer, got {:?}", x))?;
            let expected = caso_teste[2].clone()
                .as_int()
                .map_err(|x| format!("Expected result as integer, got {:?}", x))?;

            let result = Solution::min_removal(nums.clone(), k);
            assert_eq!(
                result, expected,
                "Expected result to be {:?}, but got {:?} for input nums={:?}, k={:?}",
                expected, result, nums, k
            );

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
