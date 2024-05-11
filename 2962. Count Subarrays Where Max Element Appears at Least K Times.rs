/*
 * Problem: 2962. Count Subarrays Where Max Element Appears at Least K Times
 * Link: https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
 * */

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_element = *nums.iter().max().unwrap();
        let mut freq = 0;
        let mut ans: i64 = 0;
        let mut l = 0;

        for r in 0..nums.len() {
            if nums[r] == max_element {
                freq += 1;
            }

            while freq == k {
                if nums[l] == max_element {
                    freq -= 1;
                }

                l += 1;
            }

            ans += l as i64;
        }

        return ans;
    }
}


