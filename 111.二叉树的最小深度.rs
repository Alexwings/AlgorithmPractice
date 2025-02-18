/*
 * @lc app=leetcode.cn id=111 lang=rust
 *
 * [111] 二叉树的最小深度
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match (root.clone()) {
            (Some(r)) => {
                let lh = Self::min_depth(r.borrow().left.clone());
                let rh = Self::min_depth(r.borrow().right.clone());
                if lh == 0 && rh == 0 {
                    return 1;
                }
                if lh > 0 && rh > 0 {
                    if lh > rh {
                        return rh + 1;
                    } else {
                        return lh + 1;
                    }
                } 
                if lh > 0 {
                    return lh + 1;
                }
                if rh > 0 {
                    return rh + 1;
                }
                return 1;
            }
            None => return 0,
        }
    }
}
// @lc code=end

