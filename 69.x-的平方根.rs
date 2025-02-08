/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根 
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        //y^2 = x;
        if (x == 1) {
            return 1;
        }
        let (mut left, mut right) = (0, x.min(46340) + 1);
        while right - left > 1 {
            let mid = (right + left) / 2;
            if (mid * mid > x) {
                right = mid;
            }else if (mid * mid < x) {
                left = mid;
            } else {
                return mid;
            }
        }
        return left
    }
}
// @lc code=end

