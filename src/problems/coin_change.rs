/// # Coin Change (Dynamic Programming)
///
/// Given an array of coin denominations and a target `amount`, return the minimum
/// number of coins needed to make up that amount. If it cannot be made up by any
/// combination of coins, return -1.
///
/// You have an unlimited supply of each coin denomination.
///
/// **Expected complexity:** O(amount * coins.len()) time, O(amount) space
///
/// ## Examples
/// - coins = [1, 5, 10, 25], amount = 36 -> 3  (25 + 10 + 1)
/// - coins = [2], amount = 3             -> -1
/// - coins = [1], amount = 0             -> 0

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp : Vec<i32> = Vec::new();
    dp.resize(amount as usize + 1, i32::MAX);
    dp[0] = 0;

    return coin_change_dp(&mut dp, &coins, amount);
}

fn coin_change_dp(dp: &mut Vec<i32>, coins: &Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut min = i32::MAX;

    for coin in coins {
        if *coin > amount {
            continue;
        }

        let mut pre = dp[amount as usize - *coin as usize];
        if pre == i32::MAX {
            coin_change_dp(dp, coins, amount - coin);
            pre = dp[amount as usize - *coin as usize];

            if pre == -1 {
                continue;
            }
        }
        min = min.min(pre);
    }

    dp[amount as usize] = min + 1;

    if min == i32::MAX {
        return -1;
    }

    return min + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_amount() {
        assert_eq!(coin_change(vec![1, 5, 10], 0), 0);
    }

    #[test]
    fn exact_coin() {
        assert_eq!(coin_change(vec![1, 5, 10, 25], 25), 1);
    }

    #[test]
    fn greedy_fails() {
        // Greedy would pick 12 -> 1 (12) + 1 (1) = 2 coins, but optimal is 3 (4+4+4)
        // Actually classic example: coins=[1,3,4], amount=6 -> greedy picks 4+1+1=3, optimal is 3+3=2
        assert_eq!(coin_change(vec![1, 3, 4], 6), 2);
    }

    #[test]
    fn impossible() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }

    #[test]
    fn standard() {
        assert_eq!(coin_change(vec![1, 5, 10, 25], 36), 3);
    }

    #[test]
    fn single_denomination() {
        assert_eq!(coin_change(vec![5], 20), 4);
    }

    #[test]
    fn large_amount() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }
}
