use crate::leetcode::utils::tree_node::TreeNode;



use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

type RcNode = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn insert_into_bst(root: Option<RcNode>, val: i32) -> Option<RcNode> {
        return Self::helper(root, val);
    }

    pub fn helper(node: Option<RcNode>, val: i32) -> Option<RcNode> {
        match node {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(n) => {
                if n.borrow().val > val {
                    let node: Option<RcNode> = Self::helper(n.borrow().left.clone(), val);
                    n.borrow_mut().left = node;
                } else {
                    let node: Option<RcNode> = Self::helper(n.borrow().right.clone(), val);
                    n.borrow_mut().right = node;
                }
                return Some(n);
            }
        }
    }
}
