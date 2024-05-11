/*
 * Problem: 2073. Time Needed to Buy Tickets
 * Link: https://leetcode.com/problems/time-needed-to-buy-tickets/
 * */

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        use std::cmp::min;
        let mut ans = 0;

        for i in 0..tickets.len() {
            if i <= k as usize {
                ans += min(tickets[i], tickets[k as usize]);
            } else {
                ans += min(tickets[i], tickets[k as usize] - 1);
            }
        }

        return ans;
    }
}
