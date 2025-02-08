/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();
        for (index, num) in nums.iter().enumerate() {
            let remain = target - num;
            if let Some(&peerIndex) = num_map.get(&num) {
                result.push(peerIndex);
                result.push(index as i32);
            } else {
                num_map.insert(remain, index as i32);
            }
        }
        return result;
    }
}
// @lc code=end

