/*
Link: https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/
Problem: 2482. Difference Between Ones and Zeros in Row and Column
*/

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut rows:Vec<i32> = vec![0; grid.len()];
        let mut cols:Vec<i32> = vec![0; grid[0].len()];
        
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                rows[i] += grid[i][j];
                cols[j] += grid[i][j];
            }
        }
        
        let mut ans: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                /*
                rows[i] and cols[j] have number of 1's in row i and col j, respectively
                Number of zeros for i'th row is rowLen - rows[i]
                Number of zeros for i'th col is colLen - cols[i]
                Hence, ans[i][j] = rows[i] + cols[j] - 
                                    (grid.len() - rows[i]) - (grid[0].len() - cols[j])
                which reduces to the below equation
                */
                ans[i][j] = (2 * rows[i]) + (2 * cols[j]) - grid.len() as i32 - grid[0].len() as i32;
            }
        }
        
        return ans;
    }
}