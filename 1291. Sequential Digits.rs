/*
Link: https://leetcode.com/problems/sequential-digits/
Problem: 1291. Sequential Digits
*/

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let n_low = Self::count_digits(low);
        let n_high = Self::count_digits(high);
        let v = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        for i in n_low..=n_high {
            for j in 0..(10 - i) {
                let mut t = String::new();
                
                for n in j..(j + i) {
                    t += v[n];
                }
                
                let num: i32 = t.parse::<i32>().unwrap();
                if num >= low && num <= high {
                    ans.push(num);
                }
            }
        }
        
        return ans;
    }
    
    pub fn count_digits(n: i32) -> usize {
        let mut ans = 0;
        let mut n = n;
        
        while n != 0 {
            n /= 10;
            ans += 1;
        }
        
        return ans;
    }
}