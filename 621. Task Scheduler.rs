/*
Link: https://leetcode.com/problems/task-scheduler/
Problem: 621. Task Scheduler
*/

#define MAXVAL(a, b) (((a) > (b)) ? (a) : (b))

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        use std::cmp::max;
        let mut map = vec![0; 26];
        let mut max_count = 0;
        let len = tasks.len() as i32;
        
        for i in 0..tasks.len() {
            let b = (tasks[i] as u8 - b'A') as usize;
            map[b] += 1;
            max_count = max(max_count, map[b]);
        }
        
        let mut ans = (max_count - 1) * (n + 1);
        
        for i in 0..26 {
            if map[i] == max_count {
                ans += 1;
            }
        }
        
        if len > ans {
            return len;
        }
        
        return ans;
    }
}