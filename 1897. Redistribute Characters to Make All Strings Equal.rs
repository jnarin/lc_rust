/*
 * Link: https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
 * Problem: 1897. Redistribute Characters to Make All Strings Equal
 * */

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut m = vec![0; 26];
        let len = words.len();

        for word in words {
            for c in word.chars() {
                m[(c as u8 - b'a') as usize] += 1;
            }
        }

        for n in m {
            if n != 0 {
                if n % len != 0 {
                    return false;
                }
            }
        }

        return true;
    }
}
