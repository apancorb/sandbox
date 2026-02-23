"""
Binary Search Pattern

A collection of algorithm problems using binary search.

RULE OF THUMB: When do you need +1 bias in binary search?

    If you're setting left = mid (keeping mid as a candidate), you MUST use +1.
    If you're setting right = mid (keeping mid as a candidate), you DON'T need +1.

Why? Integer division floors down, so mid is biased toward left:
    - left=3, right=4 -> mid = (3+4)//2 = 3 (biased left)
    - If we then do left=mid, we get left=3 again -> infinite loop!
    - Adding +1 fixes it: mid = (3+4)//2 + 1 = 4 -> left=4 -> done

Quick check:
    - Searching for FIRST occurrence? right = mid, no +1 needed
    - Searching for LAST occurrence? left = mid, NEED +1
    - Searching for HIGHEST valid answer? left = mid, NEED +1
    - Searching for LOWEST valid answer? right = mid, no +1 needed
"""


def find_insertion_index(nums: list[int], target: int) -> int:
    """
    Find the Insertion Index

    Given a sorted array with unique values and a target:
    - If target exists, return its index
    - Otherwise, return where it would be inserted to maintain order

    Examples:
        >>> find_insertion_index([1, 2, 4, 5, 7, 8, 9], 4)
        2
        >>> find_insertion_index([1, 2, 4, 5, 7, 8, 9], 6)
        4  # 6 would go between 5 and 7

    Time Complexity: O(log n) - binary search
    Space Complexity: O(1) - only pointers

    Standard binary search. When target not found, left pointer ends up
    at the insertion position.
    """
    left = 0
    right = len(nums)

    while left < right:
        mid = (left + right) // 2
        if nums[mid] >= target:
            right = mid
        else:
            left = mid + 1

    return left


# -----------------------------------------------------------------------------
# Tests for find_insertion_index
# -----------------------------------------------------------------------------

def test_find_insertion_index_found():
    assert find_insertion_index([1, 2, 4, 5, 7, 8, 9], 4) == 2


def test_find_insertion_index_not_found():
    assert find_insertion_index([1, 2, 4, 5, 7, 8, 9], 6) == 4


def test_find_insertion_index_insert_at_start():
    assert find_insertion_index([2, 4, 6, 8], 1) == 0


def test_find_insertion_index_insert_at_end():
    assert find_insertion_index([2, 4, 6, 8], 10) == 4


def test_find_insertion_index_first_element():
    assert find_insertion_index([1, 3, 5, 7], 1) == 0


def test_find_insertion_index_last_element():
    assert find_insertion_index([1, 3, 5, 7], 7) == 3


def test_find_insertion_index_empty():
    assert find_insertion_index([], 5) == 0


def test_find_insertion_index_single_found():
    assert find_insertion_index([5], 5) == 0


def test_find_insertion_index_single_insert_before():
    assert find_insertion_index([5], 3) == 0


def test_find_insertion_index_single_insert_after():
    assert find_insertion_index([5], 7) == 1


def test_find_insertion_index_negative():
    assert find_insertion_index([-10, -5, 0, 5, 10], -3) == 2


def test_find_insertion_index_two_elements():
    assert find_insertion_index([1, 3], 2) == 1


def find_first_and_last(nums: list[int], target: int) -> list[int]:
    """
    First and Last Occurrences of a Number

    Given an array sorted in non-decreasing order, return the first and last
    indexes of a target number. If not found, return [-1, -1].

    Example:
        >>> find_first_and_last([1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], 4)
        [3, 5]

    Time Complexity: O(log n) - two binary searches
    Space Complexity: O(1) - only pointers

    Run binary search twice: once to find leftmost occurrence (lower bound),
    once to find rightmost occurrence (upper bound).
    """
    if not nums:
        return [-1, -1]

    # find_lower: find FIRST occurrence
    # Example: [1, 2, 4, 4, 4, 5], target=4
    #   mid lands on 4 -> could be first, so right=mid (keep it)
    #   keeps shrinking right until left==right at first 4
    def find_lower():
        left, right = 0, len(nums) - 1
        while left < right:
            mid = (left + right) // 2
            if nums[mid] < target:
                left = mid + 1
            else:
                right = mid  # mid could be answer, keep it
        return left if nums[left] == target else -1

    # find_upper: find LAST occurrence
    # Example: [1, 2, 4, 4, 4, 5], target=4
    #   mid lands on 4 -> could be last, so left=mid (keep it)
    #   keeps shrinking left until left==right at last 4
    #
    # Why +1? Without it, when left=3, right=4:
    #   mid = (3+4)//2 = 3, then left=mid=3 -> infinite loop!
    # With +1:
    #   mid = (3+4)//2 + 1 = 4, then left=mid=4 -> left==right, done
    def find_upper():
        left, right = 0, len(nums) - 1
        while left < right:
            mid = (left + right) // 2 + 1  # bias right to avoid infinite loop
            if nums[mid] > target:
                right = mid - 1
            else:
                left = mid
        return left if nums[left] == target else -1

    return [find_lower(), find_upper()]


# -----------------------------------------------------------------------------
# Tests for find_first_and_last
# -----------------------------------------------------------------------------

def test_find_first_and_last_example():
    assert find_first_and_last([1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], 4) == [3, 5]


def test_find_first_and_last_not_found():
    assert find_first_and_last([1, 2, 3, 5, 6], 4) == [-1, -1]


def test_find_first_and_last_single_occurrence():
    assert find_first_and_last([1, 2, 3, 4, 5], 3) == [2, 2]


def test_find_first_and_last_all_same():
    assert find_first_and_last([4, 4, 4, 4, 4], 4) == [0, 4]


def test_find_first_and_last_at_start():
    assert find_first_and_last([1, 1, 1, 2, 3, 4], 1) == [0, 2]


def test_find_first_and_last_at_end():
    assert find_first_and_last([1, 2, 3, 4, 4, 4], 4) == [3, 5]


def test_find_first_and_last_empty():
    assert find_first_and_last([], 4) == [-1, -1]


def test_find_first_and_last_single_found():
    assert find_first_and_last([4], 4) == [0, 0]


def test_find_first_and_last_single_not_found():
    assert find_first_and_last([5], 4) == [-1, -1]


def test_find_first_and_last_two_occurrences():
    assert find_first_and_last([1, 2, 2, 3], 2) == [1, 2]


def cutting_wood(heights: list[int], k: int) -> int:
    """
    Cutting Wood

    Given tree heights and amount of wood needed (k), find the highest
    sawblade height that collects at least k wood. Each tree taller than
    the blade contributes (tree_height - blade_height) wood.

    Example:
        >>> cutting_wood([2, 6, 3, 8], 7)
        3  # Cut at height 3: (6-3)+(8-3) = 3+5 = 8 >= 7

    Time Complexity: O(n log h) - binary search on height, O(n) check each
    Space Complexity: O(1)

    Binary search on the answer. Search for highest blade height that
    still gives enough wood.
    """
    # Instead of searching for a value in an array, we search for the
    # best answer in a range [0, max_height].
    #
    # Key insight: this is monotonic!
    #   - Lower blade = more wood collected
    #   - Higher blade = less wood collected
    #   - If blade=3 gives enough, blade=2 also gives enough
    #   - If blade=5 doesn't give enough, blade=6 also won't
    #
    # Example: heights=[2,6,3,8], k=7
    #   blade=0: (2-0)+(6-0)+(3-0)+(8-0) = 19 wood (enough, but wasteful)
    #   blade=3: (6-3)+(8-3) = 8 wood (enough!)
    #   blade=4: (6-4)+(8-4) = 6 wood (not enough)
    #   Answer: 3 (highest blade that still works)

    def has_enough_wood(blade_height: int) -> bool:
        wood = 0
        for h in heights:
            if h > blade_height:
                wood += h - blade_height
        return wood >= k

    left = 0
    right = max(heights)

    while left < right:
        mid = (left + right) // 2 + 1  # +1 bias: we want highest valid answer
        if has_enough_wood(mid):
            left = mid      # enough wood, try going higher
        else:
            right = mid - 1  # not enough, blade too high

    return left


# -----------------------------------------------------------------------------
# Tests for cutting_wood
# -----------------------------------------------------------------------------

def test_cutting_wood_example():
    assert cutting_wood([2, 6, 3, 8], 7) == 3


def test_cutting_wood_exact_cut():
    assert cutting_wood([5, 5, 5, 5], 4) == 4


def test_cutting_wood_all_same_height():
    assert cutting_wood([10, 10, 10], 15) == 5


def test_cutting_wood_single_tree():
    assert cutting_wood([10], 5) == 5


def test_cutting_wood_large_k():
    # blade=10: 10+5+0+7=22 (not enough), blade=9: 11+6+1+8=26 (enough)
    assert cutting_wood([20, 15, 10, 17], 24) == 9


def find_in_rotated_array(nums: list[int], target: int) -> int:
    """
    Find Target in a Rotated Sorted Array

    A rotated sorted array is a sorted array where a portion is moved from
    the beginning to the end. Example: [1,2,3,4,5] -> [3,4,5,1,2]

    Given a rotated sorted array of unique numbers, return the index of target.
    Return -1 if not found.

    Example:
        >>> find_in_rotated_array([8, 9, 1, 2, 3, 4, 5, 6, 7], 1)
        2

    Time Complexity: O(log n) - binary search
    Space Complexity: O(1)

    Key insight: At any mid point, ONE half is always sorted:
        [8, 9, 1, 2, 3, 4, 5, 6, 7]
              ^mid
        Left half [8,9,1] is NOT sorted (8 > 1)
        Right half [2,3,4,5,6,7] IS sorted (2 < 7)

    Strategy:
        1. Find which half is sorted (compare endpoints)
        2. Check if target is in the sorted half (easy range check)
        3. If yes, search that half. If no, search the other half.
    """
    if not nums:
        return -1

    left, right = 0, len(nums) - 1

    while left < right:
        mid = (left + right) // 2

        # Found it!
        if nums[mid] == target:
            return mid

        # Check if LEFT half is sorted (left <= mid value)
        if nums[left] <= nums[mid]:
            # Target in sorted left half? (between left and mid)
            if nums[left] <= target < nums[mid]:
                right = mid - 1
            else:
                left = mid + 1
        # Otherwise RIGHT half is sorted
        else:
            # Target in sorted right half? (between mid and right)
            if nums[mid] < target <= nums[right]:
                left = mid + 1
            else:
                right = mid - 1

    # Check final position
    return left if nums[left] == target else -1


# -----------------------------------------------------------------------------
# Tests for find_in_rotated_array
# -----------------------------------------------------------------------------

def test_find_in_rotated_array_example():
    assert find_in_rotated_array([8, 9, 1, 2, 3, 4, 5, 6, 7], 1) == 2


def test_find_in_rotated_array_not_found():
    assert find_in_rotated_array([8, 9, 1, 2, 3, 4, 5, 6, 7], 10) == -1


def test_find_in_rotated_array_first_element():
    assert find_in_rotated_array([4, 5, 6, 7, 0, 1, 2], 4) == 0


def test_find_in_rotated_array_last_element():
    assert find_in_rotated_array([4, 5, 6, 7, 0, 1, 2], 2) == 6


def test_find_in_rotated_array_pivot_element():
    assert find_in_rotated_array([4, 5, 6, 7, 0, 1, 2], 7) == 3


def test_find_in_rotated_array_no_rotation():
    assert find_in_rotated_array([1, 2, 3, 4, 5], 3) == 2


def test_find_in_rotated_array_single_found():
    assert find_in_rotated_array([5], 5) == 0


def test_find_in_rotated_array_single_not_found():
    assert find_in_rotated_array([5], 3) == -1


def test_find_in_rotated_array_two_elements():
    assert find_in_rotated_array([2, 1], 1) == 1


def test_find_in_rotated_array_target_in_left_half():
    assert find_in_rotated_array([6, 7, 8, 1, 2, 3, 4, 5], 8) == 2


def test_find_in_rotated_array_target_in_right_half():
    assert find_in_rotated_array([6, 7, 8, 1, 2, 3, 4, 5], 3) == 5


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
