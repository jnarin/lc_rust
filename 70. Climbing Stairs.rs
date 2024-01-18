/*
Link: https://leetcode.com/problems/climbing-stairs/
Problem: 70. Climbing Stairs
*/

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        
        if n == 2 {
            return 2;
        }
        
        let (mut ans, mut a, mut b) = (0, 1, 2);
        
        for i in 2..n {
            ans = a + b;
            a = b;
            b = ans;
        }
        
        return ans;
    }
}