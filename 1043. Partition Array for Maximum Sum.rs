/*
 * Link: https://leetcode.com/problems/partition-array-for-maximum-sum/
 * Problem: 1043. Partition Array for Maximum Sum
 * */

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        use std::cmp::min;
        use std::cmp::max;

        let mut dp = vec![0; arr.len() + 1];

        for i in 1..=arr.len() {
            let mut m = i32::MIN;
            for j in 1..=min(i, k as usize) {
                m = max(m, arr[i - j]);
                dp[i] = max(dp[i], dp[i - j] + m * j as i32);
            }
        }

        dp[arr.len()]
    }
}

