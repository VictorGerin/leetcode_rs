use std::{collections::HashMap, rc::Rc};
use std::cell::RefCell;
use leetcode_lib::data_structures::{TreeNode, TreeNodeRef};

pub struct Solution;

enum StackItem {
    Unvisited(TreeNodeRef),
    Visited(TreeNodeRef),
}

pub struct TreeNodeIterPostOrder {
    stack: Vec<StackItem>,
}

impl Iterator for TreeNodeIterPostOrder {
    type Item = TreeNodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            match item {
                StackItem::Visited(node) => {
                    // Já processamos os filhos, retornar o nó
                    return Some(node);
                }
                StackItem::Unvisited(node) => {
                    // Marcar como visitado e empilhar filhos
                    // Empilhamos na ordem: pai (visited), direito, esquerdo
                    // Assim processamos: esquerdo, direito, pai (pós-ordem)
                    self.stack.push(StackItem::Visited(node.clone()));
                    
                    if let Some(right) = node.borrow().right.clone() {
                        self.stack.push(StackItem::Unvisited(right));
                    }
                    
                    if let Some(left) = node.borrow().left.clone() {
                        self.stack.push(StackItem::Unvisited(left));
                    }
                }
            }
        }
        None
    }
}

impl TreeNodeIterPostOrder {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            stack: match root {
                Some(root) => vec![StackItem::Unvisited(root)],
                None => vec![]
            }
        }
    }
}

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

