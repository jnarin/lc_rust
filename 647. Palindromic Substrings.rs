/*
 * Link: https://leetcode.com/problems/palindromic-substrings/
 * Problem: 647. Palindromic Substrings
 * */

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let len = s.len();
        let mut ans = len as i32;
        let mut dp = vec![vec![false; len]; len];

        for i in 0..len {
            dp[i][i] = true;
        }

        for i in 0..(len - 1) {
            if s.as_bytes()[i] == s.as_bytes()[i + 1] {
                dp[i][i + 1] = true;
                ans += 1;
            }
        }
        for k in 3..=len {
            for i in 0..(len - k + 1) {
                let j = i + k - 1;
                if dp[i + 1][j - 1] == true && s.as_bytes()[i] == s.as_bytes()[j] {
                    dp[i][j] = true;
                    ans += 1;
                }
            }
        }

        return ans;
    }
}

