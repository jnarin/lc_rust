/*
 * Link: https://leetcode.com/problems/squares-of-a-sorted-array/
 * Problem: 977. Squares of a Sorted Array
 */

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![0; len];
        let (mut l, mut r, mut idx) = (0, len - 1, len - 1);

        while l <= r && r < len {
            if nums[l].abs() > nums[r].abs() {
                ans[idx] = nums[l] * nums[l];
                l += 1;
            } else {
                ans[idx] = nums[r] * nums[r];
                r -= 1;
            }

            idx -= 1;
        }

        return ans;
    }
}
