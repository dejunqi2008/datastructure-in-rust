pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        nums.sort();

        let len = nums.len();
        let k = len - 2;
        for i in 0..k {
            let mut l = i + 1;
            let mut r = len - 1;
            if  i > 0 &&  nums[i] == nums[i - 1] {
                continue;
            }
            while l < r {
                let a = nums[i];
                let b = nums[l];
                let c = nums[r];
                let sum = a + b + c;
                if sum == 0 {
                    println!("{}, {}, {}", &a, &b, &c);
                    let v = vec![a, b, c];
                    res.push(v);
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    r -= 1;
                    l += 1;
                } else if sum > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }

        return res;
    }


    pub fn test() {
        let nums = vec![-1,0,1,2,-1,-4];
        Self::three_sum(nums);

    }
}
