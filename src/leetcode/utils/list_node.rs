
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        return ListNode {
            next: None,
            val
        };
    }
}

pub fn print_list_helper(head: &Option<Box<ListNode>>) {
    let mut cur = head;
    loop {
        match cur {
            None => break,
            Some(node) => {
                print!("{} -> ", node.val);
                cur = &node.next
            }
        }
    }
    println!("None")
}

pub fn create_linkedlist(arr: &[i32]) -> Option<Box<ListNode>> {
    // let arr = [5, 4, 3, 2, 1];
    let mut head: Option<Box<ListNode>> = None;

    for i in 0..arr.len() {
        let node: Box<ListNode> = Box::new(ListNode {
            val: arr[i],
            next: head.take()
        });
        head = Some(node);
    }
    return head;
}