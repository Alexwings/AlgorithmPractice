/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x == 0 { return true; }
        if x < 0 { return false;}
        let mut le = 0;
        let mut mut_x = x;
        while mut_x != 0 {
            mut_x = mut_x / 10_i32;
            le += 1;
        }
        le -= 1;
        let mut re = 1;
        mut_x = x;
        let mut mut_x_r = x;
        let mut result = true;
        while le >= re {
            if mut_x / 10_i32.pow(le) != mut_x_r % 10_i32.pow(re) {
                println!("le: {}++++re:{}", le, re);
                println!("l: {}----r:{}", mut_x / 10_i32.pow(le), mut_x_r % 10_i32.pow(re));
                result = false;
                break;
            } else {
                result = true;
                mut_x = mut_x % 10_i32.pow(le);
                mut_x_r = mut_x_r / 10_i32.pow(re);
                le -= 1;
            }
        }
        return result
    }
}
// @lc code=end

