/*
Link: https://leetcode.com/problems/unique-number-of-occurrences/
Problem: 1207. Unique Number of Occurrences
*/

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut freqMap = HashMap::new();
        let mut freqCountSet = HashSet::new();
        
        for n in arr {
            *freqMap.entry(n).or_insert(0) += 1;
        }
        
        for (n, f) in freqMap {
            if freqCountSet.contains(&f) {
                return false;
            }
            
            freqCountSet.insert(f);
        }
        
        return true;
    }
}