/*
 * Link: https://leetcode.com/problems/decode-ways/
 * Problem: 91. Decode Ways
 * */

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let len = s.len();
        let mut dp = vec![0; len + 1];

        dp[len] = 1;
        if s.as_bytes()[len - 1] > b'0' {
            dp[len - 1] = 1;
        }

        for i in (0..len - 1).rev() {
            if s.as_bytes()[i] != b'0' {
                dp[i] += dp[i + 1];
            }

            if s.as_bytes()[i] == b'1' || (s.as_bytes()[i] == b'2' && s.as_bytes()[i + 1] < b'7') {
                dp[i] += dp[i + 2];
            }
        }

        return dp[0];
    }
}
