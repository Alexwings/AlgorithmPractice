/*
 * @lc app=leetcode.cn id=191 lang=rust
 *
 * [191] 位1的个数
 */

// @lc code=start
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut num = n; 
        let mut count = 0_i32;
        while num != 0 {
            if num & 1 == 1 {
                count += 1;
            }
            num = num >> 1;
        }
        return count;
    }
}
// @lc code=end

