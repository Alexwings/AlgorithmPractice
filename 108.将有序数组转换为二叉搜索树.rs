/*
 * @lc app=leetcode.cn id=108 lang=rust
 *
 * [108] 将有序数组转换为二叉搜索树
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::sorted_array_to_bst_tree(nums.clone(), 0, nums.len());
    }

    fn sorted_array_to_bst_tree(nums: Vec<i32>, s: usize, e: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if e - s <= 0 {
            return None;
        }
        if e - s == 1 && s < nums.len() {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[s]))));
        }
        let mid = (s + e) / 2;
        if mid < nums.len() {
            let mut current = TreeNode::new(nums[mid]);
            let left = Self::sorted_array_to_bst_tree(nums.clone(), s, mid);
            let right = Self::sorted_array_to_bst_tree(nums.clone(), mid + 1, e);
            current.left = left.clone();
            current.right = right.clone();
            return Some(Rc::new(RefCell::new(current)));
        }
        return None;
    }
}
// @lc code=end

