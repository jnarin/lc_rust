/*
Link: https://leetcode.com/problems/range-sum-of-bst/
Problem: 938. Range Sum of BST
*/

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut ans = 0;
        
        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            
            if val >= low && val <= high {
                ans += val;
            }
            
            ans += Self::range_sum_bst(node.left.clone(), low, high) + Self::range_sum_bst(node.right.clone(), low, high);
        }
        
        return ans;        
    }
}