/*
 * Link: https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/
 * Problem: 1758. Minimum Changes To Make Alternating Binary String
 * */

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let (mut x, mut y): (i32, i32) = (0, 0);

        for i in 0..s.len() {
            let c = s.as_bytes()[i];

            if (c - b'0') as usize == i & 1 {
                x += 1;
            }
        }

        y = s.len() as i32 - x;

        return std::cmp::min(x, y);
    }
}
