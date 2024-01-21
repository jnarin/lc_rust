/*
 * Link: https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-i/
 * Problem: 2839. Check if Strings Can be Made Equal With Operations I
 * */

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut freqMap = vec![vec![0; 26]; 2];

        for i in 0..s1.len() {
            freqMap[i & 1][(s1.as_bytes()[i] - b'a') as usize] += 1;
            freqMap[i & 1][(s2.as_bytes()[i] - b'a') as usize] -= 1;
        }

        for i in 0..26 {
            if freqMap[0][i] != 0 || freqMap[1][i] != 0 {
                return false;
            }
        }

        return true;
    }
}

