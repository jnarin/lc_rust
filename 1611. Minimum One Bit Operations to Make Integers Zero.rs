/*
Link: https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/
Problem: 1611. Minimum One Bit Operations to Make Integers Zero
*/

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        
        let mut bit = 0;
        while (1 << bit) <= n {
            bit = bit + 1;
        }
        
        bit = bit - 1;
        
        return ((1 << bit + 1) - 1) - Solution::minimum_one_bit_operations(n - (1 << bit));
        
    }
}