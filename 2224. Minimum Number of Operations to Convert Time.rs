/*
 * Link: https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/
 * Problem: 2224. Minimum Number of Operations to Convert Time
 * */

impl Solution {
    pub fn convert_stringtime_to_mins(time: String) -> i32 {
        let t: Vec<char> = time.chars().collect();
        let h = ((t[0] as i32 * 10) + t[1] as i32) * 60;
        let m = (t[3] as i32 * 10) + t[4] as i32;
        
        (h + m)
    }
    
    pub fn convert_time(current: String, correct: String) -> i32 {
        let (mut ans, current_time, correct_time) = 
            (0, 
            Self::convert_stringtime_to_mins(current), 
            Self::convert_stringtime_to_mins(correct));
            
        let mut diff = correct_time - current_time;
        
        while diff != 0 {
            if diff >= 60 {
                diff -= 60;
            } else if diff >= 15 {
                diff -= 15;
            } else if diff >= 5 {
                diff -= 5;
            } else {
                diff -= 1;
            }
            
            ans += 1;
        }
        
        ans
    }
}

