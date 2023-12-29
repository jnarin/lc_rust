/*
 * Link: https://leetcode.com/problems/single-number/
 * Problem: 136. Single Number
 * */

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for n in nums {
            ans ^= n;
        }

        return ans;
    }
}

