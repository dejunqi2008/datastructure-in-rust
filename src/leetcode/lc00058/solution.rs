#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res: &str = "";
        for r in s.split_whitespace() {
            res = r;
            // println!("s: {}", s);
        }
        return res.to_string().len() as i32;
    }

    pub fn test() {
        let str = "   fly me   to   the moon  ".to_string();
        for s in str.split_whitespace() {
            println!("s: {}", s);
        }
    }
}
