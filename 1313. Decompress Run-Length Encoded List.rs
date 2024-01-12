/*
 * Link: https://leetcode.com/problems/decompress-run-length-encoded-list/
 * Problem: 1313. Decompress Run-Length Encoded List
 * */

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let n = nums.len();
        let mut ans = Vec::new();

        while (i < n) {
            let mut j = nums[i];
            while (j > 0) {
                ans.push(nums[i + 1]);
                j -= 1;
            }

            i += 2;
        }

        return ans;

    }
}

