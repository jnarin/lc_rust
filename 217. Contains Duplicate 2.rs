/*
 * Link: https://leetcode.com/problems/contains-duplicate/
 * Problem: 217. Contains Duplicate
 * */

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;

        nums.sort();

        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                return true;
            }
        }

        return false;
    }
}

