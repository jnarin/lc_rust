/*
Link: https://leetcode.com/problems/largest-divisible-subset/
Problem: 368. Largest Divisible Subset
*/

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = Vec::new();
        let mut lis = vec![1; len + 1];
        let mut nums = nums;
        
        nums.sort();
        
        let mut n = 1;
        for i in 1..len {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && lis[i] < (lis[j] + 1) {
                    lis[i] = lis[j] + 1;
                    
                    if n < lis[i] {
                        n = lis[i];
                    }
                }
            }
        }
        
        let mut p = -1;
        for i in (0..len).rev() {
            if lis[i] == n && (p == -1 || p % nums[i] == 0) {
                ans.push(nums[i]);
                n -= 1;
                p = nums[i];
            }
        }
        
        return ans;
    }
}