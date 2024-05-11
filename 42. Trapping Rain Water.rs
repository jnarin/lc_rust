/*
 * Problem: 42. Trapping Rain Water
 * Link: https://leetcode.com/problems/trapping-rain-water/
 * */

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut l = vec![0; height.len()];
        let mut r = vec![0; height.len()];
        use std::cmp::max;
        use std::cmp::min;

        for i in 1..height.len() - 1 {
            l[i] = max(l[i - 1], height[i - 1]);
        }

        for i in (0..height.len() - 1).rev() {
            r[i] = max(r[i + 1], height[i + 1]);
        }

        for i in 1..height.len() - 1 {
            ans += max(0, min(l[i], r[i]) - height[i]);
        }

        return ans;
    }
}
