/*
Link: https://leetcode.com/problems/missing-number/
Problem: 268. Missing Number
*/

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        return ((nums.len() * (nums.len() + 1)) >> 1) as i32 
                - nums.iter().sum::<i32>();
    }
}