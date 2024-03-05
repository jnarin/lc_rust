/*
Link: https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/
Problem: 1750. Minimum Length of String After Deleting Similar Ends
*/
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let (mut l, mut r) = (0, s.len() - 1);
        
        while l < r && s.as_bytes()[l] == s.as_bytes()[r] {
            let c = s.as_bytes()[l];
            
            while l <= r && c == s.as_bytes()[l] {
                l += 1;
            }
            
            while l <= r && c == s.as_bytes()[r] {
                r -= 1;
            }
        }
        
        return (r - l + 1) as i32;
    }
}