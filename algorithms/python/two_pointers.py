"""
Two Pointers Pattern

A collection of algorithm problems solved using the two pointers technique.
"""


def pair_sum(nums: list[int], target: int) -> list[int]:
    """
    Pair Sum - Sorted

    Given an array of integers sorted in ascending order and a target value,
    return the indexes of any pair of numbers in the array that sum to the
    target. The order of the indexes in the result doesn't matter. If no pair
    is found, return an empty list.

    Examples:
        >>> pair_sum([-5, -2, 3, 4, 6], 7)
        [2, 3]
        >>> pair_sum([1, 1, 1], 2)  # Any valid pair like [0, 1], [0, 2], [1, 2]
        [0, 1]

    Time Complexity: O(n) - single pass through the array with two pointers
    Space Complexity: O(1) - only using two pointer variables

    The two pointers start at opposite ends. If sum is too small, move left
    pointer right (increase sum). If sum is too large, move right pointer
    left (decrease sum). Works because the array is sorted.
    """
    if len(nums) < 2:
        return []

    left = 0
    right = len(nums) - 1

    while left < right:
        total = nums[left] + nums[right]
        if total == target:
            return [left, right]
        elif total < target:
            left += 1
        else:
            right -= 1

    return []


# -----------------------------------------------------------------------------
# Tests for pair_sum
# -----------------------------------------------------------------------------

def test_pair_sum_example_1():
    assert pair_sum([-5, -2, 3, 4, 6], 7) == [2, 3]


def test_pair_sum_example_2():
    result = pair_sum([1, 1, 1], 2)
    # Any valid pair is acceptable
    assert result in [[0, 1], [1, 0], [0, 2], [2, 0], [1, 2], [2, 1]]


def test_pair_sum_no_solution():
    assert pair_sum([1, 2, 3], 10) == []


def test_pair_sum_negative_numbers():
    assert pair_sum([-10, -5, 0, 5, 10], 0) == [0, 4]


def test_pair_sum_two_elements():
    assert pair_sum([1, 9], 10) == [0, 1]


def test_pair_sum_empty_array():
    assert pair_sum([], 5) == []


def test_pair_sum_single_element():
    assert pair_sum([5], 5) == []


def test_pair_sum_large_numbers():
    assert pair_sum([-1000000, 0, 1000000], 0) == [0, 2]


def triplet_sum(nums: list[int]) -> list[list[int]]:
    """
    Triplet Sum (3Sum)

    Given an array of integers, return all triplets [a, b, c] such that
    a + b + c = 0. The solution must not contain duplicate triplets
    (e.g., [1, 2, 3] and [2, 3, 1] are considered duplicate triplets).
    If no such triplets are found, return an empty list.

    Each triplet can be arranged in any order, and the output can be
    returned in any order.

    Example:
        >>> triplet_sum([0, -1, 2, -3, 1])
        [[-3, 1, 2], [-1, 0, 1]]

    Time Complexity: O(n^2) - sorting is O(n log n), then for each element
        we do a two-pointer search O(n), giving O(n^2) total
    Space Complexity: O(1) - excluding the output, only using pointers
        (sorting may use O(n) depending on implementation)

    For each element, we use two pointers to find pairs that sum to its
    negation. Skip duplicates at each level to avoid duplicate triplets.
    """
    if len(nums) < 3:
        return []

    nums.sort()
    result = []

    for i in range(len(nums)):
        # Early termination: if current number > 0, no valid triplet possible
        if nums[i] > 0:
            break

        # Skip duplicates for the first element
        if i > 0 and nums[i] == nums[i - 1]:
            continue

        # Two pointer search for pairs summing to -nums[i]
        target = -nums[i]
        left = i + 1
        right = len(nums) - 1

        while left < right:
            total = nums[left] + nums[right]
            if total == target:
                result.append([nums[i], nums[left], nums[right]])
                left += 1
                # Skip duplicates for the second element
                while left < right and nums[left] == nums[left - 1]:
                    left += 1
            elif total < target:
                left += 1
            else:
                right -= 1

    return result


# -----------------------------------------------------------------------------
# Tests for triplet_sum
# -----------------------------------------------------------------------------

def test_triplet_sum_example():
    result = triplet_sum([0, -1, 2, -3, 1])
    result = [sorted(t) for t in result]
    result.sort()
    expected = [[-3, 1, 2], [-1, 0, 1]]
    assert result == expected


def test_triplet_sum_empty_array():
    assert triplet_sum([]) == []


def test_triplet_sum_single_element():
    assert triplet_sum([0]) == []


def test_triplet_sum_two_elements():
    assert triplet_sum([1, -1]) == []


def test_triplet_sum_all_zeros():
    assert triplet_sum([0, 0, 0]) == [[0, 0, 0]]


def test_triplet_sum_no_solution():
    assert triplet_sum([1, 0, 1]) == []


def test_triplet_sum_with_duplicates():
    result = triplet_sum([0, 0, 1, -1, 1, -1])
    result = [sorted(t) for t in result]
    result.sort()
    # Should only return one triplet [-1, 0, 1] without duplicates
    assert result == [[-1, 0, 1]]


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
