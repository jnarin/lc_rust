/*
Link: https://leetcode.com/problems/rearrange-array-elements-by-sign/
Problem: 2149. Rearrange Array Elements by Sign
*/

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let (mut i, mut p, mut n) = (0, 0, 0);
        
        while i < nums.len() {
            while p < nums.len() && nums[p] < 0 {
                p += 1;
            }
            
            if p < nums.len() {
                ans.push(nums[p]);
                i += 1;
                p += 1;
            }
            
            while n < nums.len() && nums[n] > 0 {
                n += 1;
            }
            
            if n < nums.len() {
                ans.push(nums[n]);
                i += 1;
                n += 1;
            }
        }
        
        return ans;
    }
}