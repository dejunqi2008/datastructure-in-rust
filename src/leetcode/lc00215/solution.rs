#![allow(unused)]
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut pq: BinaryHeap<Reverse<&i32>> = BinaryHeap::new();
        for v in nums.iter() {
            pq.push(Reverse(v));
            if pq.len() as i32 > k {
                pq.pop();
            }
        }

        return *pq.pop().unwrap().0;
    }

    pub fn test() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        Self::find_kth_largest(nums, 2);
    } 
}
