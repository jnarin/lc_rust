/*
Link: https://leetcode.com/problems/1-bit-and-2-bit-characters/
Problem: 717. 1-bit and 2-bit Characters
*/

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let len = bits.len();
        let mut i = 0;
        
        while i < len {
            if i == len - 1 && bits[i] == 0 {
                return true;
            }
            
            if bits[i] == 0 {
                i += 1;
            } else {
                i += 2;
            }
        }
        
        return false;        
    }
}