#![allow(unused)]
use std::collections::HashMap;

pub struct Solution {
    two_sum: fn(Vec<i32>, i32) -> Vec<i32>,
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut map: HashMap<i32, i32> = HashMap::new();
        let len = nums.len();
        
        for i in 0..len {
            let a = *nums.get(i).unwrap();
            let b = target - a;
            if map.contains_key(&b) {
                let idx = *map.get(&b).unwrap() as i32;
                return vec![idx, i as i32];
            }
            map.insert(a, i as i32);
        }

        return vec![];

    }

    pub fn test() {
        Self::case1();
        Self::case2();
    }

    fn case1() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let res: Vec<i32> = Self::two_sum(nums, target);
        for i in &res {
            print!("{} ", i);
        }
        println!("\n");
    }

    fn case2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let res: Vec<i32> = Self::two_sum(nums, target);
        for i in &res {
            print!("{} ", i);
        }
        println!("\n");
    }
}