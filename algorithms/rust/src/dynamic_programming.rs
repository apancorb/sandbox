use std::collections::HashMap;

/// Climbing Stairs (Top-Down)
///
/// Count distinct ways to climb n steps, taking 1 or 2 steps at a time.
///
/// # Examples
///
/// ```text
/// Input: n = 4
/// Output: 5
/// Ways: 1+1+1+1, 1+1+2, 1+2+1, 2+1+1, 2+2
/// ```
///
/// Recurrence: ways(n) = ways(n-1) + ways(n-2)
/// From step n, you could have come from n-1 (took 1 step) or n-2 (took 2 steps).
/// Same as Fibonacci!
///
/// Example walkthrough for n=4:
///
/// ```text
/// ways(4) = ways(3) + ways(2)
/// ways(3) = ways(2) + ways(1) = 2 + 1 = 3
/// ways(2) = 2 (base case)
/// ways(4) = 3 + 2 = 5
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(n) for memo + recursion stack
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

/// Climbing Stairs (Bottom-Up)
///
/// Same problem, iterative approach. Build a dp table from base cases
/// instead of using recursion.
///
/// # Examples
///
/// ```text
/// Input: n = 4
/// Output: 5
/// ```
///
/// Build dp table from base cases. Each step count is the sum of the
/// two previous, since you can arrive by taking 1 or 2 steps.
///
/// Example walkthrough for n=4:
///
/// ```text
/// dp[1] = 1 (one way: take 1 step)
/// dp[2] = 2 (two ways: 1+1 or 2)
/// dp[3] = dp[2] + dp[1] = 2 + 1 = 3
/// dp[4] = dp[3] + dp[2] = 3 + 2 = 5
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(n)
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

/// Minimum Coin Combination (Top-Down)
///
/// Return minimum coins needed to make target. Return -1 if impossible.
/// Unlimited supply of each coin.
///
/// # Examples
///
/// ```text
/// Input: coins = [1, 2, 3], target = 5
/// Output: 2  (use 2 + 3 = 5, two coins)
/// ```
///
/// ```text
/// Input: coins = [2, 4], target = 5
/// Output: -1
/// ```
///
/// For each amount, try every coin and take the minimum. The key insight
/// is that after using one coin, you have a smaller subproblem of the
/// same type. Recurrence: dp(t) = 1 + min(dp(t - coin) for each coin <= t).
///
/// Example walkthrough for coins=[1,2,3], target=5:
///
/// ```text
/// dp(5) = 1 + min(dp(4), dp(3), dp(2))
/// dp(3) = 1 + min(dp(2), dp(1), dp(0)) = 1 + 0 = 1  (use coin 3)
/// dp(2) = 1 + min(dp(1), dp(0)) = 1 + 0 = 1         (use coin 2)
/// dp(5) = 1 + min(dp(4), 1, 1) = 1 + 1 = 2          (use 2+3)
/// ```
///
/// # Complexity
///
/// - Time: O(target * len(coins))
/// - Space: O(target) for memo
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

/// Minimum Coin Combination (Bottom-Up)
///
/// Same problem, iterative approach. Build up from amount 0 to target.
///
/// # Examples
///
/// ```text
/// Input: coins = [1, 2, 3], target = 5
/// Output: 2  (use 2 + 3 = 5, two coins)
/// ```
///
/// Fill a dp table where dp[t] = min coins to make amount t. For each
/// amount from 1 to target, try every coin and keep the minimum.
///
/// Example walkthrough for coins=[1,2,3], target=5:
///
/// ```text
/// dp[0] = 0 (base case)
/// dp[1] = 1 (coin 1)
/// dp[2] = 1 (coin 2)
/// dp[3] = 1 (coin 3)
/// dp[4] = 2 (coin 1 + dp[3], or coin 2 + dp[2])
/// dp[5] = 2 (coin 2 + dp[3], or coin 3 + dp[2])
/// ```
///
/// # Complexity
///
/// - Time: O(target * len(coins))
/// - Space: O(target)
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
/// Count unique paths from top-left to bottom-right of an m x n grid.
/// Can only move down or right.
///
/// # Examples
///
/// ```text
/// Input: m = 3, n = 3
/// Output: 6
/// ```
///
/// dp[i][j] = number of ways to reach cell (i, j).
/// First row and first column are all 1 (only one way: straight line).
/// For other cells: dp[i][j] = dp[i-1][j] + dp[i][j-1]
/// (came from above or from left)
///
/// Example walkthrough for 3x3 grid:
///
/// ```text
/// 1  1  1
/// 1  2  3
/// 1  3  6  ← answer
/// ```
///
/// # Complexity
///
/// - Time: O(m * n)
/// - Space: O(m * n)
pub fn matrix_pathways(m: usize, n: usize) -> usize {
    let mut dp = vec![vec![1; n]; m];

    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
        }
    }

    dp[m - 1][n - 1]
}

/// Neighborhood Burglary
///
/// Max money from non-adjacent houses. Can't rob two houses in a row.
///
/// # Examples
///
/// ```text
/// Input: houses = [200, 300, 200, 50]
/// Output: 400  (rob houses 0 and 2: 200 + 200 = 400)
/// ```
///
/// dp[i] = max money robbing from houses[0..i].
/// At each house: rob it (houses[i] + dp[i-2]) or skip it (dp[i-1]).
/// dp[i] = max(dp[i-1], houses[i] + dp[i-2])
///
/// Example walkthrough for [200, 300, 200, 50]:
///
/// ```text
/// dp[0] = 200
/// dp[1] = max(200, 300) = 300
/// dp[2] = max(300, 200+200) = 400  ← rob 0 and 2
/// dp[3] = max(400, 50+300) = 400   ← skip house 3
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(n)
pub fn neighborhood_burglary(houses: &[usize]) -> usize {
    if houses.is_empty() {
        return 0;
    } else if houses.len() == 1 {
        return houses[0];
    }

    let mut dp = vec![0; houses.len()];
    dp[0] = houses[0];
    dp[1] = houses[0].max(houses[1]);

    for i in 2..dp.len() {
        dp[i] = dp[i - 1].max(houses[i] + dp[i - 2]);
    }

    dp[dp.len() - 1]
}

/// Longest Common Subsequence
///
/// Find the length of the longest common subsequence of two strings.
/// A subsequence keeps order but can skip characters.
///
/// # Examples
///
/// ```text
/// Input: s1 = "acabac", s2 = "aebab"
/// Output: 3  (LCS is "aba")
/// ```
///
/// dp[i][j] = LCS length of s1[0..i] and s2[0..j].
///
/// If chars match:  dp[i][j] = 1 + dp[i-1][j-1]  (use both chars)
/// If they don't:   dp[i][j] = max(dp[i-1][j], dp[i][j-1])  (skip one)
///
/// Example walkthrough — table for "acabac" vs "aebab":
///
/// ```text
///       ""  a  e  b  a  b
/// ""  [  0  0  0  0  0  0 ]
/// a   [  0  1  1  1  1  1 ]
/// c   [  0  1  1  1  1  1 ]
/// a   [  0  1  1  1  2  2 ]
/// b   [  0  1  1  2  2  3 ]
/// a   [  0  1  1  2  3  3 ]
/// c   [  0  1  1  2  3  3 ]  ← answer: 3
/// ```
///
/// # Complexity
///
/// - Time: O(m * n) where m, n = lengths of s1, s2
/// - Space: O(m * n)
pub fn longest_common_subsequence(s1: &str, s2: &str) -> usize {
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[s1.len()][s2.len()]
}

/// Longest Palindrome in a String
///
/// Return the longest palindromic substring.
///
/// # Examples
///
/// ```text
/// Input: s = "abccbaba"
/// Output: "abccba"
/// ```
///
/// dp[i][j] = True if s[i..j] is a palindrome. Start with base cases
/// (single chars and pairs), then expand to longer lengths. If the outer
/// characters match and the inner substring is already a palindrome, the
/// whole thing is a palindrome.
///
/// Example walkthrough for "abccbaba":
///
/// ```text
/// Base: each char is a palindrome, "cc" is a pair palindrome
/// Length 3: "bcc" no, "ccb" no, "bab" yes (b==b and "a" is palindrome)
/// Length 4: "bccb" yes (b==b and "cc" is palindrome)
/// Length 5: "abccb" no (a!=b)
/// Length 6: "abccba" yes (a==a and "bccb" is palindrome) ← answer
/// ```
///
/// # Complexity
///
/// - Time: O(n^2)
/// - Space: O(n^2)
pub fn longest_palindrome(s: &str) -> String {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    if n == 0 {
        return String::new();
    }

    let mut dp = vec![vec![false; n]; n];
    let mut max_len = 1;
    let mut start_index = 0;

    for i in 0..n {
        dp[i][i] = true;
    }

    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = true;
            max_len = 2;
            start_index = i;
        }
    }

    for substr_len in 3..=n {
        for i in 0..(n - substr_len + 1) {
            let j = i + substr_len - 1;
            if s[i] == s[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                max_len = substr_len;
                start_index = i;
            }
        }
    }

    s[start_index..start_index + max_len].iter().collect()
}

/// Maximum Subarray Sum (Kadane's Algorithm)
///
/// Return the largest sum of any contiguous subarray.
///
/// # Examples
///
/// ```text
/// Input: nums = [3, 1, -6, 2, -1, 4, -9]
/// Output: 5  (subarray [2, -1, 4] has sum 5)
/// ```
///
/// Kadane's: at each element, decide: extend current subarray or start fresh?
/// curr = max(num, curr + num)
/// If curr + num < num, previous subarray is dragging us down → start new.
///
/// Example walkthrough for [3, 1, -6, 2, -1, 4, -9]:
///
/// ```text
///  3:  curr=3,  best=3
///  1:  curr=4,  best=4
/// -6:  curr=-2, best=4
///  2:  curr=2,  best=4   ← started fresh (2 > -2+2)
/// -1:  curr=1,  best=4
///  4:  curr=5,  best=5   ← new best!
/// -9:  curr=-4, best=5
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
pub fn max_subarray_sum(nums: &[i32]) -> i32 {
    // Kadane's Algorithm:
    // At each position, decide: start a new subarray here, or extend the previous one?
    // curr_sum = max(num, curr_sum + num)
    //   - If curr_sum + num < num, the previous subarray is dragging us down, so start fresh
    //   - Otherwise, extend the current subarray
    // max_sum tracks the best subarray sum seen so far
    let mut curr_sum = nums[0];
    let mut max_sum = nums[0];

    for &num in &nums[1..] {
        curr_sum = num.max(curr_sum + num);
        max_sum = max_sum.max(curr_sum);
    }

    max_sum
}

/// Maximum Subarray Sum (DP Array)
///
/// Same problem using explicit dp array. dp[i] = max subarray sum ending
/// at index i.
///
/// # Examples
///
/// ```text
/// Input: nums = [3, 1, -6, 2, -1, 4, -9]
/// Output: 5  (subarray [2, -1, 4] has sum 5)
/// ```
///
/// At each index, either extend the previous subarray or start fresh.
/// dp[i] = max(nums[i], dp[i-1] + nums[i]). The answer is max(dp).
///
/// Example walkthrough for [3, 1, -6, 2, -1, 4, -9]:
///
/// ```text
/// dp[0] = 3
/// dp[1] = max(1, 3+1) = 4
/// dp[2] = max(-6, 4-6) = -2
/// dp[3] = max(2, -2+2) = 2   ← start fresh
/// dp[4] = max(-1, 2-1) = 1
/// dp[5] = max(4, 1+4) = 5    ← best!
/// dp[6] = max(-9, 5-9) = -4
/// Answer: max(dp) = 5
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(n)
pub fn max_subarray_sum_dp(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut dp = vec![0; nums.len()];
    // Base case: the maximum subarray sum of an array with just one
    // element is that element.
    dp[0] = nums[0];
    let mut max_sum = dp[0];

    // Populate the rest of the DP array.
    for i in 1..nums.len() {
        // Determine the maximum subarray sum ending at the current index.
        dp[i] = (dp[i - 1] + nums[i]).max(nums[i]);
        max_sum = max_sum.max(dp[i]);
    }

    max_sum
}

/// 0/1 Knapsack
///
/// Given items with weights and values, find max value that fits in capacity.
/// Each item can be taken at most once (0/1 = take or skip).
///
/// # Examples
///
/// ```text
/// Input: cap = 7, weights = [5, 3, 4, 1], values = [70, 50, 40, 10]
/// Output: 90  (take items 1 and 2: weight 3+4=7, value 50+40=90)
/// ```
///
/// dp[i][c] = max value using items i..n-1 with capacity c. For each item,
/// either skip it or take it (if it fits). Fill bottom-up from the last
/// item to the first.
///
/// Example walkthrough for cap=7, weights=[5,3,4,1], values=[70,50,40,10]:
///
/// ```text
/// Start from item 3 (w=1, v=10):
///     dp[3][1..7] = 10 (always fits)
/// Item 2 (w=4, v=40):
///     dp[2][4] = max(skip=10, take=40) = 40
///     dp[2][5] = max(10, 40+10) = 50
/// Item 1 (w=3, v=50):
///     dp[1][3] = max(dp[2][3], 50+dp[2][0]) = max(10, 50) = 50
///     dp[1][7] = max(dp[2][7], 50+dp[2][4]) = max(50, 50+40) = 90 ★
/// Answer: dp[0][7] = 90
/// ```
///
/// # Complexity
///
/// - Time: O(n * cap)
/// - Space: O(n * cap)
pub fn knapsack(cap: usize, weights: &[usize], values: &[usize]) -> usize {
    let n = values.len();
    if n == 0 || cap == 0 {
        return 0;
    }
    // base case: first col is 0 since cap of 0 means no items fit in the
    // knapsack. last row is set to 0 since there is no nth item to pick from
    let mut dp = vec![vec![0; cap + 1]; n + 1];
    // populate the dp table
    for i in (0..n).rev() {
        for c in 1..=cap {
            // if the item 'i' fits in the current knapsack cap,
            // the maximum value at dp[i][c] is the largest of either:
            // 1. The maximum value if we include item 'i'
            // 2. The maximum value if we exclude item 'i'
            if weights[i] <= c {
                dp[i][c] = dp[i + 1][c].max(values[i] + dp[i + 1][c - weights[i]]);
            // if it does not fit, we have to exclude it
            } else {
                dp[i][c] = dp[i + 1][c];
            }
        }
    }
    dbg!(&dp);
    dp[0][cap]
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

    #[test]
    fn test_neighborhood_burglary_example() {
        assert_eq!(neighborhood_burglary(&[200, 300, 200, 50]), 400);
    }

    #[test]
    fn test_neighborhood_burglary_empty() {
        assert_eq!(neighborhood_burglary(&[]), 0);
    }

    #[test]
    fn test_neighborhood_burglary_single() {
        assert_eq!(neighborhood_burglary(&[100]), 100);
    }

    #[test]
    fn test_neighborhood_burglary_two() {
        assert_eq!(neighborhood_burglary(&[100, 200]), 200);
    }

    #[test]
    fn test_neighborhood_burglary_all_same() {
        assert_eq!(neighborhood_burglary(&[10, 10, 10, 10, 10]), 30);
    }

    #[test]
    fn test_neighborhood_burglary_alternating() {
        assert_eq!(neighborhood_burglary(&[1, 100, 1, 100, 1]), 200);
    }

    #[test]
    fn test_longest_common_subsequence_example() {
        assert_eq!(longest_common_subsequence("acabac", "aebab"), 3);
    }

    #[test]
    fn test_longest_common_subsequence_empty() {
        assert_eq!(longest_common_subsequence("", "abc"), 0);
        assert_eq!(longest_common_subsequence("abc", ""), 0);
    }

    #[test]
    fn test_longest_common_subsequence_no_common() {
        assert_eq!(longest_common_subsequence("abc", "xyz"), 0);
    }

    #[test]
    fn test_longest_common_subsequence_identical() {
        assert_eq!(longest_common_subsequence("abcd", "abcd"), 4);
    }

    #[test]
    fn test_longest_common_subsequence_subsequence() {
        assert_eq!(longest_common_subsequence("abcde", "ace"), 3);
    }

    #[test]
    fn test_longest_common_subsequence_single_char() {
        assert_eq!(longest_common_subsequence("a", "a"), 1);
        assert_eq!(longest_common_subsequence("a", "b"), 0);
    }

    #[test]
    fn test_longest_palindrome_example() {
        assert_eq!(longest_palindrome("abccbaba"), "abccba");
    }

    #[test]
    fn test_longest_palindrome_empty() {
        assert_eq!(longest_palindrome(""), "");
    }

    #[test]
    fn test_longest_palindrome_single() {
        assert_eq!(longest_palindrome("a"), "a");
    }

    #[test]
    fn test_longest_palindrome_all_same() {
        assert_eq!(longest_palindrome("aaaa"), "aaaa");
    }

    #[test]
    fn test_longest_palindrome_odd_length() {
        assert_eq!(longest_palindrome("racecar"), "racecar");
    }

    #[test]
    fn test_longest_palindrome_at_end() {
        assert_eq!(longest_palindrome("xyzaba"), "aba");
    }

    #[test]
    fn test_max_subarray_sum_example() {
        assert_eq!(max_subarray_sum(&[3, 1, -6, 2, -1, 4, -9]), 5);
    }

    #[test]
    fn test_max_subarray_sum_all_positive() {
        assert_eq!(max_subarray_sum(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_max_subarray_sum_all_negative() {
        assert_eq!(max_subarray_sum(&[-3, -1, -4, -2]), -1);
    }

    #[test]
    fn test_max_subarray_sum_single() {
        assert_eq!(max_subarray_sum(&[5]), 5);
        assert_eq!(max_subarray_sum(&[-5]), -5);
    }

    #[test]
    fn test_max_subarray_sum_mixed() {
        assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn test_max_subarray_sum_dp_example() {
        assert_eq!(max_subarray_sum_dp(&[3, 1, -6, 2, -1, 4, -9]), 5);
    }

    #[test]
    fn test_max_subarray_sum_dp_all_positive() {
        assert_eq!(max_subarray_sum_dp(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_max_subarray_sum_dp_all_negative() {
        assert_eq!(max_subarray_sum_dp(&[-3, -1, -4, -2]), -1);
    }

    #[test]
    fn test_max_subarray_sum_dp_single() {
        assert_eq!(max_subarray_sum_dp(&[5]), 5);
        assert_eq!(max_subarray_sum_dp(&[-5]), -5);
    }

    #[test]
    fn test_max_subarray_sum_dp_mixed() {
        assert_eq!(max_subarray_sum_dp(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn test_max_subarray_sum_dp_empty() {
        assert_eq!(max_subarray_sum_dp(&[]), 0);
    }

    #[test]
    fn test_knapsack_example() {
        assert_eq!(knapsack(7, &[5, 3, 4, 1], &[70, 50, 40, 10]), 90);
    }

    #[test]
    fn test_knapsack_empty() {
        assert_eq!(knapsack(10, &[], &[]), 0);
    }

    #[test]
    fn test_knapsack_zero_capacity() {
        assert_eq!(knapsack(0, &[1, 2, 3], &[10, 20, 30]), 0);
    }

    #[test]
    fn test_knapsack_single_item_fits() {
        assert_eq!(knapsack(5, &[3], &[100]), 100);
    }

    #[test]
    fn test_knapsack_single_item_too_heavy() {
        assert_eq!(knapsack(2, &[3], &[100]), 0);
    }

    #[test]
    fn test_knapsack_take_all() {
        assert_eq!(knapsack(10, &[1, 2, 3], &[10, 20, 30]), 60);
    }

    #[test]
    fn test_knapsack_choose_lighter() {
        // Can take item with weight 4 (value 50) or item with weight 5 (value 40)
        assert_eq!(knapsack(5, &[4, 5], &[50, 40]), 50);
    }
}
