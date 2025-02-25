/*
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * [112] 路径总和
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(r) = root {
            match (r.borrow().left.clone(), r.borrow().right.clone()) {
                (Some(l), Some(right)) => {
                    let les = Self::has_path_sum(Some(l), target_sum - r.borrow().val);
                    let res = Self::has_path_sum(Some(right), target_sum - r.borrow().val);
                    return les || res;
                },
                (Some(l), None) => {
                    let les = Self::has_path_sum(Some(l), target_sum - r.borrow().val);
                    return les;
                },
                (None, Some(right))  => {
                    let res = Self::has_path_sum(Some(right), target_sum - r.borrow().val);
                    return res;
                },
                (None, None) => {
                    return target_sum == r.borrow().val;
                }
            }
        } else {
            return false;
        }
    }
}
// @lc code=end

