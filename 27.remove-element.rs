/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() <= 0 { return 0;}
        let mut i = nums.len() - 1;
        let mut count = 0;
        let mut s = 0_usize;
        while i >= 0 && s <= i {
            if (nums[s] == val) {
                Solution::swap(nums, &s, &i);
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            } else {
                count += 1;
                s += 1;
            }
        }
        return count as i32;
    }
    fn swap(nums: &mut Vec<i32>, i: &usize, j: &usize) {
        let tmp = nums[*i];
        nums[*i] = nums[*j];
        nums[*j] = tmp;
    }
}
// @lc code=end

