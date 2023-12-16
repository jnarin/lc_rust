/*
 * Link: https://leetcode.com/problems/valid-anagram/
 * Problem: 242. Valid Anagram
 * */

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut m: Vec<usize> = vec![0; 128];

        for c in s.chars() {
            m[c as usize] += 1;
        }

        for c in t.chars() {
            m[c as usize] -= 1;
        }

        let mut ans: bool = true;

        for n in m {
            if n != 0 {
                ans = false;
                break;
            }
        }

        return ans;
    }
}
