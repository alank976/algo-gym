// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        // update lowerest price (as buy_at) and find profittable sell during iteration
        let mut buy_at = prices[0];
        let mut max_profit = 0;

        for price in prices {
            match price - buy_at {
                profit if profit > max_profit => max_profit = profit,
                profit if profit < 0 => buy_at = price,
                _ => (),
            };
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        for (expected, inputs) in vec![
            (2, vec![1, 2, 3]),
            (0, vec![7, 6, 5]),
            (6, vec![11, 7, 6, 5, 8, 4, 10]),
            (0, vec![]),
            (2, vec![2, 4, 1]),
        ] {
            assert_eq!(expected, Solution::max_profit(inputs));
        }
    }
}
