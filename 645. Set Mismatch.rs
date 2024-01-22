/*
Link: https://leetcode.com/problems/set-mismatch/submissions/
Problem: 645. Set Mismatch
*/

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut s = HashSet::new();
        let mut ans = Vec::new();
        
        for n in &nums {
            if s.contains(&n) == true {
                ans.push(*n);
            } else {
                s.insert(n);
            }
        }
        
        for i in 1..=nums.len() {
            if s.contains(&(i as i32)) == false {
                ans.push(i as i32);
                break;
            }
        }
        
        return ans;
    }
}