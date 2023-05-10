#![allow(unused)]
use std::fmt::Display;

pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}

pub struct LinkedList<T> {
    size: usize,
    head: Option<Box<ListNode<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            size: 0,
            head: None
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    // prepand node to the head
    pub fn prepand(&mut self, val: T) {
        let node = Box::new(ListNode {
            val,
            next: self.head.take()
        });
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // 1. self.head : Option<Box<ListNode<T>>>
        // 2. self.head.take(): Option<Box<ListNode<T>>> *still an Option enum type, it is Some(val) of the enum
        // 3.  self.head.take().map(node -> node.val): Option<Box<ListNode<T>>> --> Option<Box<T>>

        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            return node.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // 1. sefl.head: Option<Box<ListNode<T>>>
        // 2. self.head.as_ref() : Option<&Box<ListNode<T>>>
        // 3. sefl.head.as_ref().map(node -> &node.val): Option<&node> -> Option<&node.val>
        self.head
            .as_ref()
            .map(|node| { &node.val })
    }

    // We can use this to change the value of its head
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.val
        })
    }
    
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        return self.0.pop();
    }
}



impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.head.is_none() {
            write!(f, "None\n")?;
        } else {
            let mut cur = self.head.as_ref();

            // while let Some(node) = cur {
            //     write!(f, "{} -> ", node.val)?;
            //     cur = node.next.as_ref();
            // }

            while cur.is_some() {
                let val = cur.map(|node| { &node.val }).unwrap();
                write!(f, "{} -> ", *val)?;
                cur = cur.take().unwrap().next.as_ref();
            }
            write!(f, "None\n")?;
        }

        return Ok(())
    }
}

pub fn test() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.prepand(1);
    list.prepand(2);
    list.prepand(3);
    list.prepand(4);
    println!("list: {}", list);
}