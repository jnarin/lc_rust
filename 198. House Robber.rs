/*
 * Link: https://leetcode.com/problems/house-robber/
 * Problem: 198. House Robber
 * */

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;

        let (mut x, mut y, mut ans) = (0, 0, 0);

        for n in nums {
            ans = max(n + x, y);
            x = y;
            y = ans;
        }

        return ans;
    }
}

