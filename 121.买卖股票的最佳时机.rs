/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if (prices.is_empty()) {
            return 0;
        }
        let mut max_profit = 0_i32;
        let mut min_price = prices[0];
        for p in prices {
            min_price = min_price.min(p);
            max_profit = max_profit.max(p - min_price);
        }
        return max_profit;
    }
}
// @lc code=end

