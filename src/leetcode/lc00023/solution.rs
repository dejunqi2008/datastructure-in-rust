#![allow(unused)]
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use crate::leetcode::utils::list_node::ListNode;

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        return other.val.partial_cmp(&self.val);
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.val.cmp(&self.val);
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut queue: BinaryHeap<Box<ListNode>> = BinaryHeap::with_capacity(lists.len());

        for list in lists {
            match list {
                None => {},
                Some(l) => {
                    queue.push(l);
                }
            }
        }

        let mut dummy: Box<ListNode> = Box::new(ListNode::new(0));
        let mut cur: &mut Box<ListNode> = &mut dummy;

        while let Some(node) = queue.pop() {
            let mut new_node = Box::new(ListNode::new(node.val));
            cur.next = Some(new_node);
            cur = cur.next.as_mut().unwrap();

            if node.next.is_some() {
                queue.push(node.next.unwrap());
            }
        }

        return dummy.next;

    }
}


/*
当遇到一下编译错误时：
mismatched types
expected mutable reference `&mut Box<list_node::ListNode>`
                found enum `Option<Box<list_node::ListNode>>`

可以用一下方法
cur = cur.next // => Option<Box<T>>
            .as_mut()  // => Option<&mut Box<T>>
            .unwrap()  // => &mut Box<T>
 */