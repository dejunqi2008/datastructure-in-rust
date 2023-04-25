use std::{collections::BinaryHeap, cmp::Reverse};
pub struct Solution;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    
    pub fn new() -> Self {
        let mut max_heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new(); 
        Self {
            max_heap,
            min_heap
        }
    }

    fn add_num(&mut self, num: i32) {
        self.min_heap.push(Reverse(num));
        let min_top = self.min_heap.pop().unwrap().0;
        self.max_heap.push(min_top);
        if self.max_heap.len() > self.min_heap.len() {
            let max_top = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(max_top));
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.min_heap.len() == self.max_heap.len() {
            let a = self.max_heap.peek().unwrap();
            let b = self.min_heap.peek().unwrap().0;
            let res = ((b + a) as f64) / 2.0;
            return res;
        }
        return self.min_heap.peek().unwrap().0 as f64;
    }


}

impl Solution {
}
