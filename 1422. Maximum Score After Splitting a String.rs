/*
 * Link: https://leetcode.com/problems/maximum-score-after-splitting-a-string/
 * Problem: 1422. Maximum Score After Splitting a String
 * */

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut ans = i32::MIN;
        let (mut countOnes, mut countZeros) = (0, 0);

        for i in 0..s.len() {
            let c = s.as_bytes()[i];
            if c == b'0' {
                countZeros += 1;
            } else {
                countOnes += 1;
            }

            if i == (s.len() - 1) {
                break;
            }

            ans = std::cmp::max(ans, countZeros - countOnes);
        }

        return ans + countOnes;
    }
}
