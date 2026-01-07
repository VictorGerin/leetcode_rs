use std::rc::Rc;
use std::cell::RefCell;
use leetcode_lib::data_structures::TreeNode;

pub struct Solution;

impl Solution {
    pub fn max_product(_root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let total_sum = Self::dfs(_root.clone());
        println!("total_sum: {}", total_sum);
        todo!()

    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = Self::dfs(node.borrow().left.clone());
                let right = Self::dfs(node.borrow().right.clone());
                node.borrow().val + left + right
            }
        }
    }
}

