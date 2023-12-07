/*
Link: https://leetcode.com/problems/largest-odd-number-in-string/
Problem: 1903. Largest Odd Number in String
*/

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut t: Vec<char> = Vec::new();
        
        for c in num.chars() {
            t.push(c);
        }
        
        for i in (0..num.len()).rev() {
            if ((t[i] as u8 - b'0') & 1) == 1 {
                return t[0..=i].iter().collect();
            }
        }
        
        return "".to_string();        
    }
}