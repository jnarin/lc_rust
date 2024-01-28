/*
 * Link: https://leetcode.com/problems/sort-colors/
 * Problem: 75. Sort Colors
 * */

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut colorCount = vec![0; 3];

        for i in 0..nums.len() {
            colorCount[nums[i] as usize] += 1;
        }

        nums.clear();
        for i in 0..3 {
            for c in 0..colorCount[i] {
                nums.push(i as i32);
            }
        }

        return;
    }
}
