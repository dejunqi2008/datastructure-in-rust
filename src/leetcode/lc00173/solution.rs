
use std::cell::RefCell;
use std::rc::Rc;
use crate::leetcode::utils::tree_node::TreeNode;


type RcTreeNode = Rc<RefCell<TreeNode>>;

struct  BSTIterator {
    stack: Vec<RcTreeNode>,
}

impl BSTIterator {

    fn new(root: Option<RcTreeNode>) -> Self {
        let mut stack: Vec<RcTreeNode> = vec![];
        Self::maintain_left(&mut stack, root);
        Self {
            stack
        }
    }
    
    fn next(&mut self) -> i32 {
        let top = self.stack.pop().unwrap();
        let right = top.borrow_mut().right.take();
        if right.is_some() {
            Self::maintain_left(&mut self.stack, right)
        }
        return top.borrow_mut().val;
    }
    
    fn has_next(&self) -> bool {
        return !self.stack.is_empty();
    }
    
    fn maintain_left(stack: &mut Vec<RcTreeNode>, mut node: Option<RcTreeNode>) {
        while let Some(cur) = node {
            let left = cur.borrow_mut().left.take();
            stack.push(cur);
            node = left;
        }
    }
    
}


pub struct Solution;

impl Solution {
    pub fn test() {
        let bst_iterator = BSTIterator::new(None);
    }
}
