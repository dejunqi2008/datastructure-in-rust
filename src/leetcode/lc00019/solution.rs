use crate::leetcode::utils::list_node::{
	ListNode,
	print_list_helper,
	create_linkedlist
};

pub struct Solution;

impl Solution {

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        let mut cur = dummy.clone();
        let mut prev = dummy.as_mut();

        for _ in 0..n {
            cur = cur.next.unwrap();
        }

        loop {
            match &cur.next {
                None => break,
                Some(_) => {
                    cur = cur.next.unwrap();
                    prev = prev.next.as_mut().unwrap();
                }
            }
        }

        prev.next = prev.next.as_mut().unwrap().next.take();

        return dummy.next;

    }


	pub fn test() {
		Self::case1();
		Self::case2();
		Self::case3();
	}

    fn case1() {
        let arr = [5, 4, 3, 2, 1];
        Self::share(&arr, 2);
    }

    fn case2() {
        let arr = [1];
        Self::share(&arr, 1)
    }

    fn case3() {
        let arr = [2, 1];
        Self::share(&arr, 1);
    }

    fn share(arr: &[i32], nth: i32) {
        let head: Option<Box<ListNode>> = create_linkedlist(&arr);
        print_list_helper(&head);
        let newhead = Self::remove_nth_from_end(head, nth);
        print_list_helper(&newhead);
        println!("\n-------")
    }
}
