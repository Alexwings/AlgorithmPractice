/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut s1, mut s2) = (0_usize, 0_usize);
        let (l1, l2) = (m as usize, n as usize);
        let mut res = Vec::<i32>::with_capacity(l1 + l2);
        while s1 < l1 || s2 < l2 {
            if (s1 < l1 && s2 < l2) {
                match nums1[s1].cmp(&nums2[s2]) {
                    std::cmp::Ordering::Greater => {
                        res.push(nums2[s2]);
                        s2 += 1;
                    },
                    std::cmp::Ordering::Less => {
                        res.push(nums1[s1]);
                        s1 += 1;
                    },
                    std::cmp::Ordering::Equal => {
                        res.push(nums1[s1]);
                        s1 += 1;
                    },
                }
            } else if (s1 < l1) {
                res.push(nums1[s1]);
                s1 += 1;
            } else if (s2 < l2) {
                res.push(nums2[s2]);
                s2 += 1;
            }
        }
        for (index, num) in res.iter().enumerate() {
            nums1[index] = *num;
        }
    }
}
// @lc code=end

