"""
Prefix Sums Pattern

A collection of algorithm problems using prefix sums.

Key idea: Precompute cumulative sums so any range sum = prefix[right] - prefix[left-1]
This turns O(n) range queries into O(1) after O(n) preprocessing.
"""


class RangeSum:
    """
    Sum Between Range

    Given an integer array, return the sum of values between two indexes.

    Example:
        >>> rs = RangeSum([3, -7, 6, 0, -2, 5])
        >>> rs.sum_range(0, 3)  # 3 + (-7) + 6 + 0
        2
        >>> rs.sum_range(2, 4)  # 6 + 0 + (-2)
        4
        >>> rs.sum_range(2, 2)  # just 6
        6

    Build prefix sums array where prefix[i] = sum of nums[0..i].
    Then sum(left, right) = prefix[right] - prefix[left-1]. This
    turns any range sum query into a single subtraction.

    Walkthrough for nums=[3, -7, 6, 0, -2, 5]:
        Build prefix: [3, -4, 2, 2, 0, 5]

        sum_range(0, 3) → prefix[3] = 2 (left=0, return directly)
        sum_range(2, 4) → prefix[4] - prefix[1] = 0 - (-4) = 4
        sum_range(2, 2) → prefix[2] - prefix[1] = 2 - (-4) = 6

    Time Complexity: O(n) preprocessing, O(1) per query
    Space Complexity: O(n) for prefix array
    """

    def __init__(self, nums: list[int]):
        # Build prefix sums: prefix[i] = sum of nums[0..i]
        self.prefix = []
        total = 0
        for num in nums:
            total += num
            self.prefix.append(total)

    def sum_range(self, left: int, right: int) -> int:
        # sum(left, right) = prefix[right] - prefix[left-1]
        if left == 0:
            return self.prefix[right]
        return self.prefix[right] - self.prefix[left - 1]


# -----------------------------------------------------------------------------
# Tests for RangeSum
# -----------------------------------------------------------------------------

def test_range_sum_example():
    rs = RangeSum([3, -7, 6, 0, -2, 5])
    assert rs.sum_range(0, 3) == 2
    assert rs.sum_range(2, 4) == 4
    assert rs.sum_range(2, 2) == 6


def test_range_sum_single_element():
    rs = RangeSum([5])
    assert rs.sum_range(0, 0) == 5


def test_range_sum_full_array():
    rs = RangeSum([1, 2, 3, 4, 5])
    assert rs.sum_range(0, 4) == 15


def test_range_sum_negative():
    rs = RangeSum([-1, -2, -3, -4])
    assert rs.sum_range(0, 3) == -10
    assert rs.sum_range(1, 2) == -5


def test_range_sum_mixed():
    rs = RangeSum([1, -1, 1, -1, 1])
    assert rs.sum_range(0, 4) == 1
    assert rs.sum_range(0, 1) == 0
    assert rs.sum_range(0, 3) == 0


def test_range_sum_multiple_queries():
    rs = RangeSum([10, 20, 30, 40, 50])
    assert rs.sum_range(0, 0) == 10
    assert rs.sum_range(1, 1) == 20
    assert rs.sum_range(4, 4) == 50
    assert rs.sum_range(0, 2) == 60
    assert rs.sum_range(2, 4) == 120


def k_sum_subarrays(nums: list[int], k: int) -> int:
    """
    K-Sum Subarrays

    Find the number of subarrays that sum to k.

    Example:
        >>> k_sum_subarrays([1, 2, -1, 1, 2], 3)
        3
        # [1,2] at 0-1, [1,2,-1,1] at 0-3, [1,2] at 3-4

    Build a prefix sums array with a leading zero so that the sum of any
    subarray nums[i..j] equals prefix[j+1] - prefix[i]. Then check all
    pairs (i, j) to count how many equal k.

    Walkthrough for nums=[1, 2, -1, 1, 2], k=3:
        prefix = [0, 1, 3, 2, 3, 5]

        Check all pairs prefix[j] - prefix[i]:
            prefix[2]-prefix[0] = 3-0 = 3 ✓  → subarray [1,2]
            prefix[4]-prefix[0] = 3-0 = 3 ✓  → subarray [1,2,-1,1]
            prefix[5]-prefix[3] = 5-2 = 3 ✓  → subarray [1,2]

        Answer: 3

    Time Complexity: O(n^2) - check all subarrays using prefix sums
    Space Complexity: O(n) - prefix array

    Note: Can be optimized to O(n) with hashmap, but this version
    is clearer for interviews.
    """
    # Build prefix sums with leading 0
    # prefix[i] = sum of nums[0..i-1]
    prefix = [0]
    total = 0
    for num in nums:
        total += num
        prefix.append(total)

    # Check all subarrays: sum(i,j) = prefix[j+1] - prefix[i]
    count = 0
    for j in range(1, len(prefix)):
        for i in range(j):
            if prefix[j] - prefix[i] == k:
                count += 1

    return count


# -----------------------------------------------------------------------------
# Tests for k_sum_subarrays
# -----------------------------------------------------------------------------

def test_k_sum_subarrays_example():
    assert k_sum_subarrays([1, 2, -1, 1, 2], 3) == 3


def test_k_sum_subarrays_single():
    assert k_sum_subarrays([3], 3) == 1
    assert k_sum_subarrays([5], 3) == 0


def test_k_sum_subarrays_all_zeros():
    assert k_sum_subarrays([0, 0, 0], 0) == 6


def test_k_sum_subarrays_negative_k():
    assert k_sum_subarrays([1, -1, -1, 1], -1) == 4


def test_k_sum_subarrays_whole_array():
    assert k_sum_subarrays([1, 2, 3], 6) == 1


def test_k_sum_subarrays_no_match():
    assert k_sum_subarrays([1, 2, 3], 10) == 0


def test_k_sum_subarrays_multiple_same():
    assert k_sum_subarrays([1, 1, 1], 2) == 2


def product_except_self(nums: list[int]) -> list[int]:
    """
    Product Array Without Current Element

    Return an array where result[i] = product of all elements except nums[i].

    Example:
        >>> product_except_self([2, 3, 1, 4, 5])
        [60, 40, 120, 30, 24]
        # result[0] = 3*1*4*5 = 60 (everything except 2)

    Strategy: For each position, we need product of LEFT side * RIGHT side.
    Build prefix products from left, and prefix products from right. Then
    multiply them together for each index.

    Walkthrough for nums=[2, 3, 1, 4, 5]:
        left products:  [1, 2, 6, 6, 24]
            left[0]=1, left[1]=2, left[2]=2*3=6, left[3]=6*1=6, left[4]=6*4=24

        right products: [60, 20, 20, 5, 1]
            right[4]=1, right[3]=5, right[2]=4*5=20, right[1]=1*20=20, right[0]=3*20=60

        result = left[i] * right[i]:
            [1*60, 2*20, 6*20, 6*5, 24*1] = [60, 40, 120, 30, 24]

    Time Complexity: O(n) - two passes
    Space Complexity: O(n) - for left/right arrays
    """
    if not nums:
        return []

    n = len(nums)

    # left[i] = product of everything to the LEFT of i
    left = [1] * n
    for i in range(1, n):
        left[i] = left[i - 1] * nums[i - 1]

    # right[i] = product of everything to the RIGHT of i
    right = [1] * n
    for i in range(n - 2, -1, -1):
        right[i] = right[i + 1] * nums[i + 1]

    # result[i] = left[i] * right[i]
    return [left[i] * right[i] for i in range(n)]


# -----------------------------------------------------------------------------
# Tests for product_except_self
# -----------------------------------------------------------------------------

def test_product_except_self_example():
    assert product_except_self([2, 3, 1, 4, 5]) == [60, 40, 120, 30, 24]


def test_product_except_self_with_zero():
    assert product_except_self([1, 2, 0, 4]) == [0, 0, 8, 0]


def test_product_except_self_two_zeros():
    assert product_except_self([0, 2, 0, 4]) == [0, 0, 0, 0]


def test_product_except_self_two_elements():
    assert product_except_self([3, 5]) == [5, 3]


def test_product_except_self_negatives():
    assert product_except_self([-1, 2, -3, 4]) == [-24, 12, -8, 6]


def test_product_except_self_all_ones():
    assert product_except_self([1, 1, 1, 1]) == [1, 1, 1, 1]


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
