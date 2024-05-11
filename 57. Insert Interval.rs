/*
Link: https://leetcode.com/problems/insert-interval/
Problem: 57. Insert Interval
*/

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        
        let mut ans: Vec<Vec<i32>> = Vec::new();
        
        if intervals.len() < 1 {
            return ans;
        }
        
        intervals.sort();
        
        ans.push(vec![intervals[0][0], intervals[0][1]]);
        
        for i in 1..intervals.len() {
            let x = ans.len() - 1;
            
            if intervals[i][0] <= ans[x][1] {
                if intervals[i][1] > ans[x][1] {
                    ans[x] = vec![ans[x][0], intervals[i][1]];
                }
            } else {
                ans.push(vec![intervals[i][0], intervals[i][1]]);
            }
        }
        
        return ans;
        
    }
}