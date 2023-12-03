/*
 * Link: https://leetcode.com/problems/minimum-time-visiting-all-points/
 * Problem: 1266. Minimum Time Visiting All Points
 * */

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;

        let mut ans: i32 = 0;

        for i in 1..points.len() {
            ans += max(i32::abs(points[i][0] - points[i - 1][0]), i32::abs(points[i][1] - points[i - 1][1]));
        }

        return ans;
    }
}
