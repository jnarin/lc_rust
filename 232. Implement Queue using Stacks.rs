/*
Link: https://leetcode.com/problems/implement-queue-using-stacks/
Problem: 232. Implement Queue using Stacks
*/

struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        return Self {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.s1.push(x);        
    }
    
    fn pop(&mut self) -> i32 {
        let ans = Self::peek(self);
        self.s2.pop();
        return ans;
    }
    
    fn peek(&mut self) -> i32 {
        if self.s2.len() == 0 {
            while self.s1.len() > 0 {
                self.s2.push(self.s1[self.s1.len() - 1]);
                self.s1.pop();
            }
        }
        
        return self.s2[self.s2.len() - 1];
    }
    
    fn empty(&self) -> bool {
        return self.s1.len() == 0 && self.s2.len() == 0;        
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