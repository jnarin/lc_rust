/*
 * Link: https://leetcode.com/problems/split-a-string-in-balanced-strings/
 * Problem: 1221. Split a String in Balanced Strings
 * */

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let (mut ans, mut count) = (0, 0);

        for c in s.chars() {
            if c == 'L' {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                ans += 1;
            }
        }

        return ans;
    }
}

