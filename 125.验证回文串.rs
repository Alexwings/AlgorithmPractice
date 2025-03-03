/*
 * @lc app=leetcode.cn id=125 lang=rust
 *
 * [125] 验证回文串
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<_> = s.chars().filter(|c| {
            return c.is_alphabetic() || c.is_digit(10);
        }).collect();
        if (chars.len() <= 0)  {
            return true;
        }
        let mut b = 0_usize;
        let mut e = chars.len() - 1;
        while b < e {
            if (chars[b].to_ascii_lowercase() != chars[e].to_ascii_lowercase()) {
                return false;
            }
            b += 1;
            e -= 1;
        }
        return true;
    }
}
// @lc code=end

