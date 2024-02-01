/*
Link: https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/
Problem: 2966. Divide Array Into Arrays With Max Difference
*/

impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        
        nums.sort();
        
        for i in (2..nums.len()).step_by(3) {
            if nums[i] - nums[i - 2] > k {
                return vec![];
            }
            
            ans.push(vec![nums[i - 2], nums[i - 1], nums[i]]);
        }
        
        return ans;
    }
}