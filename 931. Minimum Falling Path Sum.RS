/*
Link: https://leetcode.com/problems/minimum-falling-path-sum/
Problem: 931. Minimum Falling Path Sum
*/

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        
        let len = matrix.len();
        if len == 0 { 
            return 0;
        }
        
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len]; len];
        for i in 0..len {
            dp[0][i] = matrix[0][i];
        }
        
        for i in 1..len {
            for j in 0..len {
                let (mut x, mut y) = (i32::MAX, i32::MAX);
                
                if j > 0 {
                    x = dp[i - 1][j - 1];
                }
                
                if j + 1 < len {
                    y = dp[i - 1][j + 1];
                }
                
                dp[i][j] = min(min(x, y), dp[i - 1][j]) + matrix[i][j];
            }
        }
        
        let mut ans = i32::MAX;
        for i in 0..len {
            ans = min(ans, dp[len - 1][i]);
        }
        
        return ans;
    }
}