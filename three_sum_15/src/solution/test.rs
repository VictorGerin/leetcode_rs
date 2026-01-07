#[cfg(test)]

use super::solution::*;

#[test]
fn three_sum_15() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let nums = caso_teste[0].clone()
                .as_vec_int()
                .map_err(|x| format!("Expected array, got {:?}", x))?;

            let expected_vec = caso_teste[1].clone()
                .as_vec()
                .map_err(|x| format!("Expected array of arrays, got {:?}", x))?;

            let expected: Vec<Vec<i32>> = expected_vec
                .into_iter()
                .map(|val| {
                    val.as_vec_int()
                        .map_err(|x| format!("Expected array of integers, got {:?}", x))
                })
                .collect::<Result<Vec<Vec<i32>>, String>>()?;

            let result = Solution::three_sum(nums.clone());
            
            // Normalizar resultados para comparação (ordenar cada triplet e a lista de triplets)
            let mut normalized_result: Vec<Vec<i32>> = result
                .into_iter()
                .map(|mut triplet| {
                    triplet.sort();
                    triplet
                })
                .collect();
            normalized_result.sort();

            let mut normalized_expected: Vec<Vec<i32>> = expected
                .into_iter()
                .map(|mut triplet| {
                    triplet.sort();
                    triplet
                })
                .collect();
            normalized_expected.sort();

            assert_eq!(
                normalized_result, 
                normalized_expected, 
                "Expected result to be {:?}, but got {:?} for input {:?}", 
                normalized_expected, 
                normalized_result, 
                nums
            );

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}

