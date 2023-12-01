/*
 * Link: https://leetcode.com/problems/palindrome-number/
 * Problem: 9. Palindrome Number
 * */

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let (mut original, mut reverse): (i32, i32) = (x, 0);
        while original != 0 {
            reverse = (reverse * 10) + (original % 10);
            original = original / 10;
        }

        return reverse == x;
    }
}
