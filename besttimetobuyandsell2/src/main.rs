pub struct Solution;

impl Solution {
 
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        if prices.is_empty() {
            return 0;
        }

        let mut max_profit = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i-1] {
                max_profit += prices[i] - prices[i-1];
            }
        }

        max_profit
    }

    pub fn max_profit_recursive(prices: &Vec<i32>, index: usize) -> i32 {
        if index >= prices.len() - 1 {
            return 0;
        }

        let mut profit = 0;

        if prices[index+1] > prices[index] {
            profit = prices[index+1] - prices[index] + Self::max_profit_recursive(prices, index+1);
        } else {
            profit = Self::max_profit_recursive(prices, index+1);
        }

        profit
        
    }

}

fn main() {
    let prices = [7, 1, 5, 3, 6, 4].to_vec();
    println!("{:?}", prices);
    let max_profit = Solution::max_profit(prices);
    println!("{max_profit}");
}
