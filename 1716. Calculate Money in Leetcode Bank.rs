/*
Link: https://leetcode.com/problems/calculate-money-in-leetcode-bank/
Problem: 1716. Calculate Money in Leetcode Bank
*/

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut start: i32 = 0;
        let mut currentBal: i32;
        let mut t: i32 = 0;
        let numDays: i32 = 7;
        
        for i in 0..n {
            if i % numDays == 0 {
                currentBal = start + 1;
                start = currentBal;
            } else {
                currentBal = t + 1;
            }
            
            t = currentBal;
            ans += currentBal;
        }
        
        return ans;
    }
}