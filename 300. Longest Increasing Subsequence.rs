/*
Link: https://leetcode.com/problems/longest-increasing-subsequence/
Problem: 300. Longest Increasing Subsequence
*/

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len() + 1];
        
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] && dp[i] < (dp[j] + 1) {
                    dp[i] = dp[j] + 1;
                }
            }
        }
        
        return *dp.iter().max().unwrap() as i32;
    }
}