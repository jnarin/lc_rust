/*
Link: https://leetcode.com/problems/perfect-squares/
Problem: 279. Perfect Squares
*/

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let t = n + 1;
        let mut dp = vec![i32::MAX; t as usize];
        use std::cmp::min;
        
        dp[0]  = 0;
        let mut count = 1;
        let mut s = count * count;
        
        while s <= n {
            for i in s..t {
                dp[i as usize] = min(dp[(i - s) as usize] + 1, dp[i as usize]);
            }
            
            count += 1;
            s = count * count;
        }
        
        return dp[n as usize];
    }
}