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
use std::collections::VecDeque;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let new_root = Self::flip_a_tree(root.clone());
        return Self::two_trees_are_equal(new_root, root);
    }

    fn flip_a_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
        if let Some(mut h) = root {
            let temp: Option<Rc<RefCell<TreeNode>>> = Self::flip_a_tree(h.borrow().left.clone());
            let right = Self::flip_a_tree(h.borrow().right.clone());
            h.borrow_mut().left = right;
            h.borrow_mut().right = temp;
            return Some(h);
        }
        return None;
    }

    fn two_trees_are_equal(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue_p = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        let mut result: Vec<i32> = vec![]
        if let Some(hp) = p.clone() {
            queue_p.push_back(hp);
            while !queue_p.is_empty() {
                if let Some(&cur) = queue_p.pop_front() {
                    result.push(cur.val);
                    if let Some(left_p) = cur.borrow().left {
                        queue_p.push_back(left_p.clone());
                    }
                    if let Some(right_p) = cur.borrow().right {
                        queue_p.push_back(right_p.clone());
                    }
                }
            }
        }
    }
}
// @lc code=end

