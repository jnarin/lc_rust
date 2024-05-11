/*
Link: https://leetcode.com/problems/binary-subarrays-with-sum/
Problem: 930. Binary Subarrays With Sum
*/

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let (mut ans, mut prefix) = (0, 0);
        
        map.insert(0, 1);
        
        for n in nums {
            prefix += n;
            if map.contains_key(&(prefix - goal)) == true {
                ans += map[&(prefix - goal)];
            }
            
            *map.entry(prefix).or_insert(0) += 1;
        }
        
        return ans;
    }
}