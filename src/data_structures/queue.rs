#![allow(unused)]
// queue.rs

pub struct Queue<T> {
    data: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            data: Vec::new()
        }
    }

    pub fn enqueue(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        return Option::Some(self.data.remove(0));
    }

    // peek the head of this queue
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        return self.data.get(0);
    } 

    pub fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    pub fn size(&self) -> usize {
        return self.data.len();
    } 
}

