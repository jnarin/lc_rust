/*
Link: https://leetcode.com/problems/maximum-odd-binary-number/
Problem: 2864. Maximum Odd Binary Number
*/

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut oneCount = 0;
        
        for c in s.chars() {
            if (c == '1') {
                oneCount += 1;
            }
        }
                
        let mut ans = vec!['0'; s.len()];
        let mut i = 0;
        
        ans[s.len() - 1] = '1';
        oneCount -= 1;

        while oneCount > 0 && i < s.len() {
            ans[i] = '1';
            i += 1;
            oneCount -= 1;
        }
        
        return ans.iter().collect();
    }
}