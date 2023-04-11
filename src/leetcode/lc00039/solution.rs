pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut can: Vec<i32> = vec![];
        Self::helper(&candidates, &mut res, &mut can, target, 0);
        return res;

    }

    fn helper(
        candidates: &Vec<i32>,
        mut res: &mut Vec<Vec<i32>>,
        mut can: &mut Vec<i32>,
        target: i32,
        idx: usize
    ) {
        if target < 0 {
            return;
        }

        if target == 0 {
            res.push(can.clone());
            return;
        }

        for i in idx..candidates.len() {
            // let val = candidates[i].clone();
            can.push(candidates[i]);
            Self::helper(candidates, res, can, target - candidates[i], i);
            can.pop();
        }
    }

    pub fn test() {
        let nums = vec![2, 3, 6, 7];
        let target = 7;
        let res = Self::combination_sum(nums, target);
        for v in res.iter() {
            println!("\n");
            for i in v.iter() {
                println!("{}", i);
            }
        }

    }
}
