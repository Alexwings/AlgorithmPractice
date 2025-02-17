/*
 * @lc app=leetcode.cn id=100 lang=rust
 *
 * [100] 相同的树
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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack_p: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut stack_q: Vec<Rc<RefCell<TreeNode>>> = vec![];
        if let Some(hp) = p {
            if let Some(hq) = q {
                stack_p.push(hp.clone());
                stack_q.push(hq.clone());
                while !stack_p.is_empty() && !stack_q.is_empty() {
                    if let Some(cur_p) = stack_p.pop() {
                        if let Some(cur_q) = stack_q.pop() {
                            if (cur_p.borrow().val != cur_q.borrow().val) {
                                return false
                            }
                            if let Some(right) = cur_p.borrow().right.as_ref() {
                                if let Some(right_q) = cur_q.borrow().right.as_ref() {
                                    stack_q.push(right_q.clone())
                                } else {
                                    //一个有右子树，一个没有，返回false
                                    return false
                                }
                                stack_p.push(right.clone());
                            } else if let Some(right) = cur_q.borrow().right.as_ref() {
                                //一个有右子树，一个没有，返回false
                                return false
                            }
                            if let Some(left) = cur_p.borrow().left.as_ref() {
                                if let Some(left_q) = cur_q.borrow().left.as_ref() {
                                    stack_q.push(left_q.clone())
                                } else {
                                    //一个有左子树，一个没有，返回false
                                    return false
                                }
                                stack_p.push(left.clone());
                            } else if let Some(left) = cur_q.borrow().left.as_ref() {
                                //一个有左子树，一个没有，返回false
                                return false
                            }
                        } else {
                            return false
                        }
                    } else {
                        return false
                    }
                }
                return (stack_p.is_empty() && stack_q.is_empty())
            } else {
                return false;
            }
        } 
        return q == None;
    }
}
// @lc code=end

