/*
 * Problem: 2373. Largest Local Values in a Matrix
 * Link: https://leetcode.com/problems/largest-local-values-in-a-matrix/
 * */

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len() - 2;
        let mut ans = vec![vec![0; n]; n];
        use std::cmp::max;

        for i in 0..n {
            for j in 0..n {
                for x in i..(i + 3) {
                    for y in j..(j + 3){
                        ans[i][j] = max(ans[i][j], grid[x][y]);
                    }
                }
            }
        }

        return ans;
    }
}
