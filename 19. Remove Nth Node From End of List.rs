/*
 * Link: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
 * Problem: 19. Remove Nth Node From End of List
 * */

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut a = head.clone();
        let mut b = &mut head;

        for _ in 0..n {
            a = a.map(|n| n.next)?;
        }

        while let Some(n) = a {
            a = n.next;
            b = &mut b.as_mut()?.next;
        }

        *b = b.as_mut().and_then(|n| n.next.clone());

        return head;
    }
}
