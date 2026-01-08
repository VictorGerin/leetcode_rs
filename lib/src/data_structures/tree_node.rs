use std::{rc::Rc, cell::RefCell};
use crate::parser::Val;

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

impl FromIterator<Val> for TreeNodeRef {
    fn from_iter<I: IntoIterator<Item=Val>>(iter: I) -> TreeNodeRef {
        let mut iter = iter.into_iter();

        let first: TreeNode = match iter.next() {
            None => return Rc::new(RefCell::new(TreeNode::new(0))),
            Some(x) => { TreeNode::new(match x {
                Val::Int(v) => v as i32,
                _ => panic!("Invalid input")
            }) }
        };

        let first = Some(Rc::new(RefCell::new(first)));
        
        let mut current_deep = 1;
        let mut current_horizontal_index = 0;
        let mut current: Vec<Option<TreeNodeRef>> = vec![];
        let mut new_layer: Vec<Option<TreeNodeRef>> = vec![];
        current.push(first.clone());

        for x in iter {
            let node = match x {
                Val::Int(v) => Some(Rc::new(RefCell::new(TreeNode::new(v as i32)))),
                Val::None => None,
                _ => panic!("Invalid input")
            };
            new_layer.push(node.clone());

            let max_layer = 2_usize.pow(current_deep);

            while let None =  &current[current_horizontal_index / 2] {
                current_horizontal_index += 1
            }

            if let Some(father) = &mut current[current_horizontal_index / 2] {
                if current_horizontal_index % 2 == 0 {
                    father.borrow_mut().left = node;
                } else {
                    father.borrow_mut().right = node;
                }
            }

            current_horizontal_index += 1;

            if current_horizontal_index == max_layer {
                current_horizontal_index = 0;
                current_deep += 1;
                current = new_layer;
                new_layer = vec![];
            }
        }

        first.unwrap()
    }
} 