/*
Link: https://leetcode.com/problems/minimum-bit-flips-to-convert-number/
Problem: 2220. Minimum Bit Flips to Convert Number
*/

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        return (start ^ goal).count_ones() as i32;
    }
}