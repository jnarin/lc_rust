/*
 * Problem: 1614. Maximum Nesting Depth of the Parentheses
 * Link: https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
 * */

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut count = 0;
        let mut ans = 0;
        use std::cmp::max;

        for i in 0..s.len() {
            let c = s.as_bytes()[i];
            if c == b')' {
                count -= 1;
                continue;
            }

            if c == b'(' {
                count += 1;
                ans = max(ans, count);
            }
        }

        return ans;
    }
}
