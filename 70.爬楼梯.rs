/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if (n < 2) {
            return 1;
        }
        if (n == 2) {
            return 2;
        }
        let mut prev = 1_i32;
        let mut res = 2_i32;
        for x in 3..(n+1) {
            let p = res;
            res = res + prev;
            prev = p;
        }
        return res;
    }
}
// @lc code=end

