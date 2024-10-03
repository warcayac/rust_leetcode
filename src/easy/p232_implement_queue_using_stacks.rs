#![allow(dead_code)]

use std::collections::VecDeque;

struct MyQueue {
    stack: VecDeque<i32>, // LIFO
}

// You must use only standard operations of a stack, which means only 
// push to top, peek/pop from top, size, and is empty operations are valid.
impl MyQueue {
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push_front(x);
    }
    
    fn pop(&mut self) -> i32 {
        let mut temp = VecDeque::new();
        for _ in 0..self.stack.len()-1 {
            temp.push_front(self.stack.pop_front().unwrap());
        }
        let result = self.stack.pop_front().unwrap();
        while let Some(val) = temp.pop_front() {
            self.stack.push_front(val);
        }
        result
    }
    
    fn peek(&mut self) -> i32 {
        let mut temp = VecDeque::new();
        let mut result: i32 = -1;
        while let Some(val) = self.stack.pop_front() {
            temp.push_front(val);
            result = val;
        }
        while let Some(val) = temp.pop_front() {
            self.stack.push_front(val);
        }
        result
    }
    
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

 /*
 Explanation
    MyQueue myQueue = new MyQueue();
    myQueue.push(1); // queue is: [1]
    myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
    myQueue.peek(); // return 1
    myQueue.pop(); // return 1, queue is [2]
    myQueue.empty(); // return false
*/

fn main() {
}
