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
}
