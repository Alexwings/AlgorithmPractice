/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut lastLength = 0_i32;
        let mut length = 0_i32;
        for c in s.chars() {
            if (c != ' ') {
                length += 1;
            } else if (length != 0) {
                lastLength = length;
                length = 0;
            }
        }
        if (length != 0) {
            return length;
        }
        return lastLength;
    }
}
// @lc code=end
