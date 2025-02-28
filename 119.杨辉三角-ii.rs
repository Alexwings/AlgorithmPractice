/*
 * @lc app=leetcode.cn id=119 lang=rust
 *
 * [119] 杨辉三角 II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let vecs = Self::get_vec(row_index + 1);
        return vecs[row_index as usize].clone();
    }
    fn get_vec(num: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::<Vec<i32>>::with_capacity(num as usize);
        for i in 0..(num as usize) {
            let mut cur_vec = Vec::<i32>::with_capacity(i + 1);
            for j in 0..=i {
                let mut cur = 0_i32;
                let mut prev = 0_i32;
                match j {
                    0 => cur = 1,
                    _ if j == i => prev = 1,
                    _ => {
                        let prev_vec = &res[i - 1];
                        cur = prev_vec[j];
                        prev = prev_vec[j - 1];
                    }
                }
                cur_vec.push(cur + prev);
            }
            res.push(cur_vec);
        }
        return res;
    }
}
// @lc code=end

