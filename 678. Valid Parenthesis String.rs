/*
 * Problem: 678. Valid Parenthesis String
 * Link: https://leetcode.com/problems/valid-parenthesis-string/
 * */

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        use std::cmp::max;
        let (mut low, mut high) = (0, 0);

        for i in 0..s.len() {
            let c = s.as_bytes()[i];

            if c == b'(' {
                low += 1;
                high += 1;
            } else if c == b')' {
                low -= 1;
                high -= 1;
                low = max(0, low);
            } else if c == b'*' {
                low -= 1;
                high += 1;
                low = max(0, low);
            }

            if high < 0 {
                return false;
            }
        }

        return low == 0;
    }
}
