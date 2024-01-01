/*
 * Link: https://leetcode.com/problems/first-unique-character-in-a-string/
 * Problem: 387. First Unique Character in a String
 * */

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut m = vec![0; 26];

        for i in 0..s.len() {
            m[(s.as_bytes()[i] - b'a') as usize] += 1;
        }

        for i in 0..s.len() {
            if m[(s.as_bytes()[i] - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        return -1;
    }
}
