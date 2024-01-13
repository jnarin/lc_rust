/*
 * Link: https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/
 * Problem: 1374. Generate a String With Characters That Have Odd Counts
 * */

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut ans = String::new();

        if (n & 1) == 1 {
            for i in 0..n {
                ans.push('p');
            }
        } else {
            for i in 0..(n - 1) {
                ans.push('q');
            }

            ans.push('p');
        }

        return ans;
    }
}
