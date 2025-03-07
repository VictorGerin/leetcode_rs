use leetcode_lib::parser::{Value, ValueIterator, ProcessInputError, read_input};
mod solution;
use solution::Solution;

fn main() -> Result<(), String> {
    let chars = read_input("most_frequent_prime_3044/src/input.txt")?;

    let values: Vec<Value> = ValueIterator::new(chars)
        .collect::<Result<_, ProcessInputError>>()
        .map_err(|e| e.to_string())?;

    values.chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let Value::Vec(matrix_data) = &caso_teste[0]
            else { return Err(format!("Invalid input: {:?}", &caso_teste[0])) };

            // Convert the input into a 2D matrix
            let matrix = matrix_data.into_iter()
                .map(|row| {
                    if let Value::Vec(row_data) = row {
                        Ok(
                            row_data.into_iter()
                            .map(|x| x.as_int().ok_or(format!("Invalid input: {:?}", x)))
                            .collect::<Result<Vec<i32>, String>>()?
                        )
                    } else {
                        Err("Invalid matrix row format".to_string())
                    }
                })
                .collect::<Result<Vec<Vec<i32>>, String>>()?;

            let expected = caso_teste[1].as_int().ok_or(format!("Invalid input: {:?}", caso_teste[1]))?;

            assert_eq!(Solution::most_frequent_prime(matrix), expected);

            Ok(())
        })?;

        println!("All test cases passed!");

    Ok(())
} 