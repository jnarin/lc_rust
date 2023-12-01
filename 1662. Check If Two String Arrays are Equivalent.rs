/* 
 * Link: https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
 * Problem: 1662. Check If Two String Arrays are Equivalent
 * */

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut s1: String = String::from("");
        let mut s2: String = String::from("");

        for w in word1 {
            for c in w.chars() {
                s1.push(c);
            }
        }

        for w in word2 {
            for c in w.chars() {
                s2.push(c);
            }
        }

        return s1 == s2;
    }
}

