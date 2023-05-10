#![allow(unused)]
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for v in nums {
            let count = map.entry(v).or_insert(0);
            *count += 1;
            if *count == 3 {
                map.remove_entry(&v);
            }
        }

        let mut res = 0;
        for (v, c) in map {
            res = v;
        }

        return res;
    }
}
