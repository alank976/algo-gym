#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            0
        } else {
            let mut buy_at = prices[0];
            let mut sell_at = 0;
            let mut profit = 0;
            
            for p in prices {
                if p <= buy_at {
                    buy_at = p;
                    sell_at = 0;
                } else if p > sell_at {
                    sell_at = p;
                    let new_profit = sell_at - buy_at;
                    if new_profit > profit {
                        profit = new_profit;
                    }
                }
            }
            profit
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(2, Solution::max_profit(vec![1, 2, 3]));
        // assert_eq!(0, Solution::max_profit(vec![7, 6, 5]));
        assert_eq!(6, Solution::max_profit(vec![11, 7, 6, 5, 8, 4, 10]));
        assert_eq!(0, Solution::max_profit(vec![]));
        assert_eq!(2, Solution::max_profit(vec![2, 4, 1]));
    }
}
