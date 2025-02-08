/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let cap = Self::find_max(&strs);
        let mut common_str = String::from(Self::find_shrotest(&strs));
        let mut index = 0;
        while common_str.len() > 0 && index < strs.len() {
            if !strs[index].starts_with(&common_str) {
                common_str.pop();
                index = 0;
            } else {
                index += 1;
            }
        }
        return common_str;
    }

    fn find_max(strs: &Vec<String>) -> usize {
        let local_strs = strs.clone();
        let mut size: usize = 0;
        for str in local_strs {
            if str.len() > size {
                size = str.len();
            }
        }
        return size;
    }
    fn find_shrotest(strs: &Vec<String>) -> String {
        let local_strs = strs.clone();
        if local_strs.is_empty() {
            return "".to_string();
        }
        let mut min = local_strs[0].clone();
        for item in local_strs {
            if (item.len() < min.len()) {
                min = item;
            }
        }
        return min;
    }
}
// @lc code=end

