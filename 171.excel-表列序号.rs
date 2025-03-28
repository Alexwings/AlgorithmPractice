/*
 * @lc app=leetcode.cn id=171 lang=rust
 *
 * [171] Excel 表列序号
 */

// @lc code=start
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let chars: Vec<_> = column_title.chars().collect();
        let len = chars.len();
        let mut res = 0_i32;
        for (index, c) in chars.iter().enumerate() {
            let item = *c as i32;
            let num = item % 64;
            res = res * 26 + num;
        }
        return res;
    }
}
// @lc code=end

