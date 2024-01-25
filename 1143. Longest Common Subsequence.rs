/*
Link: https://leetcode.com/problems/longest-common-subsequence/
Problem: 1143. Longest Common Subsequence
*/
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        use std::cmp::max;
        let (m, n) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        for i in 0..m {
            for j in 0..n {
                if text1.as_bytes()[i] == text2.as_bytes()[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j]);
                }
            }
        }
        
        return dp[m][n] as i32;        
    }
}