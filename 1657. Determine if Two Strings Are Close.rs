/*
 * Link: https://leetcode.com/problems/determine-if-two-strings-are-close/
 * Problem: 1657. Determine if Two Strings Are Close
 * */

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut word1Map = vec![0; 26];
        let mut word2Map = vec![0; 26];

        for i in 0..word1.len() {
            word1Map[(word1.as_bytes()[i] - b'a') as usize] += 1;
        }

        for i in 0..word2.len() {
            word2Map[(word2.as_bytes()[i] - b'a') as usize] += 1;
        }

        if word1Map == word2Map {
            return true;
        }

        for i in 0..26 {
            if word1Map[i] == 0 && word2Map[i] != 0 {
                return false;
            }

            if word1Map[i] != 0 && word2Map[i] == 0 {
                return false;
            }
        }

        word1Map.sort();
        word2Map.sort();

        if word1Map == word2Map {
            return true;
        }

        return false;
    }
}

