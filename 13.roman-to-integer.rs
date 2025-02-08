/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
use std::collections::HashMap;
const R_TO_I_VALUE: [(&'static str, i32); 7] = [
    ("I", 1), 
    ("V", 5), 
    ("X", 10), 
    ("L", 50), 
    ("C", 100),
    ("D", 500),
    ("M", 1000)
]; 
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let r_to_i: HashMap<&'static str, i32> = R_TO_I_VALUE.into_iter().collect();
        let chars_iter = s.chars();
        let mut result = 0_i32;
        let mut prev: i32 = 0_i32;
        for char in chars_iter {
            let st: &str = &format!("{}", char);
            if let Some(&num) = r_to_i.get(st) {
                if (num > prev) {
                    result = result + num - 2 * prev;
                } else {
                    result = result + num;
                }
                prev = num;
            }
        }
        return result;
    }
}
// @lc code=end

