/*
Link: https://leetcode.com/problems/largest-3-same-digit-number-in-string/
Problem: 2264. Largest 3-Same-Digit Number in String
*/

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut ans: String = String::new();
        let val: Vec<String> = vec!["999".to_string(), "888".to_string(), 
            "777".to_string(), "666".to_string(), "555".to_string(), "444".to_string(), 
            "333".to_string(), "222".to_string(), "111".to_string(), "000".to_string()];
        
        for i in 0..val.len() {
            if (num.contains(&val[i])){
                ans = val[i].clone();
                break;
            }
        }
        
        return ans;
    }
}


