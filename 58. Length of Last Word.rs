/*
 * Problem: 58. Length of Last Word
 * Link: https://leetcode.com/problems/length-of-last-word/
 * */

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let (mut r, mut i) = (0, s.len() - 1);

        while i < s.len() && s.as_bytes()[i] == b' ' {
            i -= 1;
        }

        while i >= 0 && i < s.len() {
            if s.as_bytes()[i] == b' ' {
                break;
            }

            r += 1;
            i -= 1;
        }

        return r;
    }
}
