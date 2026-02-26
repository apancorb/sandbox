"""
Dynamic Programming Pattern

A collection of algorithm problems using dynamic programming.

DP = break a problem into overlapping subproblems, solve each once, reuse results.

Two approaches:
    Top-down (memoization): recursive + cache already-solved subproblems
    Bottom-up (tabulation):  fill a table iteratively from base cases up

When to use DP:
    1. Optimal substructure: optimal solution uses optimal solutions to subproblems
    2. Overlapping subproblems: same subproblem solved multiple times

Common DP patterns:
    dp[i] = best answer using first i elements
    dp[i][j] = best answer for subproblem defined by two parameters
    Kadane's: dp[i] = best ending at index i
"""


def climbing_stairs(n: int) -> int:
    """
    Climbing Stairs (Top-Down)

    Count distinct ways to climb n steps, taking 1 or 2 steps at a time.

    Example:
        >>> climbing_stairs(4)
        5

        Ways: 1+1+1+1, 1+1+2, 1+2+1, 2+1+1, 2+2

    Time Complexity: O(n)
    Space Complexity: O(n) for memo + recursion stack

    Recurrence: ways(n) = ways(n-1) + ways(n-2)
    From step n, you could have come from n-1 (took 1 step) or n-2 (took 2 steps).
    Same as Fibonacci!
    """
    memo = {}

    def helper(n):
        if n <= 2:
            return n
        if n in memo:
            return memo[n]
        memo[n] = helper(n - 1) + helper(n - 2)
        return memo[n]

    return helper(n)


def climbing_stairs_bottom_up(n: int) -> int:
    """
    Climbing Stairs (Bottom-Up)

    Same problem, iterative approach.

    Time Complexity: O(n)
    Space Complexity: O(n)

    Build dp table from base cases:
        dp[1] = 1 (one way: take 1 step)
        dp[2] = 2 (two ways: 1+1 or 2)
        dp[i] = dp[i-1] + dp[i-2]
    """
    if n <= 2:
        return n

    dp = [0] * (n + 1)
    dp[1] = 1
    dp[2] = 2

    for i in range(3, n + 1):
        dp[i] = dp[i - 1] + dp[i - 2]

    return dp[n]


# -----------------------------------------------------------------------------
# Tests for climbing_stairs
# -----------------------------------------------------------------------------

def test_climbing_stairs_example():
    assert climbing_stairs(4) == 5

def test_climbing_stairs_1():
    assert climbing_stairs(1) == 1

def test_climbing_stairs_2():
    assert climbing_stairs(2) == 2

def test_climbing_stairs_3():
    assert climbing_stairs(3) == 3

def test_climbing_stairs_5():
    assert climbing_stairs(5) == 8

def test_climbing_stairs_10():
    assert climbing_stairs(10) == 89

def test_climbing_stairs_bottom_up_example():
    assert climbing_stairs_bottom_up(4) == 5

def test_climbing_stairs_bottom_up_1():
    assert climbing_stairs_bottom_up(1) == 1

def test_climbing_stairs_bottom_up_2():
    assert climbing_stairs_bottom_up(2) == 2

def test_climbing_stairs_bottom_up_3():
    assert climbing_stairs_bottom_up(3) == 3

def test_climbing_stairs_bottom_up_5():
    assert climbing_stairs_bottom_up(5) == 8

def test_climbing_stairs_bottom_up_10():
    assert climbing_stairs_bottom_up(10) == 89


def min_coin_combination(coins: list[int], target: int) -> int:
    """
    Minimum Coin Combination (Top-Down)

    Return minimum coins needed to make target. Return -1 if impossible.
    Unlimited supply of each coin.

    Example:
        >>> min_coin_combination([1, 2, 3], 5)
        2

        Use 2 + 3 = 5 (2 coins)

    Time Complexity: O(target * len(coins))
    Space Complexity: O(target) for memo

    For each amount, try every coin and take the minimum.
    Recurrence: dp(t) = 1 + min(dp(t - coin) for each coin <= t)
    """
    memo = {}

    def helper(t):
        if t == 0:
            return 0
        if t in memo:
            return memo[t]

        best = float('inf')
        for coin in coins:
            if coin <= t:
                sub = helper(t - coin)
                if sub != float('inf'):
                    best = min(best, 1 + sub)

        memo[t] = best
        return best

    result = helper(target)
    return -1 if result == float('inf') else result


def min_coin_combination_bottom_up(coins: list[int], target: int) -> int:
    """
    Minimum Coin Combination (Bottom-Up)

    Same problem, iterative approach.

    Time Complexity: O(target * len(coins))
    Space Complexity: O(target)

    dp[t] = min coins to make amount t
    Base: dp[0] = 0
    For each amount 1..target, try all coins:
        dp[t] = min(dp[t], 1 + dp[t - coin]) if coin <= t
    """
    dp = [float('inf')] * (target + 1)
    dp[0] = 0

    for t in range(1, target + 1):
        for coin in coins:
            if coin <= t and dp[t - coin] != float('inf'):
                dp[t] = min(dp[t], 1 + dp[t - coin])

    return dp[target] if dp[target] != float('inf') else -1


# -----------------------------------------------------------------------------
# Tests for min_coin_combination
# -----------------------------------------------------------------------------

def test_min_coin_combination_example1():
    assert min_coin_combination([1, 2, 3], 5) == 2

def test_min_coin_combination_example2():
    assert min_coin_combination([2, 4], 5) == -1

def test_min_coin_combination_zero():
    assert min_coin_combination([1, 2, 3], 0) == 0

def test_min_coin_combination_single_coin():
    assert min_coin_combination([5], 10) == 2

def test_min_coin_combination_exact_coin():
    assert min_coin_combination([1, 5, 10], 10) == 1

def test_min_coin_combination_larger():
    assert min_coin_combination([1, 5, 10, 25], 63) == 6

def test_min_coin_combination_bottom_up_example1():
    assert min_coin_combination_bottom_up([1, 2, 3], 5) == 2

def test_min_coin_combination_bottom_up_example2():
    assert min_coin_combination_bottom_up([2, 4], 5) == -1

def test_min_coin_combination_bottom_up_zero():
    assert min_coin_combination_bottom_up([1, 2, 3], 0) == 0

def test_min_coin_combination_bottom_up_single_coin():
    assert min_coin_combination_bottom_up([5], 10) == 2

def test_min_coin_combination_bottom_up_exact_coin():
    assert min_coin_combination_bottom_up([1, 5, 10], 10) == 1

def test_min_coin_combination_bottom_up_larger():
    assert min_coin_combination_bottom_up([1, 5, 10, 25], 63) == 6


def matrix_pathways(m: int, n: int) -> int:
    """
    Matrix Pathways

    Count unique paths from top-left to bottom-right of an m x n grid.
    Can only move down or right.

    Example:
        >>> matrix_pathways(3, 3)
        6

    Time Complexity: O(m * n)
    Space Complexity: O(m * n)

    dp[i][j] = number of ways to reach cell (i, j).
    First row and first column are all 1 (only one way: straight line).
    For other cells: dp[i][j] = dp[i-1][j] + dp[i][j-1]
        (came from above or from left)

    Grid for 3x3:
        1  1  1
        1  2  3
        1  3  6  ← answer
    """
    dp = [[1] * n for _ in range(m)]

    for i in range(1, m):
        for j in range(1, n):
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1]

    return dp[m - 1][n - 1]


# -----------------------------------------------------------------------------
# Tests for matrix_pathways
# -----------------------------------------------------------------------------

def test_matrix_pathways_example():
    assert matrix_pathways(3, 3) == 6

def test_matrix_pathways_1x1():
    assert matrix_pathways(1, 1) == 1

def test_matrix_pathways_1xn():
    assert matrix_pathways(1, 5) == 1

def test_matrix_pathways_mx1():
    assert matrix_pathways(5, 1) == 1

def test_matrix_pathways_2x2():
    assert matrix_pathways(2, 2) == 2

def test_matrix_pathways_3x7():
    assert matrix_pathways(3, 7) == 28


def neighborhood_burglary(houses: list[int]) -> int:
    """
    Neighborhood Burglary

    Max money from non-adjacent houses. Can't rob two houses in a row.

    Example:
        >>> neighborhood_burglary([200, 300, 200, 50])
        400

        Rob houses 0 and 2: 200 + 200 = 400

    Time Complexity: O(n)
    Space Complexity: O(n)

    dp[i] = max money robbing from houses[0..i].
    At each house: rob it (houses[i] + dp[i-2]) or skip it (dp[i-1]).
        dp[i] = max(dp[i-1], houses[i] + dp[i-2])

    Example [200, 300, 200, 50]:
        dp[0] = 200
        dp[1] = max(200, 300) = 300
        dp[2] = max(300, 200+200) = 400  ← rob 0 and 2
        dp[3] = max(400, 50+300) = 400   ← skip house 3
    """
    if not houses:
        return 0
    if len(houses) == 1:
        return houses[0]

    dp = [0] * len(houses)
    dp[0] = houses[0]
    dp[1] = max(houses[0], houses[1])

    for i in range(2, len(houses)):
        dp[i] = max(dp[i - 1], houses[i] + dp[i - 2])

    return dp[-1]


# -----------------------------------------------------------------------------
# Tests for neighborhood_burglary
# -----------------------------------------------------------------------------

def test_neighborhood_burglary_example():
    assert neighborhood_burglary([200, 300, 200, 50]) == 400

def test_neighborhood_burglary_empty():
    assert neighborhood_burglary([]) == 0

def test_neighborhood_burglary_single():
    assert neighborhood_burglary([100]) == 100

def test_neighborhood_burglary_two():
    assert neighborhood_burglary([100, 200]) == 200

def test_neighborhood_burglary_all_same():
    assert neighborhood_burglary([10, 10, 10, 10, 10]) == 30

def test_neighborhood_burglary_alternating():
    assert neighborhood_burglary([1, 100, 1, 100, 1]) == 200


def longest_common_subsequence(s1: str, s2: str) -> int:
    """
    Longest Common Subsequence

    Find the length of the longest common subsequence of two strings.
    A subsequence keeps order but can skip characters.

    Example:
        >>> longest_common_subsequence("acabac", "aebab")
        3

        LCS is "aba" (length 3)

    Time Complexity: O(m * n) where m, n = lengths of s1, s2
    Space Complexity: O(m * n)

    dp[i][j] = LCS length of s1[0..i] and s2[0..j].

    If chars match:  dp[i][j] = 1 + dp[i-1][j-1]  (use both chars)
    If they don't:   dp[i][j] = max(dp[i-1][j], dp[i][j-1])  (skip one)

    Table for "acabac" vs "aebab":
              ""  a  e  b  a  b
        ""  [  0  0  0  0  0  0 ]
        a   [  0  1  1  1  1  1 ]
        c   [  0  1  1  1  1  1 ]
        a   [  0  1  1  1  2  2 ]
        b   [  0  1  1  2  2  3 ]
        a   [  0  1  1  2  3  3 ]
        c   [  0  1  1  2  3  3 ]  ← answer: 3
    """
    m, n = len(s1), len(s2)
    dp = [[0] * (n + 1) for _ in range(m + 1)]

    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = 1 + dp[i - 1][j - 1]
            else:
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])

    return dp[m][n]


# -----------------------------------------------------------------------------
# Tests for longest_common_subsequence
# -----------------------------------------------------------------------------

def test_longest_common_subsequence_example():
    assert longest_common_subsequence("acabac", "aebab") == 3

def test_longest_common_subsequence_empty():
    assert longest_common_subsequence("", "abc") == 0
    assert longest_common_subsequence("abc", "") == 0

def test_longest_common_subsequence_no_common():
    assert longest_common_subsequence("abc", "xyz") == 0

def test_longest_common_subsequence_identical():
    assert longest_common_subsequence("abcd", "abcd") == 4

def test_longest_common_subsequence_subsequence():
    assert longest_common_subsequence("abcde", "ace") == 3

def test_longest_common_subsequence_single_char():
    assert longest_common_subsequence("a", "a") == 1
    assert longest_common_subsequence("a", "b") == 0


def longest_palindrome(s: str) -> str:
    """
    Longest Palindrome in a String

    Return the longest palindromic substring.

    Example:
        >>> longest_palindrome("abccbaba")
        'abccba'

    Time Complexity: O(n^2)
    Space Complexity: O(n^2)

    dp[i][j] = True if s[i..j] is a palindrome.

    Base cases:
        Single chars: dp[i][i] = True
        Two chars: dp[i][i+1] = True if s[i] == s[i+1]

    Expand: for length 3..n:
        dp[i][j] = s[i] == s[j] and dp[i+1][j-1]
        (outer chars match AND inner substring is a palindrome)
    """
    n = len(s)
    if n == 0:
        return ""

    dp = [[False] * n for _ in range(n)]
    start = 0
    max_len = 1

    # Every single char is a palindrome
    for i in range(n):
        dp[i][i] = True

    # Check pairs
    for i in range(n - 1):
        if s[i] == s[i + 1]:
            dp[i][i + 1] = True
            start = i
            max_len = 2

    # Check lengths 3 to n
    for length in range(3, n + 1):
        for i in range(n - length + 1):
            j = i + length - 1
            if s[i] == s[j] and dp[i + 1][j - 1]:
                dp[i][j] = True
                start = i
                max_len = length

    return s[start:start + max_len]


# -----------------------------------------------------------------------------
# Tests for longest_palindrome
# -----------------------------------------------------------------------------

def test_longest_palindrome_example():
    assert longest_palindrome("abccbaba") == "abccba"

def test_longest_palindrome_empty():
    assert longest_palindrome("") == ""

def test_longest_palindrome_single():
    assert longest_palindrome("a") == "a"

def test_longest_palindrome_all_same():
    assert longest_palindrome("aaaa") == "aaaa"

def test_longest_palindrome_odd_length():
    assert longest_palindrome("racecar") == "racecar"

def test_longest_palindrome_at_end():
    assert longest_palindrome("xyzaba") == "aba"


def max_subarray_sum(nums: list[int]) -> int:
    """
    Maximum Subarray Sum (Kadane's Algorithm)

    Return the largest sum of any contiguous subarray.

    Example:
        >>> max_subarray_sum([3, 1, -6, 2, -1, 4, -9])
        5

        Subarray [2, -1, 4] has sum 5

    Time Complexity: O(n)
    Space Complexity: O(1)

    Kadane's: at each element, decide: extend current subarray or start fresh?
        curr = max(num, curr + num)
        If curr + num < num, previous subarray is dragging us down → start new.

    Walkthrough [3, 1, -6, 2, -1, 4, -9]:
        3:  curr=3,  best=3
        1:  curr=4,  best=4
       -6:  curr=-2, best=4
        2:  curr=2,  best=4   ← started fresh (2 > -2+2)
       -1:  curr=1,  best=4
        4:  curr=5,  best=5   ← new best!
       -9:  curr=-4, best=5
    """
    curr = best = nums[0]

    for num in nums[1:]:
        curr = max(num, curr + num)
        best = max(best, curr)

    return best


def max_subarray_sum_dp(nums: list[int]) -> int:
    """
    Maximum Subarray Sum (DP Array)

    Same problem using explicit dp array.

    Time Complexity: O(n)
    Space Complexity: O(n)

    dp[i] = max subarray sum ending at index i.
        dp[i] = max(nums[i], dp[i-1] + nums[i])
    Answer = max(dp)
    """
    if not nums:
        return 0

    dp = [0] * len(nums)
    dp[0] = nums[0]
    best = dp[0]

    for i in range(1, len(nums)):
        dp[i] = max(nums[i], dp[i - 1] + nums[i])
        best = max(best, dp[i])

    return best


# -----------------------------------------------------------------------------
# Tests for max_subarray_sum
# -----------------------------------------------------------------------------

def test_max_subarray_sum_example():
    assert max_subarray_sum([3, 1, -6, 2, -1, 4, -9]) == 5

def test_max_subarray_sum_all_positive():
    assert max_subarray_sum([1, 2, 3, 4]) == 10

def test_max_subarray_sum_all_negative():
    assert max_subarray_sum([-3, -1, -4, -2]) == -1

def test_max_subarray_sum_single():
    assert max_subarray_sum([5]) == 5
    assert max_subarray_sum([-5]) == -5

def test_max_subarray_sum_mixed():
    assert max_subarray_sum([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6

def test_max_subarray_sum_dp_example():
    assert max_subarray_sum_dp([3, 1, -6, 2, -1, 4, -9]) == 5

def test_max_subarray_sum_dp_all_positive():
    assert max_subarray_sum_dp([1, 2, 3, 4]) == 10

def test_max_subarray_sum_dp_all_negative():
    assert max_subarray_sum_dp([-3, -1, -4, -2]) == -1

def test_max_subarray_sum_dp_single():
    assert max_subarray_sum_dp([5]) == 5
    assert max_subarray_sum_dp([-5]) == -5

def test_max_subarray_sum_dp_mixed():
    assert max_subarray_sum_dp([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6

def test_max_subarray_sum_dp_empty():
    assert max_subarray_sum_dp([]) == 0


def knapsack(cap: int, weights: list[int], values: list[int]) -> int:
    """
    0/1 Knapsack

    Given items with weights and values, find max value that fits in capacity.
    Each item can be taken at most once (0/1 = take or skip).

    Example:
        >>> knapsack(7, [5, 3, 4, 1], [70, 50, 40, 10])
        90

        Take items 1 and 2 (weight 3+4=7, value 50+40=90)

    Time Complexity: O(n * cap)
    Space Complexity: O(n * cap)

    dp[i][c] = max value using items i..n-1 with capacity c.

    For each item:
        If it fits: dp[i][c] = max(skip it, take it)
            skip: dp[i+1][c]
            take: values[i] + dp[i+1][c - weights[i]]
        If too heavy: dp[i][c] = dp[i+1][c]  (must skip)

    Fill bottom-up from last item to first.
    """
    n = len(values)
    if n == 0 or cap == 0:
        return 0

    dp = [[0] * (cap + 1) for _ in range(n + 1)]

    for i in range(n - 1, -1, -1):
        for c in range(1, cap + 1):
            if weights[i] <= c:
                dp[i][c] = max(dp[i + 1][c], values[i] + dp[i + 1][c - weights[i]])
            else:
                dp[i][c] = dp[i + 1][c]

    return dp[0][cap]


# -----------------------------------------------------------------------------
# Tests for knapsack
# -----------------------------------------------------------------------------

def test_knapsack_example():
    assert knapsack(7, [5, 3, 4, 1], [70, 50, 40, 10]) == 90

def test_knapsack_empty():
    assert knapsack(10, [], []) == 0

def test_knapsack_zero_capacity():
    assert knapsack(0, [1, 2, 3], [10, 20, 30]) == 0

def test_knapsack_single_item_fits():
    assert knapsack(5, [3], [100]) == 100

def test_knapsack_single_item_too_heavy():
    assert knapsack(2, [3], [100]) == 0

def test_knapsack_take_all():
    assert knapsack(10, [1, 2, 3], [10, 20, 30]) == 60

def test_knapsack_choose_lighter():
    assert knapsack(5, [4, 5], [50, 40]) == 50


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
