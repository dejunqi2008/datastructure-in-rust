// standard library of Rust

use std::collections::HashMap;

fn vec_demo() {
    // 修改一个向量内部元素值的方法
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

//  
    for i in v.iter_mut() {
        println!("{}", *i);
        *i = *i * 2;
    }

    println!("\n");
    for i in v.iter() {
        println!("{}", i);
    }

}


fn hasmap_demo() {
    // unordered map : HashMap
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("A", 100);
    map.insert("B", 98);

    println!("{}", map.get(&"A").unwrap());

    match map.get(&"B") {
        Some(val) => println!("Value of B is {}", val),
        None => println!("B is not found in the map")
    }

    match map.get(&"C") {
        Some(val) => println!("Value of C is {}", val),
        None => println!("C is not found in the map")
    }

    // loop through a map
    for (key, value) in map.iter() {
        println!("[{} : {}]", key, value);
    }
}

pub fn test() {
    // vec_demo();
    hasmap_demo();
}