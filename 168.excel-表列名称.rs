/*
 * @lc app=leetcode.cn id=168 lang=rust
 *
 * [168] Excel 表列名称
 */

// @lc code=start
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut col_num = column_number;
        let mut res = String::new();
        let const_char = 'A' as i32;
        while col_num > 0 {
            let remainder = (col_num - 1) % 26;
            let cur_char = char::from_u32((const_char + remainder) as u32).unwrap(); 
            res.insert(0, cur_char);
            col_num = (col_num - 1) / 26;
        }
        return res;
    }
}
// @lc code=end

