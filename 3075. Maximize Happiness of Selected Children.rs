/*
 * Problem: 3075. Maximize Happiness of Selected Children
 * Link: https://leetcode.com/problems/maximize-happiness-of-selected-children/
 * */

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut dec = 0;
        use std::cmp::max;

        happiness.sort_by(|a, b| b.cmp(a));

        for i in 0..k {
            ans += max(0, happiness[i as usize] - dec) as i64;
            dec += 1;
        }

        return ans;
    }
}
