/*
 * Problem: 165. Compare Version Numbers
 * Link: https://leetcode.com/problems/compare-version-numbers/
 * */

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let (mut v1, mut v2);
        let (mut i, mut j) = (0, 0);

        while i < version1.len() || j < version2.len() {
            v1 = 0;
            v2 = 0;

            while i < version1.len() && version1.as_bytes()[i] != b'.' {
                v1 = (v1 * 10) + (version1.as_bytes()[i] - b'0') as usize;
                i += 1;
            }

            while j < version2.len() && version2.as_bytes()[j] != b'.' {
                v2 = (v2 * 10) + (version2.as_bytes()[j] - b'0') as usize;
                j += 1;
            }

            if v1 < v2 {
                return -1;
            }

            if v1 > v2 {
                return 1;
            }

            i += 1;
            j += 1;
        }

        return 0;
    }
}
