#![allow(unused)]
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Item {
    val: i32,
    cnt: u32,
}

impl PartialOrd<Item> for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        self.cnt.partial_cmp(&other.cnt)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cnt.cmp(&other.cnt)
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];

        let mut map: HashMap<i32, u32> = HashMap::new();
        

        for n in nums {
            let count = map.entry(n).or_insert(0);
            *count += 1;
        }

        let mut queue: BinaryHeap<Item> = BinaryHeap::new();
        for v in map.into_iter() {
            queue.push(Item {
                val: v.0,
                cnt: v.1
            })
        }
        let mut k = k;
        while !queue.is_empty() && k > 0 {
            res.push(queue.pop().unwrap().val);
            // println!("{:?}", queue.pop());
            k -= 1;
        }
        return res;
    }

    pub fn test() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        
        let res = Self::top_k_frequent(nums, k);
        println!("{:?}", res);

    }

}
