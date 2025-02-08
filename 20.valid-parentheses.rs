/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::with_capacity(s.chars().count());
        for c in s.chars() {
            match c {
                '{' => stack.push(c),
                '(' => stack.push(c),
                '[' => stack.push(c),
                _ => {
                    if let Some(last) = stack.pop() {
                        if !Solution::match_char(&last, &c) {
                            return false;
                        }
                    } else {
                        stack.push(c)
                    }
                }
            }
        }
        return stack.is_empty();
    }
    fn match_char(top: &char, target:&char) -> bool {
        match top {
            '{' => return *target == '}',
            '[' => return *target == ']',
            '(' => return *target == ')',
            _ => return false,
        }
    }
}
// @lc code=end

