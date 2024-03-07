/*
Link: https://leetcode.com/problems/middle-of-the-linked-list/
Problem: 876. Middle of the Linked List
*/

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut len = 0;
        let mut n = &head;
        
        while let Some(node) = n.as_deref() {
            n = &node.next;
            len += 1;
        }
        
        len /= 2;
        for _ in 0..len {
            head = head.and_then(|head| head.next);
        }
        
        return head;
    }
}