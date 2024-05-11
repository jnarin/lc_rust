/*
Link: https://leetcode.com/problems/find-the-pivot-integer/
Problem: 2485. Find the Pivot Integer
*/

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let t = (n * (n + 1)) >> 1;
        let mut ans = -1;
        
        for i in 1..=n {
            let a = (i * (i + 1)) >> 1;
            let b = t - a + i;
            
            if a == b {
                ans = i;
                break;
            }
        }
        
        return ans;
    }
}