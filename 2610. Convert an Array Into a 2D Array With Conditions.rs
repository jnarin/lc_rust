/*
Link: https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/
Problem: 2610. Convert an Array Into a 2D Array With Conditions
*/

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut m: HashMap<i32, usize> = HashMap::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        
        for n in nums {
            *m.entry(n).or_insert(0) += 1;
            
            if m[&n] > ans.len() {
                ans.push(vec![]);
            }
            
            ans[m[&n] - 1].push(n);
        }
        
        return ans;
    }
}
