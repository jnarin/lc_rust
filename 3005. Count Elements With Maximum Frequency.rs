/*
Link: https://leetcode.com/problems/count-elements-with-maximum-frequency/
Problem: 3005. Count Elements With Maximum Frequency
*/

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let mut max_frequency = 0;
        use std::collections::HashMap;
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for i in nums {
            *m.entry(i).or_insert(0) += 1;
            
            max_frequency = max(max_frequency, m[&i]);
        }

        for (_, f) in m {
            if f == max_frequency {
                ans += f;
            }
        }

        return ans;
        
    }
}