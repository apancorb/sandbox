"""
Sort and Search Pattern

A collection of sorting and searching algorithms.

Key algorithms:
- Quicksort: O(n log n) avg, O(n²) worst, in-place
- Merge sort: O(n log n) always, not in-place
- Heap-based selection: O(n log k) for kth element
"""


def sort_array(nums: list[int]) -> list[int]:
    """
    Sort Array (Quicksort)

    Sort an integer array in ascending order using quicksort.

    Example:
        >>> sort_array([6, 8, 4, 2, 7, 3, 1, 5])
        [1, 2, 3, 4, 5, 6, 7, 8]

    Time Complexity: O(n log n) average, O(n²) worst
    Space Complexity: O(log n) average (recursion stack)

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

    Time Complexity: O(m + n)
    Space Complexity: O(m) - copy of nums1

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

    Time Complexity: O(n log n) - sorting
    Space Complexity: O(n) - sorted copy

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
