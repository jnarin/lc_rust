/*
 * Link: https://leetcode.com/problems/move-zeroes/
 * Problem: 283. Move Zeroes
 * */

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut t = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[t] = nums[i];
                t += 1;
            }
        }

        for i in t..nums.len() {
            nums[i] = 0;
        }

        return;
    }
}
