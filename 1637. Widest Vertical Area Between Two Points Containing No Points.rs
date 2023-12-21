/*
Link: https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
Problem: 1637. Widest Vertical Area Between Two Points Containing No Points
*/

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut x: Vec<i32>= Vec::new();
        
        for p in points {
            x.push(p[0]);
        }
        
        x.sort();
        
        let mut ans = i32::MIN;
        
        for i in 0..(x.len() - 1) {
            ans = std::cmp::max(ans, x[i + 1] - x[i]);
        }
        
        return ans;        
    }
}