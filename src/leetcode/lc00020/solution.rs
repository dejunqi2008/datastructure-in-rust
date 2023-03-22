use crate::data_structures::stack::Stack;
use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack: Stack<char> = Stack::new();
    let mut map: HashMap<char, char> = HashMap::new();

    map.insert(')', '(');
    map.insert(']', '[');
    map.insert('}', '{');

    for c in s.chars() {
        // println!("characyer: {}", c);
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else {
            if stack.is_empty() || stack.peek().unwrap() != map.get(&c).unwrap() {
                return false;
            }
            stack.pop();
        }
    }
    return stack.is_empty();
}

pub fn test() {
    let s1 = String::from("()[]{}");
    let s2 = String::from("(]");
    println!("{}, {}", is_valid(s1), is_valid(s2));
}