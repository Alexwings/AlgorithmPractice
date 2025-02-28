/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::<Vec<i32>>::with_capacity(num_rows as usize);
        res.push(vec![1]);
        for i in 1_usize..(num_rows as usize) {
            let prev_vec = &res[i - 1_usize];
            let mut cur_vec = Vec::<i32>::with_capacity(i + 1_usize);
            for k in 0_usize..=i {
                let mut prev = 0_i32;
                let mut cur = 0_i32;
                match k {
                    0 => cur = 1,
                    _ if k == i => prev = 1,
                    _ => {
                        prev = prev_vec[k - 1];
                        cur = prev_vec[k];
                    }
                }
                cur_vec.push(prev + cur);
            }
            res.push(cur_vec)
        }
        return res;
    }
}
// @lc code=end

