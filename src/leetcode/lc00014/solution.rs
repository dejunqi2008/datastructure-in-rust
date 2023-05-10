#![allow(unused)]
pub struct Solution;


impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let len = strs.get(0).unwrap().len();
        let mut res = String::new();

        for i in 0..len {
            let mut temp: Option<char> = None;
            for s in strs.iter() {
                if i >= s.len() {
                    return res;
                }

                let c: char = s.chars().nth(i).unwrap();
                match temp {
                    None => { temp = Some(c) },
                    Some(ch) => {
                        if ch != c {
                            return res;
                        }
                    }
                }
            }

            if temp != None {
                res.push(temp.unwrap());
            }
        }

        return res;
    }

    pub fn test() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ];
        // let strs = vec![
        //     "dog".to_string(),
        //     "racecar".to_string(),
        //     "car".to_string()
        // ];

        let res: String = Self::longest_common_prefix(strs);
        println!("prefix: {}", res);
    }
}
