"""
Heaps Pattern

A collection of algorithm problems using heaps (priority queues).

Python's heapq is a MIN heap by default.
For a max heap, negate the values: push -val, pop gives -val (flip back).

heapq operations and their time complexities:
    heapq.heappush(heap, val)     O(log n)  - add element, bubble up
    heapq.heappop(heap)           O(log n)  - remove min, bubble down
    heapq.heapify(list)           O(n)      - build heap from list (NOT n log n!)
    heapq.heapreplace(heap, val)  O(log n)  - pop min + push new in one call
    heap[0]                       O(1)      - peek at min without removing

Common uses:
- kth largest/smallest → min heap of size k
- Median stream → two heaps (max left, min right)
- Merge k sorted → min heap of heads
"""

import heapq
from collections import Counter


def kth_largest(nums: list[int], k: int) -> int:
    """
    Kth Largest Integer

    Return the kth largest integer in an array.

    Example:
        >>> kth_largest([5, 2, 4, 3, 1, 6], 3)
        4
        # Sorted desc: [6, 5, 4, 3, 2, 1] → 3rd largest is 4

    Time Complexity: O(n log k) - heap operations
    Space Complexity: O(k) - heap size

    Keep a min heap of size k (the "top k" bucket).
    The smallest in the bucket = kth largest overall.

    Example k=3 on [5, 2, 4, 3, 1, 6]:
        5 → bucket: [5]           not full yet
        2 → bucket: [2, 5]       not full yet
        4 → bucket: [2, 4, 5]    full! (size k=3)
        3 → 3 > min(2)? yes → kick 2, add 3 → bucket: [3, 4, 5]
        1 → 1 > min(3)? no → skip
        6 → 6 > min(3)? yes → kick 3, add 6 → bucket: [4, 5, 6]
        Answer: min of bucket = 4
    """
    heap = []  # min heap

    for num in nums:
        if len(heap) < k:
            heapq.heappush(heap, num)
        elif num > heap[0]:
            heapq.heapreplace(heap, num)  # pop min, push num

    return heap[0]


# -----------------------------------------------------------------------------
# Tests for kth_largest
# -----------------------------------------------------------------------------

def test_kth_largest_example():
    assert kth_largest([5, 2, 4, 3, 1, 6], 3) == 4


def test_kth_largest_first():
    assert kth_largest([5, 2, 4, 3, 1, 6], 1) == 6


def test_kth_largest_last():
    assert kth_largest([5, 2, 4, 3, 1, 6], 6) == 1


def test_kth_largest_single():
    assert kth_largest([42], 1) == 42


def test_kth_largest_duplicates():
    assert kth_largest([3, 2, 3, 1, 2, 4, 5, 5, 6], 4) == 4


def test_kth_largest_negative():
    assert kth_largest([-1, -5, 0, 3, -2], 2) == 0


class MedianFinder:
    """
    Median of an Integer Stream

    Design a data structure that supports:
        - add(num): adds an integer
        - get_median(): returns the median of all integers so far

    Example:
        >>> mf = MedianFinder()
        >>> mf.add(3); mf.add(6)
        >>> mf.get_median()
        4.5
        >>> mf.add(1)
        >>> mf.get_median()
        3.0

    Time Complexity: O(log n) per add, O(1) per get_median
    Space Complexity: O(n)

    Two heaps split the data into smaller and larger halves:

        left (max heap): smaller half    right (min heap): larger half

        Example after adding [1, 3, 6]:
            left: [1, 3]   right: [6]
            max=3           min=6

        The median is always at the boundary between the two heaps!

        Rules:
            - left.size == right.size  OR  left.size == right.size + 1
            - Everything in left <= everything in right
            - Median = top of left (odd count), avg of both tops (even)

    Python only has min heap, so left uses NEGATED values for max heap.
    push -5 into min heap → acts like pushing 5 into max heap.

    Example walkthrough: add(3), add(6), add(1)
        add(3): left=[-3], right=[]          → left has 3
        add(6): 6 > 3, goes right            → left=[-3], right=[6]
        add(1): 1 <= 3, goes left            → left=[-3,-1], right=[6]
                left has 2 more, rebalance!  → left=[-1], right=[3,6]
                wait that's wrong... → left=[-3], right=[1,6]? no...

        Actually: add(1): 1 <= max(left)=3 → push to left
                  left=[-3,-1] (max heap: 3,1), right=[6]
                  left.size(2) - right.size(1) = 1, ok no rebalance
                  median = top of left = 3 ✓

    """

    def __init__(self):
        # left = max heap (stores negated values since Python only has min heap)
        # right = min heap
        self.left = []
        self.right = []

    def add(self, num: int):
        # Decide which heap to push to
        if not self.left or num <= -self.left[0]:
            # num belongs in the smaller half
            heapq.heappush(self.left, -num)

            # Rebalance: left can be at most 1 bigger than right
            if len(self.left) - len(self.right) > 1:
                # Move left's max to right
                heapq.heappush(self.right, -heapq.heappop(self.left))
        else:
            # num belongs in the larger half
            heapq.heappush(self.right, num)

            # Rebalance: right can never be bigger than left
            if len(self.right) > len(self.left):
                # Move right's min to left
                heapq.heappush(self.left, -heapq.heappop(self.right))

    def get_median(self) -> float:
        if len(self.left) == len(self.right):
            # Even count: average of both tops
            return (-self.left[0] + self.right[0]) / 2.0
        # Odd count: left always has the extra element
        return float(-self.left[0])


# -----------------------------------------------------------------------------
# Tests for MedianFinder
# -----------------------------------------------------------------------------

def test_median_finder_example():
    mf = MedianFinder()
    mf.add(3)
    mf.add(6)
    assert mf.get_median() == 4.5
    mf.add(1)
    assert mf.get_median() == 3.0


def test_median_finder_single():
    mf = MedianFinder()
    mf.add(5)
    assert mf.get_median() == 5.0


def test_median_finder_two_elements():
    mf = MedianFinder()
    mf.add(1)
    mf.add(2)
    assert mf.get_median() == 1.5


def test_median_finder_odd_count():
    mf = MedianFinder()
    for n in [1, 2, 3, 4, 5]:
        mf.add(n)
    assert mf.get_median() == 3.0


def test_median_finder_even_count():
    mf = MedianFinder()
    for n in [1, 2, 3, 4]:
        mf.add(n)
    assert mf.get_median() == 2.5


def test_median_finder_negative():
    mf = MedianFinder()
    for n in [-5, -3, -1]:
        mf.add(n)
    assert mf.get_median() == -3.0


def test_median_finder_duplicates():
    mf = MedianFinder()
    for n in [5, 5, 5]:
        mf.add(n)
    assert mf.get_median() == 5.0


def k_most_frequent_strings(strs: list[str], k: int) -> list[str]:
    """
    K Most Frequent Strings

    Find the k most frequent strings. Sort by frequency (desc),
    then lexicographically for ties.

    Example:
        >>> k_most_frequent_strings(["go","coding","byte","byte","go","interview","go"], 2)
        ["go", "byte"]
        # go: 3 times, byte: 2 times

    Time Complexity: O(n log n) - heap/sort
    Space Complexity: O(n) - counter
    """
    counts = Counter(strs)
    # Sort by frequency desc, then alphabetically for ties
    # Use heap: (-count, name) so highest freq pops first, and alpha breaks ties
    heap = [(-count, name) for name, count in counts.items()]
    heapq.heapify(heap)
    return [heapq.heappop(heap)[1] for _ in range(k)]


# -----------------------------------------------------------------------------
# Tests for k_most_frequent_strings
# -----------------------------------------------------------------------------

def test_k_most_frequent_strings_example():
    assert k_most_frequent_strings(["go", "coding", "byte", "byte", "go", "interview", "go"], 2) == ["go", "byte"]


def test_k_most_frequent_strings_single():
    assert k_most_frequent_strings(["a", "b", "c", "a", "a"], 1) == ["a"]


def test_k_most_frequent_strings_all_same_freq():
    assert k_most_frequent_strings(["c", "b", "a"], 2) == ["a", "b"]


def test_k_most_frequent_strings_k_equals_n():
    assert k_most_frequent_strings(["x", "y", "z"], 3) == ["x", "y", "z"]


def test_k_most_frequent_strings_tie_breaker():
    assert k_most_frequent_strings(["apple", "banana", "apple", "banana", "cherry"], 2) == ["apple", "banana"]


def test_k_most_frequent_strings_k_zero():
    assert k_most_frequent_strings(["a", "b", "c"], 0) == []


def test_k_most_frequent_strings_all_same():
    assert k_most_frequent_strings(["same", "same", "same"], 1) == ["same"]


def test_k_most_frequent_strings_mixed():
    assert k_most_frequent_strings(["a", "a", "a", "a", "b", "b", "b", "c", "c", "d"], 3) == ["a", "b", "c"]


def combine_sorted_lists(lists: list[list[int]]) -> list[int]:
    """
    Combine K Sorted Lists

    Given k sorted lists, combine them into one sorted list.

    Example:
        >>> combine_sorted_lists([[1, 3, 5], [2, 4, 6], [0, 7, 8]])
        [0, 1, 2, 3, 4, 5, 6, 7, 8]

    Time Complexity: O(n log k) - n total elements, k lists
    Space Complexity: O(k) - heap holds one element per list

    Push first element of each list into min heap.
    Pop smallest, push the next element from that same list.
    Heap entries are (value, list_index, element_index) to break ties.
    """
    heap = []
    # Push first element of each list: (value, list_idx, elem_idx)
    for i, lst in enumerate(lists):
        if lst:
            heapq.heappush(heap, (lst[0], i, 0))

    result = []
    while heap:
        val, list_idx, elem_idx = heapq.heappop(heap)
        result.append(val)
        # Push next element from same list
        if elem_idx + 1 < len(lists[list_idx]):
            next_val = lists[list_idx][elem_idx + 1]
            heapq.heappush(heap, (next_val, list_idx, elem_idx + 1))

    return result


# -----------------------------------------------------------------------------
# Tests for combine_sorted_lists
# -----------------------------------------------------------------------------

def test_combine_sorted_lists_example():
    assert combine_sorted_lists([[1, 3, 5], [2, 4, 6], [0, 7, 8]]) == [0, 1, 2, 3, 4, 5, 6, 7, 8]


def test_combine_sorted_lists_empty():
    assert combine_sorted_lists([]) == []


def test_combine_sorted_lists_all_empty():
    assert combine_sorted_lists([[], [], []]) == []


def test_combine_sorted_lists_single():
    assert combine_sorted_lists([[1, 2, 3]]) == [1, 2, 3]


def test_combine_sorted_lists_two():
    assert combine_sorted_lists([[1, 3, 5], [2, 4, 6]]) == [1, 2, 3, 4, 5, 6]


def test_combine_sorted_lists_duplicates():
    assert combine_sorted_lists([[1, 2, 2], [1, 1, 2]]) == [1, 1, 1, 2, 2, 2]


def test_combine_sorted_lists_negative():
    assert combine_sorted_lists([[-3, -1, 2], [-2, 0, 3]]) == [-3, -2, -1, 0, 2, 3]


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
