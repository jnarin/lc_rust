/*
Link: https://leetcode.com/problems/bitwise-and-of-numbers-range/
Problem: 201. Bitwise AND of Numbers Range
*/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut count = 0;
        let (mut left, mut right) = (left, right);
        
        while (left != right) {
            left >>= 1;
            right >>= 1;
            
            count += 1;
        }
        
        return left << count;
    }
}