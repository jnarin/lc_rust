/*
Link: https://leetcode.com/problems/check-if-bitwise-or-has-trailing-zeros/
Problem: 2980. Check if Bitwise OR Has Trailing Zeros
*/

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut evenNumCount = 0;
        
        for n in nums {
            if (n & 1) == 0 {
                evenNumCount += 1;
                
                if (evenNumCount > 1) {
                    break;
                }
            }
        }
        
        return (evenNumCount > 1);
    }
}