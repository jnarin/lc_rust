/*
Link: https://leetcode.com/problems/insert-delete-getrandom-o1/
Problem: 380. Insert Delete GetRandom O(1)
*/

use std::collections::HashMap;

struct RandomizedSet {
    m: HashMap<i32, usize>,
    v: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        return Self {
            m: HashMap::new(),
            v: Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        let mut ans = false;
     
        if self.m.contains_key(&val) == false {
            self.m.insert(val, self.v.len());
            self.v.push(val);
            
            ans  = true;
        }
        
        return ans;
    }
    
    fn remove(&mut self, val: i32) -> bool {
        let mut ans = false;
        
        if self.m.contains_key(&val) == true {
            let idx = self.m[&val];
            let len = self.v.len();
            
            if self.v[len - 1] == val {
                self.v.pop();
                self.m.remove(&val);
            } else {
                self.m.remove(&val);
                self.v[idx] = self.v[len - 1];
                self.v.pop();
                self.m.insert(self.v[idx], idx);
            }
            
            ans = true;
        }
        
        return ans;
    }
    
    fn get_random(&self) -> i32 {
        return self.v[rand::random::<usize>() % self.v.len()];
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */