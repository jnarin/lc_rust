/*
Link: https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
Problem: 1481. Least Number of Unique Integers after K Removals
*/
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        let mut k = k;
        
        for n in arr {
            *m.entry(n).or_insert(0) += 1;
        }
        
        let mut freq = Vec::new();
        for (_, f) in m {
            freq.push(f);
        }
        
        freq.sort();
        
        for i in 0..freq.len() {
            k -= freq[i];
            
            if k < 0 {
                return (freq.len() - i) as i32;
            }
        }
        
        return 0;
    }
}