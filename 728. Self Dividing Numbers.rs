/*
 * Problem: 728. Self Dividing Numbers
 * Link: https://leetcode.com/problems/self-dividing-numbers/
 * */

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ans = Vec::new();

        for n in left..=right {
            if Self::check_num(n) == true {
                ans.push(n);
            }
        }

        return ans;
    }

    pub fn check_num(n: i32) -> bool {
        let mut t = n;

        while (t != 0) {
            let d = t % 10;

            if d == 0 {
                return false;
            }

            if n % d != 0 {
                return false;
            }

            t /= 10;
        }

        return true;
    }
}

