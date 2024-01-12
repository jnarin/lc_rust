/*
Link: https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
Problem: 1365. How Many Numbers Are Smaller Than the Current Number
*/

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sortedNums = nums.clone();
        sortedNums.sort_by(|a, b| b.cmp(a));
                
        let len = sortedNums.len();
        let mut ans = vec![0; nums.len()];
        
        use std::collections::HashMap;
        let mut map: HashMap<i32, usize> = HashMap::new();
        
        for i in 0..len {
            map.insert(sortedNums[i], i);
        }
        
        for i in 0..len {
            ans[i] = (len - 1 - map[&nums[i]]) as i32;
        }
        
        return ans;        
    }
}