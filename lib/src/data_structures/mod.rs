mod tree_node;
mod list_node;

pub use tree_node::{TreeNode, TreeNodeRef};
pub use list_node::ListNode as ListNodeGeneric;

// Type alias para compatibilidade com código existente
pub type ListNode = ListNodeGeneric<i32>; 