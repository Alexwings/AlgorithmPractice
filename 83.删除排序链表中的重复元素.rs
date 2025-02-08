/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();
        while let Some(node) = current {
        // 内层循环处理连续重复元素
        while let Some(next) = node.next.as_mut() {
            if next.val == node.val {
                // 跳过重复节点
                node.next = next.next.take();
            } else {
                break;
            }
        }
        // 移动到下一个节点继续检查
        current = node.next.as_mut();
    }
        head
    }
}
// @lc code=end

