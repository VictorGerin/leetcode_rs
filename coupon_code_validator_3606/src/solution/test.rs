#[cfg(test)]

use super::solution::*;

#[test]
fn coupon_code_validator_3606() -> Result<(), String> {
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(4)
        .try_for_each(|caso_teste| -> Result<(), String> {
            let code = caso_teste[0].clone()
                .as_vec()
                .map_err(|x| format!("Expected array, got {:?}", x))?
                .iter()
                .map(|v| v.clone().as_string().map_err(|e| format!("Expected string in array, got {:?}", e)))
                .collect::<Result<Vec<String>, String>>()?;

            let business_line = caso_teste[1].clone()
                .as_vec()
                .map_err(|x| format!("Expected array, got {:?}", x))?
                .iter()
                .map(|v| v.clone().as_string().map_err(|e| format!("Expected string in array, got {:?}", e)))
                .collect::<Result<Vec<String>, String>>()?;

            let is_active = caso_teste[2].clone()
                .as_vec()
                .map_err(|x| format!("Expected array, got {:?}", x))?
                .iter()
                .map(|v| v.clone().as_bool().map_err(|e| format!("Expected boolean in array, got {:?}", e)))
                .collect::<Result<Vec<bool>, String>>()?;

            let expected = caso_teste[3].clone()
                .as_vec()
                .map_err(|x| format!("Expected array, got {:?}", x))?
                .iter()
                .map(|v| v.clone().as_string().map_err(|e| format!("Expected string in array, got {:?}", e)))
                .collect::<Result<Vec<String>, String>>()?;

            let result = Solution::validate_coupons(code.clone(), business_line.clone(), is_active.clone());
            assert_eq!(result, expected, "Expected result to be {:?}, but got {:?} for input code={:?}, business_line={:?}, is_active={:?}", expected, result, code, business_line, is_active);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
