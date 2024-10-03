#![allow(dead_code)]

use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>, // FIFO
}

// You must use only standard operations of a queue, which means that only 
// push to back, peek/pop from front, size and is empty operations are valid
impl MyStack {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let mut temp = VecDeque::new();
        for _ in 0..self.queue.len()-1 {
            temp.push_back(self.queue.pop_front().unwrap());
        }
        let result = self.queue.pop_front().unwrap();
        self.queue = temp;
        result
    }

    fn top(&mut self) -> i32 {
        let mut temp = VecDeque::new();
        let mut result: i32 = -1;
        while let Some(val) = self.queue.pop_front() {
            temp.push_back(val);
            result = val;
        }
        self.queue = temp;
        result
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

/*
Explanation
    MyStack myStack = new MyStack();
    myStack.push(1);
    myStack.push(2);
    myStack.top(); // return 2
    myStack.pop(); // return 2
    myStack.empty(); // return False
*/
 
 fn main() {
}
