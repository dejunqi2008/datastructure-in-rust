use std::collections::HashMap;
use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let len = s.len();
        let mut res = 0;

        let mut map: HashMap<char, i32> = HashMap::new();

        let bytes = s.as_bytes();

        while r < len {
            let c = bytes[r] as char;
            if map.contains_key(&c) && &l <= map.get(&c).take().unwrap() {
                res = max(res, r as i32 - l);
                l = map.get(&c).take().unwrap() + 1;
            }
            map.insert(c, r as i32);
            r += 1;
        }

        return max(res, r as i32 - l);
    }
}
