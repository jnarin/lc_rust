/*
Link: https://leetcode.com/problems/transpose-matrix/submissions/
Problem: 867. Transpose Matrix
*/
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let r = matrix.len();
        let c = matrix[0].len();
        let mut ans: Vec<Vec<i32>> = vec![vec![0; r]; c];
       
        for i in 0..r {
            for j in 0..c {
                ans[j][i] = matrix[i][j];
            }
        }
        
        return ans;
    }
}