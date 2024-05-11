/*
 * Problem: 41. First Missing Positive
 * Link: https://leetcode.com/problems/first-missing-positive/
 * */

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        for i in 0..len {
            while nums[i] > 0 && nums[i] <= len as i32 && nums[i] != nums[(nums[i] - 1) as usize] {
                let (mut a, mut t) = (nums[i] as usize, nums[i]);

                nums[i] = nums[a - 1];
                nums[a - 1] = t;
            }
        }

        for i in 0..len {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32
            }
        }

        return (len + 1) as i32;
    }
}


