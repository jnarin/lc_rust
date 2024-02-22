/*
Link: https://leetcode.com/problems/find-the-town-judge/
Problem: 997. Find the Town Judge
*/

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut in_trust = vec![0; (n + 1) as usize];
        let mut out_trust = vec![0; (n + 1) as usize];
        
        for t in trust {
            out_trust[t[0] as usize] += 1;
            in_trust[t[1] as usize] += 1;
        }
        
        for i in 1..=n {
            let i = i as usize;
            if in_trust[i] == n - 1 && out_trust[i] == 0 {
                return i as i32;
            }
        }
        
        return -1;
    }
}