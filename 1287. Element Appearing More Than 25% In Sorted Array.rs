/*
Link: https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/
Problem: 1287. Element Appearing More Than 25% In Sorted Array
*/
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let t = arr.len() / 4;
        
        for i in t..arr.len() {
            if arr[i - t] == arr[i] {
                return arr[i];
            }
        }
        
        return i32::MIN;
        
    }
}