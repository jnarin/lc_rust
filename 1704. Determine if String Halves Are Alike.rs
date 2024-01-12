/*
Link: https://leetcode.com/problems/determine-if-string-halves-are-alike/
Problem: 1704. Determine if String Halves Are Alike
*/

impl Solution {
    pub fn isVowel(c: u8) -> bool {
        let vowels: Vec<u8> = vec![b'a', b'A', b'e', b'E', b'i', b'I', b'o', b'O', b'u', b'U'];
        
        for v in vowels {
            if c == v {
                return true;
            }
        }
        
        return false;
    }
    
    pub fn halves_are_alike(s: String) -> bool {
        let len = s.len();
        let (mut i, mut j) = (0, len >> 1);
        let (mut a, mut b) = (0, 0);
        
        while (i < j && j < len) {
            if Self::isVowel(s.as_bytes()[i]) == true {
                a += 1;
            }
            
            if Self::isVowel(s.as_bytes()[j]) == true {
                b += 1;
            }
            
            i += 1;
            j += 1;
        }
        
        return a == b;
    }
}