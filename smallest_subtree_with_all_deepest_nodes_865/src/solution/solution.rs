use std::{cell::RefCell, cmp::Ordering, collections::HashMap};

use leetcode_lib::data_structures::{TreeNode, TreeNodeIterPostOrder, TreeNodeRef};

pub struct Solution;


impl Solution {
    /// Dada a raiz de uma árvore binária, a profundidade de cada nó é a menor distância até a raiz.
    /// Retorna a menor subárvore tal que ela contenha todos os nós mais profundos na árvore original.
    /// Um nó é chamado de mais profundo se ele tem a maior profundidade possível entre qualquer nó na árvore inteira.
    /// A subárvore de um nó é uma árvore consistindo desse nó, mais o conjunto de todos os descendentes desse nó.
    pub fn subtree_with_all_deepest(root: Option<TreeNodeRef>) -> Option<TreeNodeRef> {

        let mut deep_map= HashMap::<*const RefCell<TreeNode>, i32>::new();

        for node in TreeNodeIterPostOrder::new(root.clone()) {

            let left_deep = node.borrow().left.as_ref()
                .map(|l| deep_map.get(&(l.as_ptr() as *const RefCell<TreeNode>)).copied().unwrap())
                .unwrap_or(0);

            let right_deep = node.borrow().right.as_ref()
                .map(|l| deep_map.get(&(l.as_ptr() as *const RefCell<TreeNode>)).copied().unwrap())
                .unwrap_or(0);

            let node_deep = left_deep.max(right_deep) + 1;
            deep_map.insert(node.as_ptr() as *const RefCell<TreeNode>, node_deep);
        
        }

        let mut current_node = root?;
        Some(loop {

            let left = current_node.borrow().left.as_ref()
                .map(|l| 
                    deep_map.get(&(l.as_ptr() as *const RefCell<TreeNode>))
                    .copied()
                    .unwrap()
                )
                .unwrap_or(0);
            
            let right = current_node.borrow().right.as_ref()
                .map(|r| 
                    deep_map.get(&(r.as_ptr() as *const RefCell<TreeNode>))
                    .copied()
                    .unwrap()
                )
                .unwrap_or(0);


            current_node = match left.cmp(&right) {
                Ordering::Less => {
                    current_node.borrow().right.clone()?
                }
                Ordering::Greater => {
                    current_node.borrow().left.clone()?
                }
                Ordering::Equal => {
                    break current_node;
                }
            }
            
        })


    }
}
