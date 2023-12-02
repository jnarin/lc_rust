/*
 * Link: https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
 * Problem: 1160. Find Words That Can Be Formed by Characters
 * */

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        use std::collections::HashMap;
        let mut charsMap: HashMap<char, i32> = HashMap::new();
        let mut ans: i32 = 0;

        for c in chars.chars() {
            *charsMap.entry(c).or_insert(0) += 1;
        }

        for w in words {
            let mut wordMap: HashMap<char, i32> = HashMap::new();
            let mut skipWord: bool = false;

            for c in w.chars() {
                *wordMap.entry(c).or_insert(0) += 1;
            }

            for c in w.chars() {
                if charsMap.contains_key(&c) == false {
                    skipWord = true;
                    break;
                }

                if (wordMap.get(&c) > charsMap.get(&c)){
                    skipWord = true;
                    break;
                }
            }

            if false == skipWord {
                ans += w.len() as i32;
            }
        }

        return ans;

    }
}

