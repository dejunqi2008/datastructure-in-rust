use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut new_interval = new_interval;
        for interval in intervals {
            let a = interval[0];
            let b = interval[1];

            let c = new_interval[0];
            let d = new_interval[1];

            if b < c {
                res.push(interval);
            } else if (a > d) {
                res.push(vec![c, d]);
                new_interval = interval;
            } else {
                let small = min(a, c);
                let big = max(b, d);
                new_interval = vec![small, big];
            }
        }

        res.push(new_interval);

        return res;
    }
}
