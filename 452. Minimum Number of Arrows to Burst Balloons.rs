/*
Link: https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
Problem: 452. Minimum Number of Arrows to Burst Balloons
*/

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 1 {
            return points.len() as i32;
        }
        
        points.sort();
        
        let mut temp_array: Vec<Vec<i32>> = Vec::new();
        temp_array.push(points[0].clone());
        
        for i in 1..points.len() {
            let t = temp_array.len() - 1;
            if points[i][0] <= temp_array[t][1] {
                if points[i][1] < temp_array[t][1] {
                    temp_array[t][1] = points[i][1];
                }
            } else {
                temp_array.push(points[i].clone());
            }
        }
        
        return temp_array.len() as i32;
    }
}