// #[derive(Debug)]
use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }

    // see reference 1:
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
    
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    
     
}


mod test {
    
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}


/*
Reference 1:

Ah yes, Indy suggests the mem::replace maneuver. This incredibly useful function lets us steal a value out of a borrow by replacing it with another value. Let's just pull in std::mem at the top of the file, so that mem is in local scope:

use std::mem;

pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
        elem: elem,
        next: mem::replace(&mut self.head, Link::Empty),
    });

    self.head = Link::More(new_node);
}

Here we replace self.head temporarily with Link::Empty before replacing it with the new head of the list. I'm not gonna lie: this is a pretty unfortunate thing to have to do. Sadly, we must (for now).

But hey, that's push all done! Probably. We should probably test it, honestly. Right now the easiest way to do that is probably to write pop, and make sure that it produces the right results.

*/