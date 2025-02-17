/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
 */

// @lc code=start
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

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut result: Vec<i32> = vec![];
        let mut cur = root.clone();
        while cur.is_some() || !queue.is_empty() {
            //左子树放入queue
            while let Some(left) = cur {
                queue.push(left.clone());
                cur = left.borrow().left.clone();
            }
            if let Some(node) = queue.pop() {
                let node_ref = node.borrow();
                result.push(node_ref.val);
                if let Some(right) = &node_ref.right {
                    cur = node_ref.right.clone();
                }
            }
        }
        return result;
    }
}
// @lc code=end
