pub struct Solution;

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let len1 = word.len();
        let len2 = abbr.len();

        let word_chars: Vec<char> = word.chars().collect();
        let abbr_chars:Vec<char> = abbr.chars().collect();
        let mut cnt: u32 = 0;


        while i < len1 && j < len2 {   

            if abbr_chars[j].is_numeric() && abbr_chars[j].to_digit(10).unwrap_or(0) > 0 {
                while j < len2 && abbr_chars[j].is_numeric() {
                    let a = cnt.checked_mul(10).unwrap_or(0);
                    let num = a.checked_add(abbr_chars[j].to_digit(10).unwrap_or(0)).unwrap_or(0);
                    cnt = num;
                    j += 1;
                }
            }

            if j == len2 {
                break;
            }
            
            i += cnt as usize;            
            if i >= len1 || abbr_chars[j] != word_chars[i] {
                break;
            }
            
            cnt = 0;
            i += 1;
            j += 1;
        }

        i += cnt as usize;

        return i == len1 && j == len2;
    }

    pub fn test() {
        Self::case1();
        Self::case2();
        Self::case3();
        Self::case4();
    }

    fn case1() {
        let word = "internationalization".to_string();
        let abbr = "i12iz4n".to_string();
        println!("res: {}", Self::valid_word_abbreviation(word, abbr)); // true
    }

    fn case2() {
        let word = "apple".to_string();
        let abbr = "a2e".to_string();
        println!("res: {}", Self::valid_word_abbreviation(word, abbr)); // false
    }
    fn case3() {
        let word = "internationalization".to_string();
        let abbr = "i5a11o1".to_string();
        println!("res: {}", Self::valid_word_abbreviation(word, abbr)); // true
    }

    fn case4() {
        let word = "a".to_string();
        let abbr = "01".to_string();
        println!("res: {}", Self::valid_word_abbreviation(word, abbr)); // false
    }
}
