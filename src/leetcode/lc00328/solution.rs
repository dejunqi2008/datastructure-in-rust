use crate::leetcode::utils::list_node::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;


        let mut odd = Box::from(ListNode::new(0));
        let mut even = Box::from(ListNode::new(0));
        
        let mut cnt = 1;

        let mut p1 = &mut odd;
        let mut p2 = &mut even;

        while head.is_some() {
            if cnt % 2 != 0 {
                p1.next = head;
                p1 = p1.next.as_mut().unwrap();
                head = p1.next.take();
            } else {
                p2.next = head;
                p2 = p2.next.as_mut().unwrap();
                head = p2.next.take();
            }
            cnt += 1;
        }
        p1.next = even.next;
        return odd.next;

    }
}
