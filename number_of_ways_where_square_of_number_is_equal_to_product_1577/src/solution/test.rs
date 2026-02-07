#[cfg(test)]

use super::solution::*;

#[test]
fn number_of_ways_where_square_of_number_is_equal_to_product_1577() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    let chars = read_input("input.txt")?;
    
    ValIter::new(chars)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(3)
        .try_for_each(|caso_teste| -> Result<(), String> {

            let nums1 = caso_teste[0].clone()
                .as_vec_int()
                .map_err(|x| format!("{:?}", x))?;

            let nums2 = caso_teste[1].clone()
                .as_vec_int()
                .map_err(|x| format!("{:?}", x))?;

            let expected = caso_teste[2].clone()
                .as_int()
                .map_err(|x| format!("Invalid expected result: {:?}", x))?;
    

            let solution = Solution::num_triplets(nums1, nums2);

            assert_eq!(solution, expected, "Expected result to be {}, but got {}", expected, solution);
            
            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}