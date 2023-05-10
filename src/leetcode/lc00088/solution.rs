#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.split_off(m as usize);
        nums1.extend(nums2.to_vec());
        nums1.sort()
        // println!("{:?}", &nums1_0);
    }


    pub fn test() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Self::merge(&mut nums1, 3, &mut nums2, 3);
        println!("{:?}", &nums1);
    }
}