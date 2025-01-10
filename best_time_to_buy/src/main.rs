/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
/// Easy
/// Dynamic Programming

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut profits: Vec<i32> = vec![0; prices.len()];
    let mut cheapest = prices[0];

    for i in 0..prices.len() {
        if prices[i] < cheapest {
            cheapest = prices[i];
        }

        if prices[i] - cheapest <= 0 {
            continue;
        }

        profits[i] = prices[i] - cheapest;
    }

    let max_profit = {

        if profits.is_empty() { return 0 }

        let mut temp = profits[0];
        for val in &profits {
            temp = temp.max(*val);
        }

        temp
    };

    // println!("{:?}", profit);
    max_profit
}

fn main() {
    println!("Hello, world!");
    max_profit(vec![7, 1, 5, 3, 6, 4]);
}
