use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::utils::tree_node::TreeNode;

pub struct Solution;


type RcNode = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn postorder_traversal(root: Option<RcNode>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut stack: Vec<RcNode> = vec![];

        if root.is_none() {
            return res;
        }

        stack.push(root.unwrap());

        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let mut top_ref = top.borrow_mut();
            res.insert(0, top_ref.val);

            if let Some(node) = top_ref.left.take() {
                stack.push(node);
            }

            if let Some(node) = top_ref.right.take() {
                stack.push(node);
            }

        }
        
        return res;
    
   }
}
