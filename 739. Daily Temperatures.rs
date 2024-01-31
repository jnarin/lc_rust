/*
Link: https://leetcode.com/problems/daily-temperatures/
Problem: 739. Daily Temperatures
*/

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let len = temperatures.len();
        let mut ans = vec![0; len];
        let mut s = Vec::new();
        
        for i in 0..len {
            while s.len() > 0 && temperatures[s[s.len() - 1]] < temperatures[i] {
                ans[s[s.len() - 1]] = (i - s[s.len() - 1]) as i32;
                s.pop();
            }
            
            s.push(i);
        }
        
        return ans;
        
    }
}