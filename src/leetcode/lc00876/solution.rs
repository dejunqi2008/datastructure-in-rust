#![allow(unused)]

use crate::leetcode::utils::list_node::{
    ListNode,
    print_list_helper,
    create_linkedlist,
};

pub struct Solution;

impl Solution {

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy = Box::new(ListNode::new(0));
        
        dummy.next = head;
        
        let mut fast = dummy.clone();
        
        let mut slow = &mut dummy;

        // loop {
        //     match fast {
        //         _ if fast.next.is_none() || fast.next.as_ref().unwrap().next.is_none() => {
        //             break;
        //         },
        //         _ => {
        //             fast = fast.next.as_mut().unwrap().next.take().unwrap();
        //             slow = slow.next.as_mut().unwrap();
        //         }
        //     }
        // }

        while fast.next.is_some() && fast.next.as_ref().unwrap().next.is_some() {
            fast = fast.next.as_mut().unwrap().next.take().unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        
        println!("{}, {}", fast.val, slow.val);

        return slow.next.take();

    }


    pub fn test() {
        let arr1 = [1, 2, 3, 4, 5];
        let arr2 = [1, 2, 3, 4, 5, 6];
        let mut head = create_linkedlist(&arr1);
        print_list_helper(&head);
        let mid1 = Self::middle_node(head);
        println!("{}", mid1.as_ref().unwrap().val);

        println!("\n------\n");

        let mut head2 = create_linkedlist(&arr2);
        print_list_helper(&head2);
        let mid2 = Self::middle_node(head2);
        println!("{}", mid2.as_ref().unwrap().val);
    }

}
