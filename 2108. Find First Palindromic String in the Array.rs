/*
Link: https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
Problem: 2108. Find First Palindromic String in the Array
*/

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        let mut ans = "".to_string();
        for i in 0..words.len() {
            if Self::is_palindrome(words[i].clone()) == true {
                ans = words[i].clone();
                break;
            }
        }
        
        return ans;
    }
    
    pub fn is_palindrome(word: String) -> bool {
        let len = word.len();
        let mut ans = true;
        
        for i in 0..(len >> 1) {
            if word.as_bytes()[i] != word.as_bytes()[len - 1 - i] {
                ans = false;
                break;
            }
        }
        
        return ans;
    }
}