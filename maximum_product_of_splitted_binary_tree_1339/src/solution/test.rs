#[cfg(test)]

use super::solution::*;

#[test]
fn maximum_product_of_splitted_binary_tree_1339() -> Result<(), String> {
    use std::rc::Rc;
    use std::cell::RefCell;
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {

            // Converter Vec<Val> para TreeNodeRef usando collect()
            // FromIterator<Val> for TreeNodeRef espera um iterador de Val
            let root = {
                let tree = caso_teste[0].clone()
                    .as_tree_node()
                    .map_err(|e| format!("Failed to convert to TreeNode: {:?}", e))?;
                Some(Rc::new(RefCell::new(tree)))
            };

            let expected = caso_teste[1].clone()
                .as_int()
                .map_err(|x| format!("Expected integer, got {:?}", x))?;

            let result = Solution::max_product(root);
            assert_eq!(result, expected, "Expected result to be {}, but got {}", expected, result);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}

