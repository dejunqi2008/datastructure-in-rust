

use std::collections::{HashMap};
// use std::ops::Range;


pub struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {

        if prerequisites.len() == 0 {
            return (0..num_courses).collect();
        }
        let mut res: Vec<i32> = vec![];
        let default_return: Vec<i32> = Vec::new();

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut parents = vec![0; num_courses as usize];
        let mut queue: Vec<i32> = vec![];

        for edge in prerequisites {
            let a = edge[0];
            let b = edge[1];
            graph.entry(b).or_insert(Vec::new()).push(a);
            parents[a as usize] += 1;
        }

        for i in 0..parents.len() {
            if parents[i] == 0 {
                queue.push(i as i32);
            }
        }

        while !queue.is_empty() {
            let e = queue.remove(0);
            // println!("{}", &e);
            res.push(e);
            let list: &Vec<i32> = graph.get(&e).unwrap_or(&default_return);
 
            for v in list.into_iter() {
                parents[*v as usize] -= 1; 
                if parents[*v as usize] == 0 {
                    queue.push(*v);
                }
            }
        }

        if res.len() as i32 == num_courses {
            return res;
        }
        return default_return;
    }

    pub fn test() {
        let mut prereq = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        prereq = vec![];
        prereq = vec![vec![0, 1]];
        let res = Self::find_order(2, prereq);
        println!("{:?}", res);
    }
}
