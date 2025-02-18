/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

// @lc code=start
fn u8_to_char(num: u8) -> char {
    match num {
        0 => return '0',
        n => return (n + ('0' as u8)) as char,
    }
}
fn digit_char_to_u8(digit: char) -> u8 {
    return (digit as u8) - ('0' as u8);
}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut m_long: Vec<char> = a.chars().collect();
        let mut m_short: Vec<char> = b.chars().collect();
        if (m_long.len() < m_short.len()) {
            let temp = m_long;
            m_long = m_short;
            m_short = temp;
        }
        let mut il = m_long.len() - 1;
        let mut is = m_short.len() - 1;
        let mut c: u8 = 0;
        let mut res: Vec<char> = vec![];
        while (il >= 0 && il < m_long.len()) || (is >= 0 && is < m_short.len()) {
            if (il >= 0 && il < m_long.len() && is >= 0 && is < m_short.len()) {
                let cur_l = digit_char_to_u8(m_long[il]);
                let cur_s = digit_char_to_u8(m_short[is]);
                let cur_res = (cur_l + cur_s + c) % 2;
                c = (cur_l + cur_s + c) / 2;
                res.push(u8_to_char(cur_res));
                il -= 1;
                is -= 1;
            } else if (il >= 0 && il < m_long.len()) {
                let cur_l = digit_char_to_u8(m_long[il]);
                let cur_res = (cur_l + c) % 2;
                c = (cur_l + c) / 2;
                res.push(u8_to_char(cur_res));
                il -= 1;
            } else if (is >= 0 && is < m_short.len()) {
                let cur_s = digit_char_to_u8(m_short[is]);
                let cur_res = (c + cur_s) % 2;
                c = (cur_s + c) / 2;
                res.push(u8_to_char(cur_res));
                is -= 1;
            }
        }
        if (c > 0) {
            res.push(u8_to_char(c));
        }
        res.reverse();
        return res.iter().collect();
    }
}
// @lc code=end
