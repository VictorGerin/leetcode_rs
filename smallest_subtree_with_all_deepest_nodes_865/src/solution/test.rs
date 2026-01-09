#[cfg(test)]

use super::solution::*;

// Helper function para comparar duas árvores
fn trees_equal(a: &Option<leetcode_lib::data_structures::TreeNodeRef>, 
               b: &Option<leetcode_lib::data_structures::TreeNodeRef>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a_ref), Some(b_ref)) => {
            let a_node = a_ref.borrow();
            let b_node = b_ref.borrow();
            a_node.val == b_node.val 
                && trees_equal(&a_node.left, &b_node.left)
                && trees_equal(&a_node.right, &b_node.right)
        },
        _ => false,
    }
}

#[test]
fn smallest_subtree_with_all_deepest_nodes_865() -> Result<(), String> {
    use std::rc::Rc;
    use std::cell::RefCell;
    use leetcode_lib::parser::{Val, ValIter, ProcessInputError, read_input};
    use leetcode_lib::data_structures::TreeNodeRef;
    
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
            
            // Comparar as árvores resultantes
            assert!(trees_equal(&result, &expected), 
                "Expected trees to be equal for input tree");

            Ok(())
        })?;

    println!("All test cases passed!");
    Ok(())
}
