/*
Link: https://leetcode.com/problems/buy-two-chocolates/
Problem: 2706. Buy Two Chocolates
*/
impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut tmpPrices = prices;
        
        tmpPrices.sort();
        
        let mut t = tmpPrices[0] + tmpPrices[1];
        
        if (t <= money) {
            return money - t;
        }
        
        return money;        
    }
}
