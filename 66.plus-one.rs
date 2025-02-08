/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1_i32;
        let mut i = digits.len() - 1;
        let mut m_digits = digits.to_vec();
        while i >= 0 && i < digits.len() {
            m_digits[i] += carry;
            carry = m_digits[i] / 10;
            m_digits[i] = m_digits[i] % 10;
            i -= 1;
            println!("{}", i)
        }
        if (carry > 0) {
            m_digits.insert(0, carry);
        }
        return m_digits;
    }
}
// @lc code=end
