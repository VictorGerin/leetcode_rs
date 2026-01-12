#[cfg(test)]

use super::solution::*;

#[test]
fn smallest_subtree_with_all_deepest_nodes_865() -> Result<(), String> {
    use std::rc::Rc;
    use std::cell::RefCell;
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    
    ValIter::new(read_input("input.txt")?)
        .collect::<Result<Vec<Val>, ProcessInputError>>()
        .map_err(|e| e.to_string())?
        .chunks_exact(2)
        .try_for_each(|caso_teste| -> Result<(), String> {
            // Primeiro elemento: árvore de entrada (formato level-order)
            let root = {
                let tree = caso_teste[0].clone()
                    .as_tree_node()
                    .map_err(|e| format!("Failed to convert to TreeNode: {:?}", e))?;
                Some(Rc::new(RefCell::new(tree)))
            };

            // Segundo elemento: árvore esperada (formato level-order)
            let expected = {
                let tree = caso_teste[1].clone()
                    .as_tree_node()
                    .map_err(|e| format!("Failed to convert to TreeNode: {:?}", e))?;
                Some(Rc::new(RefCell::new(tree)))
            };

            let result = Solution::subtree_with_all_deepest(root.clone());
            
            // Comparar as árvores resultantes usando as_post_ordem_vec
            let result_vec = match &result {
                Some(ref_node) => ref_node.borrow().as_post_ordem_vec(),
                None => Vec::new(),
            };
            
            let expected_vec = match &expected {
                Some(ref_node) => ref_node.borrow().as_post_ordem_vec(),
                None => Vec::new(),
            };
            
            assert_eq!(result_vec, expected_vec, 
                "Expected trees to be equal for input tree. Result: {:?}, Expected: {:?}", 
                result_vec, expected_vec);

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
