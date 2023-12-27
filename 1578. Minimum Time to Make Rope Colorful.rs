/*
 * Link: https://leetcode.com/problems/minimum-time-to-make-rope-colorful/
 * Problem: 1578. Minimum Time to Make Rope Colorful
 * */

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;
        let (mut c, mut p);
        let mut needed_time = needed_time;

        if colors.len() == 0 {
            return ans;
        }

        p = colors.as_bytes()[0];

        for i in 1..needed_time.len() {
            if p == colors.as_bytes()[i] {
                c = std::cmp::min(needed_time[i], needed_time[i - 1]);
                ans += c;
                needed_time[i] = std::cmp::max(needed_time[i], needed_time[i - 1]);
            } else {
                p = colors.as_bytes()[i];
            }
        }

        ans
    }
}
