use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut parent = vec![0; num_courses as usize];
        let mut queue: Vec<i32> = vec![];
        let default_list = vec![];
        let mut cnt = 0;

        for pre in prerequisites {
            let a = pre[0];
            let b = pre[1];
            map.entry(a).or_insert(Vec::new()).push(b);
            parent[b as usize] += 1;
        }

        for i in 0..(parent.len()) {
            if parent[i] == 0 {
                queue.push(i as i32);
            }
        }

        while !queue.is_empty() {
            let top = queue.remove(0);
            cnt += 1;
            let list = map.get(&top).unwrap_or(&default_list);
            for v in list {
                parent[*v as usize] -= 1;
                if parent[*v as usize] == 0 {
                    queue.push(*v)
                }
            }
        }

        return cnt == num_courses;
    }
}
