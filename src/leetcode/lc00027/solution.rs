pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx: usize = 0;
        let len = nums.len();

        for i in 0..len {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        return idx as i32;
    }

    pub fn test() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let p = Self::remove_element(&mut nums, 2);
        println!("{}", p);
    }
}
