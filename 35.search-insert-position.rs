/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0_i32, nums.len() as i32 - 1);
        while (low <= high) {
            let mut mid = (low + high) / 2;
            match (nums[mid as usize].cmp(&target)) {
                Ordering::Equal => {
                    return mid;
                }
                Ordering::Greater => {
                    high = mid - 1;
                }
                Ordering::Less => {
                    low = mid + 1;
                }
            }
        }
        return low;
    }
}
// @lc code=end
