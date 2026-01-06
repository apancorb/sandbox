use std::collections::HashMap;

/// Climbing Stairs
///
/// Determine the number of distinct ways to climb a staircase of n steps by taking either 1 or
/// 2 steps at a time.
///
/// # Example
///
/// ```text
/// Input: n = 4
///
/// Output: 5
///
/// Explanation: The 5 ways are:
/// - 1 + 1 + 1 + 1
/// - 1 + 1 + 2
/// - 1 + 2 + 1
/// - 2 + 1 + 1
/// - 2 + 2
/// ```
pub fn climbing_stairs(n: usize) -> usize {
    fn climbing_stairs_helper(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        }

        if let Some(&val) = memo.get(&n) {
            return val;
        }

        let res = climbing_stairs_helper(n - 1, memo) + climbing_stairs_helper(n - 2, memo);
        memo.insert(n, res);
        res
    }

    climbing_stairs_helper(n, &mut HashMap::new())
}

pub fn climbing_stairs_bottom_up(n: usize) -> usize {
    if n <= 2 {
        return n;
    }

    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..n + 1 {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}

/// Minimum Coin Combination
///
/// You are given an array of coin values and a target amount of money. Return the minimum
/// number of coins needed to total the target amount. If this isn't possible, return -1.
/// You may assume there's an unlimited supply of each coin.
///
/// # Example 1
///
/// ```text
/// Input: coins = [1, 2, 3], target = 5
///
/// Output: 2
///
/// Explanation: Use one 2-dollar coin and one 3-dollar coin to make 5 dollars.
/// ```
///
/// # Example 2
///
/// ```text
/// Input: coins = [2, 4], target = 5
///
/// Output: -1
/// ```
pub fn min_coin_combination(coins: &[usize], target: usize) -> i32 {
    fn min_coin_combination_helper(
        coins: &[usize],
        target: usize,
        memo: &mut HashMap<usize, usize>,
    ) -> usize {
        if target == 0 {
            return 0;
        }

        if let Some(&val) = memo.get(&target) {
            return val;
        }

        let mut min_coin_combination = usize::MAX;
        for &coin in coins {
            if coin <= target {
                let sub_result = min_coin_combination_helper(coins, target - coin, memo);
                if sub_result != usize::MAX {
                    min_coin_combination = min_coin_combination.min(1 + sub_result);
                }
            }
        }

        memo.insert(target, min_coin_combination);
        min_coin_combination
    }

    let res = min_coin_combination_helper(coins, target, &mut HashMap::new());
    if res == usize::MAX { -1 } else { res as i32 }
}

pub fn min_coin_combination_bottom_up(coins: &[usize], target: usize) -> i32 {
    // The DP array will store the minimum number of coins needed for
    // each amount. Set each element to a large number initially.
    let mut dp = vec![usize::MAX; target + 1];
    // Base case: if the target is 0, then 0 coins are needed.
    dp[0] = 0;
    // Update the DP array for all target amounts greater than 0.
    for t in 1..=target {
        for &coin in coins {
            if coin <= t && dp[t - coin] != usize::MAX {
                dp[t] = dp[t].min(1 + dp[t - coin]);
            }
        }
    }
    if dp[target] != usize::MAX {
        dp[target] as i32
    } else {
        -1
    }
}

/// Matrix Pathways
///
/// You are positioned at the top-left corner of a m x n matrix, and can only move downward
/// or rightward through the matrix. Determine the number of unique pathways you can take
/// to reach the bottom-right corner of the matrix.
///
/// # Example
///
/// ```text
/// Input: m = 3, n = 3
///
/// Output: 6
/// ```
///
/// # Constraints
///
/// - m, n >= 1
pub fn matrix_pathways(m: usize, n: usize) -> usize {
    let mut dp = vec![vec![1; n]; m];

    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
        }
    }

    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climbing_stairs_example() {
        assert_eq!(climbing_stairs(4), 5);
    }

    #[test]
    fn test_climbing_stairs_1() {
        assert_eq!(climbing_stairs(1), 1);
    }

    #[test]
    fn test_climbing_stairs_2() {
        assert_eq!(climbing_stairs(2), 2);
    }

    #[test]
    fn test_climbing_stairs_3() {
        assert_eq!(climbing_stairs(3), 3);
    }

    #[test]
    fn test_climbing_stairs_5() {
        assert_eq!(climbing_stairs(5), 8);
    }

    #[test]
    fn test_climbing_stairs_10() {
        assert_eq!(climbing_stairs(10), 89);
    }

    #[test]
    fn test_climbing_stairs_bottom_up_example() {
        assert_eq!(climbing_stairs_bottom_up(4), 5);
    }

    #[test]
    fn test_climbing_stairs_bottom_up_1() {
        assert_eq!(climbing_stairs_bottom_up(1), 1);
    }

    #[test]
    fn test_climbing_stairs_bottom_up_2() {
        assert_eq!(climbing_stairs_bottom_up(2), 2);
    }

    #[test]
    fn test_climbing_stairs_bottom_up_3() {
        assert_eq!(climbing_stairs_bottom_up(3), 3);
    }

    #[test]
    fn test_climbing_stairs_bottom_up_5() {
        assert_eq!(climbing_stairs_bottom_up(5), 8);
    }

    #[test]
    fn test_climbing_stairs_bottom_up_10() {
        assert_eq!(climbing_stairs_bottom_up(10), 89);
    }

    #[test]
    fn test_min_coin_combination_example1() {
        assert_eq!(min_coin_combination(&[1, 2, 3], 5), 2);
    }

    #[test]
    fn test_min_coin_combination_example2() {
        assert_eq!(min_coin_combination(&[2, 4], 5), -1);
    }

    #[test]
    fn test_min_coin_combination_zero() {
        assert_eq!(min_coin_combination(&[1, 2, 3], 0), 0);
    }

    #[test]
    fn test_min_coin_combination_single_coin() {
        assert_eq!(min_coin_combination(&[5], 10), 2);
    }

    #[test]
    fn test_min_coin_combination_exact_coin() {
        assert_eq!(min_coin_combination(&[1, 5, 10], 10), 1);
    }

    #[test]
    fn test_min_coin_combination_larger() {
        assert_eq!(min_coin_combination(&[1, 5, 10, 25], 63), 6); // 25+25+10+1+1+1
    }

    #[test]
    fn test_min_coin_combination_bottom_up_example1() {
        assert_eq!(min_coin_combination_bottom_up(&[1, 2, 3], 5), 2);
    }

    #[test]
    fn test_min_coin_combination_bottom_up_example2() {
        assert_eq!(min_coin_combination_bottom_up(&[2, 4], 5), -1);
    }

    #[test]
    fn test_min_coin_combination_bottom_up_zero() {
        assert_eq!(min_coin_combination_bottom_up(&[1, 2, 3], 0), 0);
    }

    #[test]
    fn test_min_coin_combination_bottom_up_single_coin() {
        assert_eq!(min_coin_combination_bottom_up(&[5], 10), 2);
    }

    #[test]
    fn test_min_coin_combination_bottom_up_exact_coin() {
        assert_eq!(min_coin_combination_bottom_up(&[1, 5, 10], 10), 1);
    }

    #[test]
    fn test_min_coin_combination_bottom_up_larger() {
        assert_eq!(min_coin_combination_bottom_up(&[1, 5, 10, 25], 63), 6); // 25+25+10+1+1+1
    }

    #[test]
    fn test_matrix_pathways_example() {
        assert_eq!(matrix_pathways(3, 3), 6);
    }

    #[test]
    fn test_matrix_pathways_1x1() {
        assert_eq!(matrix_pathways(1, 1), 1);
    }

    #[test]
    fn test_matrix_pathways_1xn() {
        assert_eq!(matrix_pathways(1, 5), 1);
    }

    #[test]
    fn test_matrix_pathways_mx1() {
        assert_eq!(matrix_pathways(5, 1), 1);
    }

    #[test]
    fn test_matrix_pathways_2x2() {
        assert_eq!(matrix_pathways(2, 2), 2);
    }

    #[test]
    fn test_matrix_pathways_3x7() {
        assert_eq!(matrix_pathways(3, 7), 28);
    }
}
