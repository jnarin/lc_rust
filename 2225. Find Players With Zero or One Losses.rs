/*
 * Link: https://leetcode.com/problems/find-players-with-zero-or-one-losses/
 * Problem: 2225. Find Players With Zero or One Losses
 * */

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut winnersMap: HashMap<i32, i32> = HashMap::new();
        let mut losersMap: HashMap<i32, i32> = HashMap::new();

        for m in matches {
            *winnersMap.entry(m[0]).or_insert(0) += 1;
            *losersMap.entry(m[1]).or_insert(0) += 1;
        }

        let mut ans: Vec<Vec<i32>> = vec![vec![]; 2];

        for (n, c) in losersMap {
            if winnersMap.contains_key(&n) == true {
                winnersMap.remove_entry(&n);
            }

            if c == 1 {
                ans[1].push(n);
            }
        }

        for (n, _) in winnersMap {
            ans[0].push(n);
        }

        ans[0].sort();
        ans[1].sort();

        return ans;
    }
}

