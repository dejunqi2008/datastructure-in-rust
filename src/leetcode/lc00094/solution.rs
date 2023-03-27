use crate::leetcode::utils::tree_node::TreeNode;


use std::rc::Rc;
use std::cell::{RefCell, RefMut};

pub struct Solution;

impl Solution {

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

        let mut res: Vec<i32> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut cur: Option<Rc<RefCell<TreeNode>>> = root;
        
        while cur.is_some() || !stack.is_empty() {
            loop {
                match cur {
                    None => break,
                    Some(node) => {
                        let left: Option<Rc<RefCell<TreeNode>>> = node.borrow_mut().left.take();
                        stack.push(node);
                        cur = left;
                    }
                }
            }

            let top_node: Rc<RefCell<TreeNode>> = stack
                                                    .pop()
                                                    .unwrap();
            let mut top_node_ref: RefMut<TreeNode> = top_node.borrow_mut();
            let val: i32 = top_node_ref.val;
            res.push(val);
            if top_node_ref.right.is_some() {
                cur = top_node_ref.right.take();
            }
        }

        return res;

    }


	pub fn test() {}


}
