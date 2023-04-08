use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::utils::tree_node::TreeNode;
pub struct Solution;

type RcTreeNode = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn level_order(root: Option<RcTreeNode>) -> Vec<Vec<i32>> {
       let mut queue: Vec<RcTreeNode> = vec![];
       let mut res: Vec<Vec<i32>> = vec![];

        if root.is_none() {
            return res;
        }

        queue.push(root.unwrap());

        while !queue.is_empty() {
            let len = queue.len();
            let mut arr: Vec<i32> = vec![];
            for i in 0..len {
                let head_node: RcTreeNode = queue.remove(0);
                let val = head_node.borrow().val;
                arr.push(val);
                if head_node.borrow().left.is_some() {
                    queue.push(head_node.borrow_mut().left.take().unwrap());
                }
                if head_node.borrow().right.is_some() {
                    queue.push(head_node.borrow_mut().right.take().unwrap());
                }
            }
            res.push(arr);
        }

        return res;
       
    }
}
