/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let n: f32 = ((nums.len() as f32) / 2.0);
        let mut dict: HashMap<i32, f32> = HashMap::new();
        for &integer in &nums {
            dict.entry(integer).and_modify(|i| *i += 1_f32).or_insert(1_f32);
        }
        for key in dict.keys() {
            if let Some(i) = dict.get(key) {
                if i >= &n {
                    return *key;
                }
            }
        }
        return 0;
    }
}
// @lc code=end

