/*
 * Link: https://leetcode.com/problems/contains-duplicate/
 * Problem: 217. Contains Duplicate
 * */

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut s = HashSet::new();

        for i in 0..nums.len() {
            if s.contains(&nums[i]) {
                return true;
            }

            s.insert(nums[i]);
        }

        return false;
    }
}

