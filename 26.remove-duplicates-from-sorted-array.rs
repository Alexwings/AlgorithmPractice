/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if (nums.len() <= 1) {
            return nums.len() as i32;
        }
        let mut d_i = 1_usize;
        let mut i = 0_usize;
        while i < nums.len() && d_i < nums.len() && nums[i] <= nums[d_i] {
            let c = &nums[i];
            let c_n = &nums[d_i];
            if c == c_n {
                while d_i < nums.len() && nums[d_i] == *c {
                    d_i += 1;
                }
                let len = d_i - i - 1;
                for a in d_i..nums.len() {
                    nums[a - len] = nums[a];
                }
            }
            i += 1;
            d_i = i + 1;
        }
        i = 0;
        d_i = 1;
        while d_i < nums.len() && nums[i] < nums[d_i] {
            i += 1;
            d_i = i + 1;
        }

        return d_i as i32;
    }
}
// @lc code=end

