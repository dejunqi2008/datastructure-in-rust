#![allow(unused)]
use std::cmp::min;
pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut memo: Vec<i32> = vec![0; nums.len()];
        let res = Self::helper(&nums, 0, &mut memo);
        return res;
    }

    fn helper(nums: &Vec<i32>, idx: usize, memo: &mut Vec<i32>) -> i32 {
        if idx == nums.len() - 1 {
            return 0;
        }

        if memo[idx] > 0 {
            return memo[idx];
        }

        let mut cur = nums.len() + 1;

        let next_jum = nums[idx];

        for i in 1..(next_jum + 1) {
            let next_pos = idx + i as usize;
            if next_pos < nums.len() {
                let res = 1 + Self::helper(nums, next_pos, memo);
                // cur = min(cur, (res as usize));
                let v = res as usize;
                let small = min(&cur, &v);
                cur = *small;
            }
        }

        memo[idx] = cur as i32;

        return cur as i32;

    }
}
