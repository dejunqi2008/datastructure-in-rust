use crate::leetcode::utils::list_node::ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let mut dummy = ListNode::new(0);
    let mut p1 = l1.as_ref();
    let mut p2 = l2.as_ref();
    let mut cur = &mut dummy;
    let mut carry = 0;
    
    while p1.is_some() || p2.is_some() {
        let mut v1 = p1.map_or(0, |node| node.val);
        let mut v2 = p2.map_or(0, |node| node.val);
        
        let mut sum = v1 + v2 + carry;
        carry = sum / 10;        
        cur.next = Some(Box::new(ListNode::new(sum % 10)));
        cur = cur.next.as_mut().unwrap();
        
        p1 = p1.and_then(|node| node.next.as_ref());
        p2 = p2.and_then(|node| node.next.as_ref());
        
    }
    
    if carry > 0 {
        cur.next = Some(Box::new(ListNode::new(carry)));
    }
    
    return dummy.next; 
    
}


pub fn test() {}


/*

map_or() 的用法
refernce: https://learning-rust.github.io/docs/e6.combinators.html
*/