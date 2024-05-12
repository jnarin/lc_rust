/*
 * Problem: 1915. Number of Wonderful Substrings
 * Link: https://leetcode.com/problems/number-of-wonderful-substrings/
 * */

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut ans = 0;
        let mut prefix = 0;
        let mut count = vec![0; 1024];

        count[0] = 1;

        for i in 0..word.len() {
            prefix ^= 1 << (word.as_bytes()[i] - b'a');

            ans += count[prefix as usize];

            for j in 0..10 {
                ans += count[prefix as usize ^ (1 << j)];
            }

            count[prefix as usize] += 1;
        }

        return ans;
    }
}
