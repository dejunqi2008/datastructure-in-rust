// PriorityQueue demo

use std::collections::BinaryHeap;
use std::cmp::Ordering;

// demo

struct Person {
    age: u32,
    name: String,
}

impl Eq for Person {}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        return self.age.cmp(&other.age);
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        return self.age == other.age;
    }
}

pub fn test() {
    let a = Person {
        age: 32, name: "Dejun".to_string()
    };

    let b = Person {
        age: 28, name: "John".to_string()
    };

    let c = Person {
        age: 42, name: "Amy".to_string()
    };

    let mut queue: BinaryHeap<Person> = BinaryHeap::new();

    queue.push(a);
    queue.push(b);
    queue.push(c);

    while !queue.is_empty() {
        let p: Person = queue.pop().unwrap();
        println!("Next person is (name: {}, age: {})", p.name, p.age);
    }
}