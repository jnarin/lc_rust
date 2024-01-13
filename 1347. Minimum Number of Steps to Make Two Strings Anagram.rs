/*
 * Link: https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
 * Problem: 1347. Minimum Number of Steps to Make Two Strings Anagram
 * */

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut mapS = vec![0; 26];
        let mut mapT = vec![0; 26];
        let mut ans = 0;

        for i in 0..s.len() {
            let c = (s.as_bytes()[i] - b'a') as usize;
            mapS[c] += 1;
        }

        for i in 0..t.len() {
            let c = (t.as_bytes()[i] - b'a') as usize;
            mapT[c] += 1;
        }

        for i in 0..mapS.len() {
            if mapS[i] == mapT[i] {
                continue;
            }

            if mapT[i] < mapS[i] {
                ans += mapS[i] - mapT[i];
            }
        }

        return ans;
    }
}

