/*
 * @lc app=leetcode.cn id=110 lang=rust
 *
 * [110] 平衡二叉树
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root.clone()) {
            (Some(r)) => {
                let lh = Self::tree_height(r.borrow().left.clone());
                let rh = Self::tree_height(r.borrow().right.clone());
                let cur = (lh - rh <= 1 && lh - rh >= -1);
                return cur && 
                    Self::is_balanced(r.borrow().left.clone()) &&
                    Self::is_balanced(r.borrow().right.clone());
            }
            None => return true,
        }
    }

    fn tree_height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match(node) {
            (Some(n)) => {
                let lh = Self::tree_height(n.borrow().left.clone()) + 1;
                let rh = Self::tree_height(n.borrow().right.clone()) + 1;
                if (lh > rh) {
                    return lh;
                } else {
                    return rh;
                }
            }
            None => return 0,
        }
    }
}
// @lc code=end

