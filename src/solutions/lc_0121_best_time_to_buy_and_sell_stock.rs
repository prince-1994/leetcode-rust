use std::cmp::max;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut pos, mut profit) = (-prices[0], 0);

    for i in 1..prices.len() {
        profit = max(profit, pos + prices[i]);
        pos = max(pos, -prices[i]);
    }
    return profit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn main() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
