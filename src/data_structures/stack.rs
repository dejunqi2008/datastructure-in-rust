#![allow(unused)]
pub struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new()
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }

        self.top -= 1;
        return self.data.pop();
    }

    pub fn peek(&self) -> Option<&T> { // for peek, data can not be moved, only return a address (reference) of the top 
        if self.top == 0 {
            return None;
        }

        return self.data.get(self.top - 1);
    }

    pub fn is_empty(&self) -> bool {
        return self.top == 0;
    }

    pub fn size(&self) -> usize {
        return self.top;
    }

}