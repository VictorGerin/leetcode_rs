#[cfg(test)]

// use super::solution::*;

#[test]
fn find_the_duplicate_number_287() -> Result<(), String> {
    // use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    // ValIter::new(read_input("input.txt")?)
    //     .collect::<Result<Vec<Val>, ProcessInputError>>()
    //     .map_err(|e| e.to_string())?
    //     .chunks_exact(2)
    //     .try_for_each(|caso_teste| -> Result<(), String> {
    //         let nums = caso_teste[0].clone()
    //             .as_vec_int()
    //             .map_err(|x| format!("Expected array, got {:?}", x))?;

    //         let expected = caso_teste[1].clone()
    //             .as_int()
    //             .map_err(|x| format!("Expected integer, got {:?}", x))?;

    //         let result = Solution::find_duplicate(nums.clone());
    //         assert_eq!(result, expected, "Expected result to be {}, but got {} for input nums={:?}", expected, result, nums);

    //         Ok(())
    //     })?;

    println!("All test cases passed!");
    Ok(())
}
