/*
Link: https://leetcode.com/problems/bag-of-tokens/
Problem: 948. Bag of Tokens
*/

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        let (mut tokens, mut power) = (tokens, power);
        
        let (mut i, mut j, mut len) = (0, tokens.len(), tokens.len());
        
        tokens.sort();
        
        while i < j {
            if power >= tokens[i] {
                power -= tokens[i];
                i += 1;
            } else if i - (len - j) != 0 && j > i + 1 {
                j -= 1;
                power += tokens[j];
            } else {
                break;
            }
        }
        
        return (i - (len - j)) as i32;
    }
}