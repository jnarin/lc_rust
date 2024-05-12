/*
 * Problem: 2520. Count the Digits That Divide a Number
 * Link: https://leetcode.com/problems/count-the-digits-that-divide-a-number/
 * */

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut t = num;
        let mut ans = 0;

        while t != 0 {
            let d = t % 10;

            if d != 0 && num % d == 0 {
                ans += 1;
            }

            t /= 10;
        }

        return ans;
    }
}
