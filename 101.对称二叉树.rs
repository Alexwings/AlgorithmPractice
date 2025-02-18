/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            return Self::is_symmetric_tree(r.borrow().left.clone(), r.borrow().right.clone());
        }
        return true;
    }

    fn is_symmetric_tree(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(l), Some(r)) => {
                if l.borrow().val != r.borrow().val {
                    return false;
                }
                return Self::is_symmetric_tree(l.borrow().left.clone(), r.borrow().right.clone()) && Self::is_symmetric_tree(l.borrow().right.clone(), r.borrow().left.clone())
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}
// @lc code=end

