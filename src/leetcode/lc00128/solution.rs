use std::collections::HashSet;
use std::cmp::max;
pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_copy = nums.clone();
        let mut set: HashSet<_> = nums.into_iter().collect();
        let mut res = 0;
        for v in num_copy {
            let mut a = v - 0;
            let mut b = v - 1;
            let mut len = 0;
            while !set.is_empty() && set.contains(&a) {
                set.remove(&a);
                a += 1;
                len += 1;
            }
            while !set.is_empty() && set.contains(&b) {
                set.remove(&b);
                b -= 1;
                len += 1; 
            }
            let updated_res = max(res, len);
            res = updated_res;
        }
        res
    }

    pub fn test() {
        let nums = vec![100,4,200,1,3,2];
        let res = Self::longest_consecutive(nums);
        println!("{}", res);
    }
}