#![allow(unused)]

use crate::leetcode::utils::list_node::{
	ListNode,
	print_list_helper,
	create_linkedlist
};

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {

        let mut dummy: Box<ListNode> = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur: Box<ListNode> = dummy.clone();
        let mut prev: &mut ListNode = dummy.as_mut();

        if cur.next.is_some() {
            cur = cur.next.unwrap();
        }

        loop {
            match cur.next {
                None => break,
                Some(_) if cur.val == val => {
                    prev.next = prev.next.as_mut().unwrap().next.take();
                    cur = cur.next.unwrap();
                },
                Some(_) => {
                    prev = prev.next.as_mut().unwrap();
                    cur = cur.next.unwrap();
                }
            }
        }

        if cur.val == val {
            prev.next = prev.next.as_mut().unwrap().next.take();
        }

        return dummy.next;

    }


	pub fn test() {
        Self::case1();
        Self::case2();
		Self::case3();
	}

	fn case1() {
		let arr: [i32; 6] = [5, 4, 3, 2, 4, 1];
		let head: Option<Box<ListNode>> = create_linkedlist(&arr);
		let new_head: Option<Box<ListNode>> = Self::remove_elements(head, 4);
		print_list_helper(&new_head);
	}

	fn case2() {
		let arr: [i32; 1] = [ 1];
		let head: Option<Box<ListNode>> = create_linkedlist(&arr);
		let new_head: Option<Box<ListNode>> = Self::remove_elements(head, 1);
		print_list_helper(&new_head);
	}

	fn case3() {
		let arr: [i32; 0] = [];
		let head: Option<Box<ListNode>> = create_linkedlist(&arr);
		let new_head: Option<Box<ListNode>> = Self::remove_elements(head, 1);
		print_list_helper(&new_head);
	}


	
}

/*
pub fn test() {
    
    let arr = [5, 4, 3, 2, 1];
    let mut head: Option<Box<ListNode>> = None;

    for i in 0..arr.len() {
        let node: Box<ListNode> = Box::new(ListNode {
            val: arr[i],
            next: head.take()
        });
        head = Some(node);
    }

    // println!("{:?}", head.as_ref().unwrap().val);

    let newhead = Solution::reverse_list(head);
}
*/