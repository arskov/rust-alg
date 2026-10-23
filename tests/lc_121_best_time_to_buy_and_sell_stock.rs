struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut max_num = 0;

        for &n in prices.iter().rev() {
            let profit = max_num - n;
            max_profit = max_profit.max(profit);
            max_num = max_num.max(n);
        }

        max_profit
    }
}
