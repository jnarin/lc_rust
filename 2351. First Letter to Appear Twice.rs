/*
 * Link: https://leetcode.com/problems/first-letter-to-appear-twice/
 * Problem: 2351. First Letter to Appear Twice
 * */

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut m = vec![0; 26];

        for i in 0..s.len() {
            let c = (s.as_bytes()[i] - b'a') as usize;
            m[c] += 1;
            if m[c] == 2 {
                return s.as_bytes()[i] as char;
            }
        }

        return 0 as char;
    }
}

