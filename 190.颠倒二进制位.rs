/*
 * @lc app=leetcode.cn id=190 lang=rust
 *
 * [190] 颠倒二进制位
 */

// @lc code=start
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut old: u32 = x;
        let mut new: u32 = 0;
        let mut cnt: i8 = 32;
        while cnt > 0 {
            //和1按位与取出old最后一位
            let l = old & 1;
            //new左移一位，和最后一位按位或得出心值赋给new
            new = new << 1;
            new = new | l;
            //old右移一位
            old = old >> 1;
            cnt -= 1;
        }
        return new;
    }
}
// @lc code=end

