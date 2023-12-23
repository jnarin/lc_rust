/*
 * Link: https://leetcode.com/problems/path-crossing/
 * Problem: 1496. Path Crossing
 * */

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        use std::collections::HashSet;

        let mut s: HashSet<String> = HashSet::new();
        let (mut x, mut y) = (0, 0);

        let mut p = format!("{},{}", x, y);
        s.insert(p);

        for c in path.chars() {
            if c == 'N' {
                x += 1;
            } else if c == 'S' {
                x -= 1;
            } else if c == 'E' {
                y += 1;
            } else if c == 'W' {
                y -= 1;
            }

            p = format!("{},{}", x, y);
            if s.contains(&p) {
                return true;
            }

            s.insert(p);
        }

        return false;
    }
}

