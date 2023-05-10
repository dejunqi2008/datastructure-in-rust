#![allow(unused)]

use std::collections::HashMap;
use std::default;
use std::rc::Rc;
use std::cell::RefCell;
use crate::leetcode::utils::tree_node::TreeNode;
pub struct Solution;

type RcTreeNode = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn vertical_order(root: Option<RcTreeNode>) -> Vec<Vec<i32>> {

        let mut res: Vec<Vec<i32>> = vec![];
        let default: Vec<i32> = vec![];

        if root.is_none() {
            return res;
        }
        let mut queue: Vec<RcTreeNode> = vec![];
        let mut index_queue: Vec<i32> = vec![];

        queue.push(root.unwrap());
        index_queue.push(0);
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut min_idx = 0;
        let mut max_idx = 0;

        while !queue.is_empty() {
            let node: RcTreeNode = queue.remove(0);
            let idx = index_queue.remove(0);
            map.entry(idx).or_insert(Vec::new()).push(node.as_ref().borrow().val);
            let left: Option<RcTreeNode> = node.as_ref().borrow_mut().left.take();
            let right: Option<RcTreeNode> = node.as_ref().borrow_mut().right.take();
            if left.is_some() {
                queue.push(left.unwrap());
                let l = idx - 1;
                min_idx = std::cmp::min(l, min_idx);
                index_queue.push(l);
            }
            if right.is_some() {
                queue.push(right.unwrap());
                let r = idx + 1;
                max_idx = std::cmp::max(r, max_idx);
                index_queue.push(r);
            }
        }

        for idx in min_idx..(max_idx + 1) {
            let list: Vec<i32> = map.get(&idx).unwrap_or(&default).clone();
            res.push(list);
        }

        return res;
    }
}
