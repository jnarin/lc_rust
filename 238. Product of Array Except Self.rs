/*
Link: https://leetcode.com/problems/product-of-array-except-self/
Problem: 238. Product of Array Except Self
*/

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let (mut left, mut right) = (1, 1);
        let mut ans = vec![0; nums.len()];
        
        for i in 0..nums.len() {
            ans[i] = left;
            left *= nums[i];
        }
        
        for i in (0..nums.len()).rev() {
            ans[i] *= right;
            right *= nums[i];
        }
        
        return ans;        
    }
}