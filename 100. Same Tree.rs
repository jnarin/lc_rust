/*
Link: https://leetcode.com/problems/same-tree/
Problem: 100. Same Tree
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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        
        if p.is_none() || q.is_none() {
            return false;
        }
        
        if p.as_ref().unwrap().borrow().val != q.as_ref().unwrap().borrow().val {
            return false;
        }
        
        return Self::is_same_tree(p.as_ref().unwrap().borrow().left.clone(), q.as_ref().unwrap().borrow().left.clone()) &&
            Self::is_same_tree(p.as_ref().unwrap().borrow().right.clone(), q.as_ref().unwrap().borrow().right.clone());
    }
}