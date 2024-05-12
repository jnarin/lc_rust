/*
 * Problem: 881. Boats to Save People
 * Link: https://leetcode.com/problems/boats-to-save-people/
 * */

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        let (mut l, mut r) = (0, people.len() - 1);
        let mut ans = 0;

        people.sort();

        while r < people.len() && l <= r {
            if people[l] + people[r] <= limit {
                l += 1;
                r -= 1;
            } else {
                r -= 1;
            }

            ans += 1;
        }

        return ans;
    }
}
