/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }
        if haystack.len() == needle.len() {
            return match (haystack == needle) {
                (true) => 0_i32,
                (false) => -1_i32,
            };
        }
        let mut hay_vec = haystack.chars().collect::<Vec<char>>();
        let mut needle_vec = needle.chars().collect::<Vec<char>>();
        let mut s = 0_usize;
        while (s <= hay_vec.len() - needle_vec.len()) {
            let mut i = 0;
            while (i < needle_vec.len() && hay_vec[s + i] == needle_vec[i]) {
                i += 1;
            }
            if (i >= needle_vec.len()) {
                return s.try_into().unwrap();
            } else {
                s += 1;
            }
        }
        return -1;
    }
}
// @lc code=end
