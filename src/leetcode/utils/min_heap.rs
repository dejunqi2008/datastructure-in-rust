use std::borrow::Borrow;
use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, Reverse};

pub struct MinHeap<T> {
    data: BinaryHeap<Reverse<T>>,
}

impl<T> MinHeap<T> where T: Ord + PartialOrd {
    pub fn new() -> Self {
        let mut data = BinaryHeap::new();
        Self {
            data
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(Reverse(val));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let res = self.data.pop().unwrap().0;
        return Some(res);
    }

    pub fn is_empty(&self) -> bool {
        return self.data.is_empty();
    } 

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let res = &(self.data.peek().unwrap().0);
        return Some(res);
    }

}


pub struct Solution;

impl Solution {
    pub fn test() {
        let mut min_heap: MinHeap<i32> = MinHeap::new();
        let nums = vec![3, 2, 0, 8, 9, 5, 10];
        for v in nums {
            min_heap.push(v);
        }

        while let Some(val) = min_heap.pop() {
            println!("val = {}", val);
            println!("peek: {:?}", min_heap.peek());
            
        }
    }
}