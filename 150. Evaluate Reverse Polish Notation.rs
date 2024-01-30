/*
Link: https://leetcode.com/problems/evaluate-reverse-polish-notation/
Problem: 150. Evaluate Reverse Polish Notation
*/

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut nums: Vec<i32> = Vec::new();
        let (mut a, mut b, mut c);
        
        for t in tokens {
            if t == "+" {
                a = nums[nums.len() - 1];
                nums.pop();
                b = nums[nums.len() - 1];
                nums.pop();
                
                c = a + b;
            } else if t == "-" {
                a = nums[nums.len() - 1];
                nums.pop();
                b = nums[nums.len() - 1];
                nums.pop();
                
                c = b - a;
            } else if t == "*" {
                a = nums[nums.len() - 1];
                nums.pop();
                b = nums[nums.len() - 1];
                nums.pop();
                
                c = a * b;
            } else if t == "/" {
                a = nums[nums.len() - 1];
                nums.pop();
                b = nums[nums.len() - 1];
                nums.pop();
                
                c = b / a;
            } else {
                nums.push(t.parse::<i32>().unwrap());
                continue;
            }
            
            nums.push(c);
        }
        
        return nums[nums.len() - 1];
    }
}