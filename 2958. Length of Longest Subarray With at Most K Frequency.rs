/*
 * Problem: 2958. Length of Longest Subarray With at Most K Frequency
 * Link: https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/
 * */
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::max;
        let (mut l, mut r) = (0, 0);
        let mut ans = 0;
        use std::collections::HashMap;
        let mut freq: HashMap<i32, i32> = HashMap::new();

        while (r < nums.len()){
            *freq.entry(nums[r]).or_insert(0) += 1;
            while (freq[&nums[r]] == k + 1) {
                *freq.entry(nums[l]).or_insert(0) -= 1;
                l += 1;
            }

            ans = max(ans, r - l + 1);
            r += 1;
        }

        return ans as i32;
    }
}
