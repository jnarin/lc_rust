/*
Link: https://leetcode.com/problems/special-positions-in-a-binary-matrix/
Problem: 1582. Special Positions in a Binary Matrix
*/

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans: i32 = 0;
        let mut rows: Vec<i32> = vec![0; mat.len()];
        let mut cols: Vec<i32> = vec![0; mat[0].len()];
        
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }
        
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 && rows[i] == 1 && cols[j] == 1 {
                    ans += 1;
                }
            }
        }
        
        return ans;
    }
}