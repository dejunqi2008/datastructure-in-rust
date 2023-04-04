
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut sign = 1;
        if x < 0 {
            sign = -1;
        }

        let mut res: i32 = 0;

        let mut x = x.abs();

        while x > 0 {
            let lastDigit = x % 10;
            let a: Option<i32> = res.checked_mul(10);
            if a == None {
                return 0;
            }

            let b: Option<i32> = a.unwrap().checked_add(lastDigit);
            if b == None {
                return 0;
            }
            res = b.unwrap();
            x = x/ 10;
        }

        return res * sign;

    }
}