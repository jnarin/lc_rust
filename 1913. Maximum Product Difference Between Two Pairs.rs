/*
 * Link: https://leetcode.com/problems/maximum-product-difference-between-two-pairs/
 * Problem: 1913. Maximum Product Difference Between Two Pairs
 * */

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut t: Vec<i32> = nums;
        t.sort();

        let (mut a, mut b, mut c, mut d) = (t[t.len() - 1], t[t.len() - 2], t[0], t[1]);
        return (a * b) - (c * d);
    }
}
