use crate::leetcode::utils::list_node::ListNode;

pub struct Solution {
    reverse_list: fn(Option<Box<ListNode>>) -> Option<Box<ListNode>>
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut prev: Option<Box<ListNode>> = None;
        let mut cur: &Option<Box<ListNode>> = &head;

        while cur.is_some() {
            let mut node: ListNode = ListNode::new(cur.as_ref()?.val);
            node.next = prev;
            prev = Some(Box::new(node));
            cur = &(
                    cur    // &Option<Box<ListNode>
                        .as_ref()? // as_ref() turns &Option<Box<ListNode> to Option<&Box<ListNode>>, then '?' takes take values inside the Option and unwrap it to &Box<ListNode>> 
                        .next
                    );
        }
        return prev;
    }
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

    // println!("{:?}", head.as_ref().unwrap().val);

    let newhead = Solution::reverse_list(head);
}