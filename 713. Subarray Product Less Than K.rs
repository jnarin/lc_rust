/*
 * Problem: 713. Subarray Product Less Than K
 * Link: https://leetcode.com/problems/subarray-product-less-than-k/
 * */

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        let (mut l, mut r) = (0, 0);
        let mut p = 1;
        let mut ans = 0;

        while r < nums.len() {
            p *= nums[r];

            while l < nums.len() && p >= k {
                p /= nums[l];
                l += 1;
            }

            ans += r - l + 1;

            r += 1;
        }

        return ans as i32;
    }
}

