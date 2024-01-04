/*
Link: https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/
Problem: 2870. Minimum Number of Operations to Make Array Empty
*/

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        
        let mut m: HashMap<i32, i32> = HashMap::new();
        
        for n in nums {
            *m.entry(n).or_insert(0) += 1;
        }
        
        
        let mut ans = 0;
        
        for (val, freq) in m {
            if freq < 2 {
                return -1;
            }
            
            ans += (freq as f64 / 3 as f64).ceil() as i32;
        }
        
        return ans;
        
    }
}