use crate::leetcode::{utils::list_node::ListNode, lc00001::solution};


pub struct Solution {
	remove_nth_from_end: fn(Option<Box<ListNode>>) -> Option<Box<ListNode>>
}

impl Solution {

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {

		let mut dummy = ListNode::new(0);
		dummy.next = head;
		let mut dummy = Box::new(dummy);
		let mut  cur = dummy.clone();
		let mut prev = dummy.as_mut();

		for _ in 0..n {
			cur = cur.next.unwrap();
		}

		while cur.next.is_some() {
			cur = cur.next.unwrap();
			prev = prev.next.as_mut().unwrap();
		}

		let post = prev.next.as_mut().unwrap();
		prev.next = post.next.clone();

		return dummy.next;

    }
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

		// println!("{}", head.as_ref().unwrap().val);
		Self::remove_nth_from_end(head, 2);
	}
}
