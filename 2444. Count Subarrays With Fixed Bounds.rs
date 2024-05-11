/*
 * Problem: 2444. Count Subarrays With Fixed Bounds
 * Link: https://leetcode.com/problems/count-subarrays-with-fixed-bounds/
 * */

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        use std::cmp::max;
        use std::cmp::min;

        let mut ans = 0 as i64;
        let mut j: i32 = -1;
        let mut prev_min_k_index: i32 = -1;
        let mut prev_max_k_index: i32 = -1;

        for i in 0..nums.len() {
            if nums[i] < min_k || nums[i] > max_k {
                j = i as i32;
            }

            if nums[i] == min_k {
                prev_min_k_index = i as i32;
            }

            if nums[i] == max_k {
                prev_max_k_index = i as i32;
            }

            ans += max(0, min(prev_min_k_index, prev_max_k_index) - j) as i64;
        }

        return ans;
    }
}

