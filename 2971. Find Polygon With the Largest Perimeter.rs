/*
Link: https://leetcode.com/problems/find-polygon-with-the-largest-perimeter/
Problem: 2971. Find Polygon With the Largest Perimeter
*/

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort();
        
        let mut p: i64 = 0;
        
        for i in 0..nums.len() {
            p += nums[i] as i64;
        }
        
        for i in (0..nums.len()).rev() {
            let l = p - nums[i] as i64;
            
            if l <= nums[i] as i64 {
                p = p - nums[i] as i64;
            }
        }
        
        if p == 0 {
            return -1;
        }
        
        return p;
    }
}