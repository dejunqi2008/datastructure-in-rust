#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
		let mut idx: usize = 0;
		let mut i: usize = 0;
		let len = nums.len();

		for i in 0..len {
			if nums[i] != nums[idx] {
				idx += 1;
				nums[idx] = nums[i];
			}
		}

		let res = (idx as i32) + 1;
		return res;
        
    }

	pub fn test() {}
}