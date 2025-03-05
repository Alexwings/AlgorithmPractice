/*
 * @lc app=leetcode.cn id=141 lang=swift
 *
 * [141] 环形链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public var val: Int
 *     public var next: ListNode?
 *     public init(_ val: Int) {
 *         self.val = val
 *         self.next = nil
 *     }
 * }
 */

class Solution {
    func hasCycle(_ head: ListNode?) -> Bool {
        guard let h = head else {
            return false;
        }
        var slow: ListNode? = h;
        var fast: ListNode? = h;
        while let s = slow {
            fast = fast?.next
            if let f = fast {
                if s === f {
                    return true;
                }
                fast = f.next;
            }
            slow = s.next;
        }
        return false;
    }
}
// @lc code=end

