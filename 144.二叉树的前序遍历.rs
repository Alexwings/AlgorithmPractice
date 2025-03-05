/*
 * @lc app=leetcode.cn id=144 lang=rust
 *
 * [144] 二叉树的前序遍历
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // return Self::preorder_traversal_1(root);
        return Self::preorder_traversal_2(root);
    }
    pub fn preorder_traversal_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(r) = root {
            match (r.borrow().left.clone(), r.borrow().right.clone()) {
                (Some(left), Some(right)) => {
                    res.push(r.borrow().val);
                    res.append(&mut Self::preorder_traversal(Some(left)));
                    res.append(&mut Self::preorder_traversal(Some(right)));
                },
                (Some(left), None) => {
                    res.push(r.borrow().val);
                    res.append(&mut Self::preorder_traversal(Some(left)));
                },
                (None, Some(right)) => {
                    res.push(r.borrow().val);
                    res.append(&mut Self::preorder_traversal(Some(right)));
                },
                _ => {
                    res.push(r.borrow().val);
                }
            }
        }
        return res;
    }

    pub fn preorder_traversal_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(r) = root {
            let mut stack = vec![r.clone()];
            while !stack.is_empty() {
                if let Some(cur) = stack.pop() {
                    res.push(cur.borrow().val);
                    if let Some(right) = cur.borrow().right.clone() {
                        stack.push(right);
                    }
                    if let Some(left) = cur.borrow().left.clone() {
                        stack.push(left);
                    }
                }
            }
        }
        return res;
    }
}
// @lc code=end

