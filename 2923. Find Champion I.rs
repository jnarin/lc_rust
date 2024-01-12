/*
 * Link: https://leetcode.com/problems/find-champion-i/
 * Problem: 2923. Find Champion I
 * */

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            let sum: i32 = grid[i].iter().sum();
            if sum == (grid.len() - 1) as i32 {
                return i as i32;
            }
        }

        return -1;
    }
}
