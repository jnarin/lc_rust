/*
 * Problem: 2997. Minimum Number of Operations to Make Array XOR Equal to K
 * Link: https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/
 * */

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut xor_total = 0;

        for i in 0..nums.len() {
            xor_total ^= nums[i];
        }

        xor_total ^= k;

        return xor_total.count_ones() as i32;
    }
}
