/*
Link: https://leetcode.com/problems/number-of-laser-beams-in-a-bank/
Problem: 2125. Number of Laser Beams in a Bank
*/

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut ans: i32 = 0;
        let (mut prevRow, mut currRow) = (0, 0);
        
        for r in bank {
            for c in r.chars() {
                if c == '1' {
                    currRow += 1;
                }
            }
            
            ans += currRow * prevRow;
            
            if currRow != 0 {
                prevRow = currRow;
            }
            
            currRow = 0;
        }
        
        return ans;
    }
}