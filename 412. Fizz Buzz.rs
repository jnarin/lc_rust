/*
 * Link: https://leetcode.com/problems/fizz-buzz/
 * Problem: 412. Fizz Buzz
 * */

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        for i in 1..=n {
            if i % 15 == 0 {
                ans.push("FizzBuzz".to_string());
            } else if i % 5 == 0 {
                ans.push("Buzz".to_string());
            } else if i % 3 == 0 {
                ans.push("Fizz".to_string());
            } else {
                ans.push(i.to_string());
            }
        }

        return ans;
    }
}

