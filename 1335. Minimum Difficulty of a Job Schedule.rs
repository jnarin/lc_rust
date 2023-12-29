/*
 * Link: https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/
 * Problem: 1335. Minimum Difficulty of a Job Schedule
 * */

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let mut ans = -1;

        if d > job_difficulty.len() as i32 {
            return ans;
        }

        let mut dp = vec![vec![i32::MAX; job_difficulty.len() + 1]; (d + 1) as usize];
        dp[0][0] = 0;

        for day in 1..=d as usize {
            for task in day..=job_difficulty.len() {
                let mut lMax = job_difficulty[task - 1];

                for s in (day..=task).rev() {
                    lMax = std::cmp::max(job_difficulty[s - 1], lMax);

                    if dp[day - 1][s - 1] != i32::MAX {
                        dp[day][task] = std::cmp::min(dp[day][task], dp[day - 1][s - 1] + lMax);
                    }
                }
            }
        }

        return dp[d as usize][job_difficulty.len()];
    }
}

