use crate::leetcode::utils::list_node::ListNode;

pub struct Solution;

impl Solution {
	pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

		let mut prev: Option<Box<ListNode>> = None;
		let mut cur = head;

		loop {
			match cur {
				None => break,
				Some(mut node) => {
					cur = node.next;
					node.next = prev;
					prev = Some(node);
				}
			}
		}

		return prev;
	}

	pub fn test() {}
}
