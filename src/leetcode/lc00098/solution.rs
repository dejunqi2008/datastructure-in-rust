#![allow(unused)]
use crate::leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

type RcTreeNode = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_valid_bst(root: Option<RcTreeNode>) -> bool {
        let mut res: (Option<i32>, bool) = (None, true);
        Self::helper(&root, &mut res);
        return res.1;
    }

    fn helper(node: &Option<RcTreeNode>, res: &mut (Option<i32>, bool)) {
        if node.is_none() {
            return;
        }

        let mut curval = node.as_ref().unwrap().borrow().val;
        let left: &Option<RcTreeNode> = &node.as_ref().unwrap().borrow().left;
        let right: &Option<RcTreeNode> = &node.as_ref().unwrap().borrow().right;

        Self::helper(left, res);
        match res.0 {
            Some(v) if curval <= v => {
                res.1 = false;
            },
            _ => {},
        }
        res.0 = Some(curval);
        Self::helper(right, res);
    }
}
