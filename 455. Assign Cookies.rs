/*
 * Link: https://leetcode.com/problems/assign-cookies/
 * Problem: 455. Assign Cookies
 * */

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        
        g.sort();
        s.sort();
        
        let mut cookieIndex = 0;
        let mut contentChildren = 0;
        
        while cookieIndex < s.len() && contentChildren < g.len() {
            if s[cookieIndex] >= g[contentChildren] {
                contentChildren += 1;
            }
            
            cookieIndex += 1;
        }
        
        return contentChildren as i32;        
    }
}

