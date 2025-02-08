/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

// @lc code=start
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(-1));
        let mut p = &mut head; 
        let mut l1 = &list1;
        let mut l2 = &list2;
        while (*l1).is_some() || (*l2).is_some() {
            match (l1, l2) {
                (&Some(ref l), &Some(ref r)) => {
                    if (l.val <= r.val) {
                        p.next = l1.clone();
                        l1 = &l.next;
                    } else {
                        p.next = l2.clone();
                        l2 = &r.next;
                    }
                },
                (&Some(ref l), &None) => {
                    p.next = l1.clone();
                    l1 = &l.next;
                },
                (&None, &Some(ref r)) => {
                    p.next = l2.clone();
                    l2 = &r.next;
                },
                _ => ()
            }
            if let Some(ref mut next) = p.next {
                p = next;
            }
        }
        return head.next;
    }
}
// @lc code=end

