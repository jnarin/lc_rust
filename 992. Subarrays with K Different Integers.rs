/*
 * Problem: 992. Subarrays with K Different Integers
 * Link: https://leetcode.com/problems/subarrays-with-k-different-integers/
 * */

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        return Self::subarrays_with_atmost_k_distinct(nums.clone(), k)
             - Self::subarrays_with_atmost_k_distinct(nums.clone(), k - 1);
    }

    pub fn subarrays_with_atmost_k_distinct(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut ans = 0;
        let mut count = vec![0; nums.len() + 1];
        let (mut l, mut r) = (0, 0);

        while (r < nums.len()) {
            count[nums[r] as usize] += 1;
            if count[nums[r] as usize] == 1 {
                k -= 1;
            }

            while k == -1 {
                count[nums[l] as usize] -= 1;
                if count[nums[l] as usize] == 0 {
                    k += 1
                }

                l += 1;
            }
            ans += r - l + 1;

            r += 1;
        }

        return ans as i32;
    }
}
