/*
 * Link: https://leetcode.com/problems/largest-substring-between-two-equal-characters/
 * Problem: 1624. Largest Substring Between Two Equal Characters
 * */

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut indexMap: Vec<i32> = vec![-1; 26];
        let mut ans = -1;

        for i in 0..s.len() {
            let t: usize = (s.as_bytes()[i] - b'a') as usize;
            if indexMap[t] > -1 {
                ans = std::cmp::max(ans, i as i32 - indexMap[t] - 1);
            } else {
                indexMap[t] = i as i32;
            }
        }

        return ans;
    }
}

