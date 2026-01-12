use std::{collections::HashMap, rc::Rc};
use std::cell::RefCell;
use leetcode_lib::data_structures::{TreeNode, TreeNodeIterPostOrder};

pub struct Solution;


impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        
        // Passo 1: Calcular soma total usando iterador pós-ordem
        let total_sum: i64 = TreeNodeIterPostOrder::new(root.clone())
            .map(|node| node.borrow().val as i64)
            .sum();
        
        // Passo 2: Calcular somas de subárvores e produtos usando HashMap
        // Usamos ponteiros como chave para identificar nós únicos
        let mut subtree_sums: HashMap<*const RefCell<TreeNode>, i64> = HashMap::new();
        let mut max_product = 0i64;
        
        for node in TreeNodeIterPostOrder::new(root.clone()) {
            let node_val = node.borrow().val as i64;
            
            // Obter somas dos filhos (já calculadas porque estamos em pós-ordem)
            let left_sum = node.borrow().left.as_ref()
                .map(|l| *subtree_sums.get(&(l.as_ptr() as *const RefCell<TreeNode>)).unwrap())
                .unwrap_or(0);
            
            let right_sum = node.borrow().right.as_ref()
                .map(|r| *subtree_sums.get(&(r.as_ptr() as *const RefCell<TreeNode>)).unwrap())
                .unwrap_or(0);
            
            // Calcular soma da subárvore enraizada neste nó
            let subtree_sum = node_val + left_sum + right_sum;
            subtree_sums.insert(node.as_ptr() as *const RefCell<TreeNode>, subtree_sum);
            
            // Calcular produto: subárvore * (total - subárvore)
            let product = subtree_sum * (total_sum - subtree_sum);
            max_product = max_product.max(product);
        }
        
        (max_product % MOD) as i32
    }
}

