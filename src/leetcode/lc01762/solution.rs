#![allow(unused)]

pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
    
    let mut list: Vec<i32> = Vec::new();
    let size = heights.len();
    let len: i32 = size as i32;
    
    list.push(len - 1);
    println!("len = {}", len);
    let mut max = heights[size - 1];
    
    for i in (0..len-1).rev() {
        println!("i = {}", i);
        let h = heights[i as usize];
        if h > max {
            max = h;
            list.push(i);
            // list.insert(0, i)
        }
     }
    

    list.sort();
    
    return list;
}


pub fn test() {
    let heights: Vec<i32> = vec![4, 2, 3, 1];
    let list: Vec<i32> = find_buildings(heights);
    println!("{:?}", list);
}