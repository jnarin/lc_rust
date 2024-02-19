/*
Link: https://leetcode.com/problems/power-of-two/
Problem: 231. Power of Two
*/

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        return n > 0 && (n & (n - 1)) == 0;        
    }
}