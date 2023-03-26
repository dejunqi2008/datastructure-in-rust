use crate::leetcode::utils::list_node::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {

		// create a mutableHead variable
		let mut mutableHead = head;

		// create a mutable reference 'cur' (runner node) to the mutable head, so that we can modify it if condition is satifised during the traverse
		let mut cur = &mut mutableHead;

		loop {
			match cur {
				None => break,
				Some(node) if val == node.val => {
					// here we "drop" the content of current runner node and replace it with its next
					// since we are modifing the cur (runner node), that is why we have to define cur as mut
					*cur = node.next.take();
				},
				Some(node) => {
					cur = &mut node.next;
				}
			}
		}


		return mutableHead;
    }


	pub fn test() {
		let arr: [i32; 6] = [5, 4, 3, 2, 4, 1];
		let mut head = None;
		for i in 0..6 {
			let node: Box<ListNode> = Box::new(ListNode {
				val: arr[i],
				next: head.take()
			});
			head = Some(node);
		}

		Self::remove_elements(head, 4);

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