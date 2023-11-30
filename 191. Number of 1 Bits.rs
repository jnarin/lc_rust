/*
Link: https://leetcode.com/problems/number-of-1-bits/
Problem: 191. Number of 1 Bits
*/
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut ans: i32 = 0; 
        let mut val: u32 = 32;
        
        while val > 0 {
            if ((n & (1 << val)) != 0){
                ans = ans + 1;
            }
            
            val = val - 1;
        }
        
        return ans;
    }
}