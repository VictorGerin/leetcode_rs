#[cfg(test)]

use super::solution::*;

#[test]
fn maximize_area_of_square_hole_in_grid_2943() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(5)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let n = caso_teste[0].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let m = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let h_bars = caso_teste[2].clone()
                .as_array()
                .map_err(|x| format!("Expected array, got {:?}", x))?
                .iter()
                .map(|v| v.as_int().map_err(|e| format!("Expected integer in array, got {:?}", e)))
                .collect::<Result<Vec<i32>, String>>()?;

            let v_bars = caso_teste[3].clone()
                .as_array()
                .map_err(|x| format!("Expected array, got {:?}", x))?
                .iter()
                .map(|v| v.as_int().map_err(|e| format!("Expected integer in array, got {:?}", e)))
                .collect::<Result<Vec<i32>, String>>()?;

            let expected = caso_teste[4].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let result = Solution::maximize_square_hole_area(n, m, h_bars.clone(), v_bars.clone());
            assert_eq!(result, expected, "Expected result to be {}, but got {} for input n={}, m={}, h_bars={:?}, v_bars={:?}", expected, result, n, m, h_bars, v_bars);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
