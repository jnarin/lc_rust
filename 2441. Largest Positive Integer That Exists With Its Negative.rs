/*
 * Problem: 2441. Largest Positive Integer That Exists With Its Negative
 * Link: https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/
 * */

impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let (mut l, mut r) = (0, nums.len() - 1);

        while l < r {
            if nums[l] == nums[r] {
                break;
            }

            if i32::abs(nums[l]) == nums[r] {
                return nums[r];
            }

            if i32::abs(nums[l]) > nums[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        return -1;
    }
}
