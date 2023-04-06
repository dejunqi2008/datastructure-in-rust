pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        if x < 0 {
            return false;
        }

        let mut rev: i32 = 0;
        let mut mut_x = x.clone();

        while mut_x > 0 {
            // rev = rev * 10 + x % 10;
            let a: i32 = rev.checked_mul(10).unwrap_or(0);
            let b: i32 = a.checked_add(mut_x % 10).unwrap();
            rev = b;
            
            mut_x /= 10;

        }

        return rev == x;
    }

    pub fn test() {
        let x = 121;
        let res = Self::is_palindrome(x);
        println!("{}", res);

    }
}
