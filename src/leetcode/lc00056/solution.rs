use std::cmp::{min, max};

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            let v0 = a[0];
            let u0 = b[0];
            return v0.cmp(&u0);
        });

        let mut res: Vec<Vec<i32>> = vec![];
        let len = intervals.len();
        let mut prev = &intervals[0];
        let mut tmp: Vec<i32> = vec![];

        for i in 1..len {
            let cur = &intervals[i];
            let a = cur[0];
            let b = cur[1];
            let p0 = prev[0];
            let p1 = prev[1];
            if p1 < a {
                res.push(vec![p0, p1]);
                prev = cur;
            } else {
                let small = min(a, p0);
                let big = max(b, p1);
                tmp = vec![small, big];
                prev = &tmp;
            }
        }
        let last = vec![prev[0], prev[1]];
        res.push(last);

        res
    }
}
