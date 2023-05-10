#![allow(unused)]
use crate::leetcode::utils::list_node::ListNode;
pub struct Solution;

impl Solution {

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }

        if list2.is_none() {
            return list1;
        }

        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;
        let mut l1 = list1;
        let mut l2 = list2;

        while l1.is_some() && l2.is_some() {
            let v1 = l1.as_ref().unwrap().val;
            let v2 = l2.as_ref().unwrap().val;

            if v1 <= v2 {
                cur.next = l1.take();
                cur = cur.next.as_mut().unwrap();
                l1 = cur.next.take();
            } else {
                cur.next = l2.take();
                cur = cur.next.as_mut().unwrap();
                l2 = cur.next.take();
            }
        }

        if l1.is_some() {
            cur.next = l1.take();
        }

        if l2.is_some() {
            cur.next = l2.take();
        }

        return dummy.next;

    }

}
