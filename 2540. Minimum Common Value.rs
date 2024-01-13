/*
 * Link: https://leetcode.com/problems/minimum-common-value/
 * Problem: 2540. Minimum Common Value
 * */

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);
        
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                return nums1[i];
            }
            
            if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        
        return -1;
    }
}
