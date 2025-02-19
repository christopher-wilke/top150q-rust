pub struct Solution;

impl Solution {

    pub fn find_buy_pos(prices: &Vec<i32>) -> usize {
        let mut lowest_pos = 0;
        for (pos, val) in prices
            .iter()
            .enumerate()
        {
                if val < &prices[lowest_pos] {
                    lowest_pos = pos;
                } 
        }
        lowest_pos
    }

    pub fn get_possible_sell_values(buy_value: i32, prices: Vec<i32>) -> Vec<i32> {
        prices
            .into_iter()
            .filter(|x| x > &&buy_value)
            .collect()
    }
  
  
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let buy_pos = Self::find_buy_pos(&prices);
        let new_prices = prices[buy_pos+1..prices.len()].to_vec();
        let possible_sell_values = Self::get_possible_sell_values(
            prices[buy_pos],
            new_prices
        );
        let current_profit = possible_sell_values[0] - prices[buy_pos];
        println!("buy value={}", prices[buy_pos]);
        println!("possible_sell_values: {:?}", possible_sell_values);
        println!("current profit: {}", current_profit);
        100
    }
}

fn main() {
    let prices = [7,1,5,3,6,4].to_vec();
    println!("{:?}", prices);
    let max_profit = Solution::max_profit(prices);
    println!("{}", max_profit);
}
