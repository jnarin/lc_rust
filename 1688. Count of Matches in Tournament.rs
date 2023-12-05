/*
Link: https://leetcode.com/problems/count-of-matches-in-tournament/
Problem: 1688. Count of Matches in Tournament
*/

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let (mut ans, mut t): (i32, i32) = (0, n);
        
        while (true){
            if t <= 1 {
                break;
            }
			
			ans += (t >> 1);
            
            if t & 1 == 1 {
                t = (t + 1) >> 1;
            } else {
                t >>= 1;
            }
        }
        
        return ans;
    }
}