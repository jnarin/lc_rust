/*
Link: https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
Problem: 1464. Maximum Product of Two Elements in an Array
*/
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut t: Vec<i32> = vec![i32::MIN, i32::MIN];
        
        for i in nums {
            if i > t[0] {
                t[1] = t[0];
                t[0] = i;
            } else if i > t[1] {
                t[1] = i;
            }
        }
        
        return (t[0] - 1) * (t[1] - 1);
    }
}