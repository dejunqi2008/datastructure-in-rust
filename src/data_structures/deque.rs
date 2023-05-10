#![allow(unused)]

pub struct Deque<T> {
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            data: Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    pub fn add_front(&mut self, val: T) {
        self.data.insert(0, val);
    }

    pub fn add_rear(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        return Option::Some(self.data.remove(0));
    }

    pub fn pop_rear(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        return Option::Some(self.data.remove(self.data.len() - 1));
    }

    pub fn peek_front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        return self.data.get(0);
    }

    pub fn peek_rear(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        return self.data.get(self.data.len() - 1);
    }

    pub fn size(&self) -> usize {
        return self.data.len();
    }

}


pub fn test() {

    let mut deque: Deque<i32> = Deque::new();

    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    let len = arr.len();

    for i in 0..len {
        if i < len / 2 {
            println!("{}", i);
            deque.add_front(arr[i])
        } else {
            deque.add_rear(arr[i]);
        }
    }

    println!("\n");
    while !deque.is_empty() {
        println!("{}", deque.pop_front().unwrap());
    }
}