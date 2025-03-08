use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
mod solution;
use solution::Solution;

fn main() -> Result<(), String> {
    let chars = read_input("most_frequent_prime_3044/src/input.txt")?;

    ValueIterator::new(chars)
        .collect::<Result<Vec<Value> , ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {

            let matrix_data = caso_teste[0].clone()
                .as_vec::<_, _, Result<Vec<Vec<i32>>, String>>(|v| {
                    v.as_vec::<_, _, Result<Vec<i32>, String>>(|v| {
                        v.as_int().ok_or(format!("Expected integer"))
                    }).ok_or(format!("Expected array"))?
                }).ok_or(format!("Expected array, got {:?}", caso_teste[0]))??; 

            let expected = caso_teste[1].clone()
                .as_int().ok_or(format!("Invalid input: {:?}", caso_teste[1]))?;

            assert_eq!(Solution::most_frequent_prime(matrix_data), expected);

            Ok(())
        })?;

        println!("All test cases passed!");

    Ok(())
} 