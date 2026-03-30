"""
Searching and Sorting Pattern

A collection of searching and sorting algorithm problems.

Sections:
- Binary Search: problems using binary search on sorted arrays and answer ranges
- Sorting: fundamental sorting algorithms and merge operations

Binary search rule of thumb:
    If you're setting left = mid (keeping mid as a candidate), you MUST use +1.
    If you're setting right = mid (keeping mid as a candidate), you DON'T need +1.

Key algorithms:
- Quicksort: O(n log n) avg, O(n²) worst, in-place
- Binary search: O(log n) for sorted data
"""


# =============================================================================
# Binary Search
# =============================================================================


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

    Standard binary search. When target not found, left pointer ends up
    at the insertion position.

    [1, 2, 4, 5, 7, 8, 9], target=6:
        left=0, right=7, mid=3 → nums[3]=5 < 6 → left=4
        left=4, right=7, mid=5 → nums[5]=8 >= 6 → right=5
        left=4, right=5, mid=4 → nums[4]=7 >= 6 → right=4
        left==right=4 → return 4 (between 5 and 7)

    Time Complexity: O(log n) - binary search
    Space Complexity: O(1) - only pointers
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

    Run binary search twice: once to find leftmost occurrence (lower bound),
    once to find rightmost occurrence (upper bound).

    [1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], target=4:
        find_lower: narrows right when nums[mid] >= 4
            mid=6 → 5 >= 4, right=6 → mid=3 → 4 >= 4, right=3
            → left==right=3, nums[3]=4 ✓ → first=3
        find_upper: narrows left when nums[mid] <= 4 (with +1 bias)
            mid=7 → 6 > 4, right=6 → mid=3 → 4 <= 4, left=3
            mid=5 → 4 <= 4, left=5 → mid=6 → 5 > 4, right=5
            → left==right=5, nums[5]=4 ✓ → last=5
        Result: [3, 5]

    Time Complexity: O(log n) - two binary searches
    Space Complexity: O(1) - only pointers
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

    Binary search on the answer. Search for highest blade height that
    still gives enough wood. This is monotonic: lower blade means more
    wood, higher blade means less wood.

    heights=[2, 6, 3, 8], k=7, search range [0, 8]:
        mid=5: wood = (6-5)+(8-5) = 1+3 = 4 < 7   → right=4
        mid=3: wood = (6-3)+(8-3) = 3+5 = 8 >= 7  → left=3
        mid=4: wood = (6-4)+(8-4) = 2+4 = 6 < 7   → right=3
        left==right=3 → answer is 3

    Time Complexity: O(n log h) - binary search on height, O(n) check each
    Space Complexity: O(1)
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

    Key insight: At any mid point, ONE half is always sorted:
        [8, 9, 1, 2, 3, 4, 5, 6, 7]
              ^mid
        Left half [8,9,1] is NOT sorted (8 > 1)
        Right half [2,3,4,5,6,7] IS sorted (2 < 7)

    Strategy:
        1. Find which half is sorted (compare endpoints)
        2. Check if target is in the sorted half (easy range check)
        3. If yes, search that half. If no, search the other half.

    Time Complexity: O(log n) - binary search
    Space Complexity: O(1)
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


# =============================================================================
# Sorting
# =============================================================================


def sort_array(nums: list[int]) -> list[int]:
    """
    Sort Array (Quicksort)

    Sort an integer array in ascending order using quicksort.

    Example:
        >>> sort_array([6, 8, 4, 2, 7, 3, 1, 5])
        [1, 2, 3, 4, 5, 6, 7, 8]

    Quicksort steps:
        1. Pick a pivot (we use last element)
        2. Partition: move smaller elements left, larger right
        3. Recursively sort left and right partitions

    Partition example: [6,8,4,2,7,3,1,5], pivot=5

        lo marks where the next "small" element should go.
        Scan left to right, when we find something < pivot, swap it to lo.

        [6, 8, 4, 2, 7, 3, 1, 5]    pivot=5
         ^lo
         i=0: 6 >= 5? yes, skip

        [6, 8, 4, 2, 7, 3, 1, 5]
         ^lo
         i=1: 8 >= 5? yes, skip

        [6, 8, 4, 2, 7, 3, 1, 5]
         ^lo
         i=2: 4 < 5? yes! swap arr[lo] with arr[i]

        [4, 8, 6, 2, 7, 3, 1, 5]    swapped 6↔4
            ^lo
         i=3: 2 < 5? yes! swap

        [4, 2, 6, 8, 7, 3, 1, 5]    swapped 8↔2
               ^lo
         i=4: 7 >= 5? yes, skip
         i=5: 3 < 5? yes! swap

        [4, 2, 3, 8, 7, 6, 1, 5]    swapped 6↔3
                  ^lo
         i=6: 1 < 5? yes! swap

        [4, 2, 3, 1, 7, 6, 8, 5]    swapped 8↔1
                     ^lo
         Done scanning. Now put pivot at lo:

        [4, 2, 3, 1, 5, 6, 8, 7]    swapped 7↔5
         ←--<5---→   ↑  ←->=5-→
                   pivot in final position!

        Recurse on left [4,2,3,1] and right [6,8,7]

    Time Complexity: O(n log n) average, O(n²) worst
    Space Complexity: O(log n) average (recursion stack)
    """
    def quicksort(left: int, right: int):
        if left >= right:
            return

        # Partition and get pivot's final position
        pivot_idx = partition(left, right)

        # Recursively sort left and right of pivot
        quicksort(left, pivot_idx - 1)
        quicksort(pivot_idx + 1, right)

    def partition(left: int, right: int) -> int:
        pivot = nums[right]  # choose last element as pivot
        lo = left            # boundary for "less than pivot" section

        # Move all elements < pivot to the left
        for i in range(left, right):
            if nums[i] < pivot:
                nums[lo], nums[i] = nums[i], nums[lo]
                lo += 1

        # Put pivot in its correct position
        nums[lo], nums[right] = nums[right], nums[lo]
        return lo

    if nums:
        quicksort(0, len(nums) - 1)
    return nums


# -----------------------------------------------------------------------------
# Tests for sort_array
# -----------------------------------------------------------------------------

def test_sort_array_example():
    assert sort_array([6, 8, 4, 2, 7, 3, 1, 5]) == [1, 2, 3, 4, 5, 6, 7, 8]


def test_sort_array_empty():
    assert sort_array([]) == []


def test_sort_array_single():
    assert sort_array([5]) == [5]


def test_sort_array_already_sorted():
    assert sort_array([1, 2, 3, 4, 5]) == [1, 2, 3, 4, 5]


def test_sort_array_reverse():
    assert sort_array([5, 4, 3, 2, 1]) == [1, 2, 3, 4, 5]


def test_sort_array_duplicates():
    assert sort_array([3, 1, 2, 1, 3]) == [1, 1, 2, 3, 3]


def test_sort_array_negative():
    assert sort_array([3, -1, 0, -5, 2]) == [-5, -1, 0, 2, 3]


def merge_sorted_array(nums1: list[int], m: int, nums2: list[int], n: int) -> None:
    """
    Merge Sorted Array

    Merge nums2 into nums1. nums1 has length m+n (last n elements are 0s).
    Modify nums1 in-place.

    Example:
        >>> nums1 = [1, 2, 3, 0, 0, 0]
        >>> merge_sorted_array(nums1, 3, [2, 5, 6], 3)
        >>> nums1
        [1, 2, 2, 3, 5, 6]

    Two pointer merge: compare elements from both arrays,
    pick the smaller one each time.

    Example: nums1=[1,3,5,0,0,0] m=3, nums2=[2,4,6] n=3
        copy1 = [1,3,5]
        i=0: 1 <= 2 → take 1
        i=1: 3 > 2  → take 2
        i=2: 3 <= 4 → take 3
        i=3: 5 > 4  → take 4
        i=4: 5 <= 6 → take 5
        i=5: done   → take 6
        Result: [1,2,3,4,5,6]

    Time Complexity: O(m + n)
    Space Complexity: O(m) - copy of nums1
    """
    # Copy nums1's actual values (first m elements)
    copy1 = nums1[:m]

    p1, p2 = 0, 0  # pointers for copy1 and nums2

    for i in range(m + n):
        # Take from copy1 if: nums2 exhausted OR copy1 has smaller value
        if p2 >= n or (p1 < m and copy1[p1] <= nums2[p2]):
            nums1[i] = copy1[p1]
            p1 += 1
        else:
            nums1[i] = nums2[p2]
            p2 += 1


# -----------------------------------------------------------------------------
# Tests for merge_sorted_array
# -----------------------------------------------------------------------------

def test_merge_sorted_array_example():
    nums1 = [1, 2, 3, 0, 0, 0]
    merge_sorted_array(nums1, 3, [2, 5, 6], 3)
    assert nums1 == [1, 2, 2, 3, 5, 6]


def test_merge_sorted_array_nums2_empty():
    nums1 = [1, 2, 3]
    merge_sorted_array(nums1, 3, [], 0)
    assert nums1 == [1, 2, 3]


def test_merge_sorted_array_nums1_empty():
    nums1 = [0]
    merge_sorted_array(nums1, 0, [1], 1)
    assert nums1 == [1]


def test_merge_sorted_array_interleaved():
    nums1 = [1, 3, 5, 0, 0, 0]
    merge_sorted_array(nums1, 3, [2, 4, 6], 3)
    assert nums1 == [1, 2, 3, 4, 5, 6]


def test_merge_sorted_array_nums2_all_smaller():
    nums1 = [4, 5, 6, 0, 0, 0]
    merge_sorted_array(nums1, 3, [1, 2, 3], 3)
    assert nums1 == [1, 2, 3, 4, 5, 6]


def h_index(citations: list[int]) -> int:
    """
    H-Index

    Given an array where citations[i] is the number of citations for the ith paper,
    return the h-index: the maximum h such that at least h papers have at least
    h citations each.

    Example:
        >>> h_index([3, 0, 6, 1, 5])
        3
        # Sorted: [0, 1, 3, 5, 6]
        # 3 papers have >= 3 citations (papers with 3, 5, 6)

    Sort ascending then scan. At index i, there are (n - i) papers
    with at least citations[i] citations. If citations[i] >= (n - i),
    that's a valid h-index.

    Example walkthrough: [3, 0, 6, 1, 5] → sorted: [0, 1, 3, 5, 6]
        i=0: citations=0, papers_left=5 → 0 >= 5? no
        i=1: citations=1, papers_left=4 → 1 >= 4? no
        i=2: citations=3, papers_left=3 → 3 >= 3? yes! h=3
        i=3: citations=5, papers_left=2 → 5 >= 2? yes! h=max(3,2)=3
        i=4: citations=6, papers_left=1 → 6 >= 1? yes! h=max(3,1)=3
        Answer: 3

    Time Complexity: O(n log n) - sorting
    Space Complexity: O(n) - sorted copy
    """
    sorted_cit = sorted(citations)
    n = len(sorted_cit)
    h = 0

    for i in range(n):
        papers_with_at_least = n - i
        if sorted_cit[i] >= papers_with_at_least:
            h = max(h, papers_with_at_least)

    return h


# -----------------------------------------------------------------------------
# Tests for h_index
# -----------------------------------------------------------------------------

def test_h_index_example1():
    assert h_index([3, 0, 6, 1, 5]) == 3


def test_h_index_example2():
    assert h_index([1, 3, 1]) == 1


def test_h_index_all_zeros():
    assert h_index([0, 0, 0]) == 0


def test_h_index_single_high():
    assert h_index([100]) == 1


def test_h_index_single_zero():
    assert h_index([0]) == 0


def test_h_index_all_same():
    assert h_index([5, 5, 5, 5, 5]) == 5


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
