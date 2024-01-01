/*
 * Link: https://leetcode.com/problems/majority-element/
 * Problem: 169. Majority Element
 * */

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut ans = nums[0];

        for i in 1..nums.len() {
            if ans == nums[i] {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    ans = nums[i];
                    count += 1;
                }
            }
        }

        return ans;
    }
}

