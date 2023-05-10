#![allow(unused)]
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

pub struct Solution;

#[derive(Debug, Eq, PartialEq)]
struct Item {
    ch: char,
    cnt: i32,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cnt.cmp(&other.cnt)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cnt.partial_cmp(&other.cnt)
    }
}

impl Solution {

    pub fn frequency_sort(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut arr = [0; 126];
        let len = s.len();
        for i in 0..len {
            let ch = s.chars().nth(i).unwrap();
            let idx = ch as u32;
            arr[idx as usize] += 1;
        }

        chars.sort_by(|a1, a2| {
            let i = *a1 as u32;
            let j = *a2 as u32;
            match arr[i as usize].cmp(&arr[j as usize]) {
                Ordering::Equal => a1.cmp(a2),
                _ => arr[j as usize].cmp(&arr[i as usize])
            }
        });
        println!("{:?}", chars);

        return chars.iter().collect();
    }

    pub fn frequency_sort_v2(s: String) -> String {
        
        let mut res: String = String::new();
        let mut map: HashMap<char, i32> = HashMap::new();
        let len = s.len();
        let mut pq: BinaryHeap<Item> = BinaryHeap::new();

        for i in 0..len {
            let ch = s.chars().nth(i).unwrap();
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }

        for (k, v) in map.into_iter() {
            pq.push(Item {
                ch: k,
                cnt: v
            });
        }

        while !pq.is_empty() {
            // println!("{:?}", pq.pop());
            let item = pq.pop().unwrap();
            let ch = item.ch;
            let cnt = item.cnt as usize;
            for _ in 0..cnt {
                res.push(ch);
            }
        }
        

        return res;
    }



    pub fn test() {
        let mut s = "tree".to_string();
        s = "loveleetcode".to_string();
        let res = Self::frequency_sort(s);
        println!("{:?}", res);

        // let chars = vec!['a', 'z', 'A', 'Z', '0', '9'];
        // for c in chars {
        //     println!("{}", c as u32);
        // }
    }
}

