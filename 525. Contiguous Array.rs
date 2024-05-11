/*
Link: https://leetcode.com/problems/contiguous-array/
Problem: 525. Contiguous Array
*/

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut array: Vec<i32> = vec![-2; 1 + (nums.len() << 1)];
        let (mut maxLen, mut count) = (0, 0);
        
        array[nums.len()] = -1;
        
        for i in 0..nums.len() {
            if nums[i] == 0 {
                count -= 1;
            } else {
                count += 1;
            }
            
            if array[count + nums.len()] >= -1 {
                maxLen = max(maxLen, i - array[count + nums.len()] as usize);
            } else {
                array[count as usize + nums.len()] = i as i32;
            }
        }
        
        return maxLen as i32;
    }
}

