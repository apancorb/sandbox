"""
Intervals Pattern

A collection of algorithm problems involving interval manipulation.

Common techniques:
- Sort by start time to process intervals in order
- Line sweep: convert intervals to events (start/end points)
- Two pointers for comparing two sorted interval lists
"""


def merge_intervals(intervals: list[list[int]]) -> list[list[int]]:
    """
    Merge Overlapping Intervals

    Merge an array of intervals so there are no overlapping intervals.

    Example:
        >>> merge_intervals([[3, 4], [7, 8], [2, 5], [6, 7], [1, 4]])
        [[1, 5], [6, 8]]

    Sort intervals by start time, then iterate through. For each interval,
    if it overlaps with the last merged interval (current start <= previous
    end), extend the previous end. Otherwise, add it as a new interval.

    Walkthrough for [[3,4], [7,8], [2,5], [6,7], [1,4]]:
        After sorting: [[1,4], [2,5], [3,4], [6,7], [7,8]]

        merged = [[1,4]]
        [2,5]: 2 <= 4 → overlap, extend → [[1,5]]
        [3,4]: 3 <= 5 → overlap, max(5,4)=5 → [[1,5]]
        [6,7]: 6 > 5 → no overlap → [[1,5], [6,7]]
        [7,8]: 7 <= 7 → overlap, extend → [[1,5], [6,8]]

        Answer: [[1,5], [6,8]]

    Time Complexity: O(n log n) - dominated by sorting
    Space Complexity: O(n) - for the result
    """
    # Sort by start time
    intervals.sort(key=lambda x: x[0])

    merged = [intervals[0]]

    for interval in intervals[1:]:
        last = merged[-1]
        # Overlapping? (current start <= last end)
        if interval[0] <= last[1]:
            # Merge: extend the end if needed
            last[1] = max(last[1], interval[1])
        else:
            # No overlap: add as new interval
            merged.append(interval)

    return merged


# -----------------------------------------------------------------------------
# Tests for merge_intervals
# -----------------------------------------------------------------------------

def test_merge_intervals_example():
    assert merge_intervals([[3, 4], [7, 8], [2, 5], [6, 7], [1, 4]]) == [[1, 5], [6, 8]]


def test_merge_intervals_no_overlap():
    assert merge_intervals([[1, 2], [4, 5], [7, 8]]) == [[1, 2], [4, 5], [7, 8]]


def test_merge_intervals_all_overlap():
    assert merge_intervals([[1, 5], [2, 6], [3, 7]]) == [[1, 7]]


def test_merge_intervals_single():
    assert merge_intervals([[1, 5]]) == [[1, 5]]


def test_merge_intervals_touching():
    # [1, 3] and [3, 5] touch at 3, should merge
    assert merge_intervals([[1, 3], [3, 5]]) == [[1, 5]]


def test_merge_intervals_contained():
    # [2, 4] is fully contained in [1, 5]
    assert merge_intervals([[1, 5], [2, 4]]) == [[1, 5]]


def test_merge_intervals_unsorted():
    assert merge_intervals([[5, 6], [1, 2], [3, 4]]) == [[1, 2], [3, 4], [5, 6]]


def test_merge_intervals_negative():
    assert merge_intervals([[-5, -2], [-3, 0], [1, 3]]) == [[-5, 0], [1, 3]]


def max_overlapping(intervals: list[list[int]]) -> int:
    """
    Largest Overlap of Intervals

    Given an array of intervals, determine the maximum number of intervals
    that overlap at any point. Each interval is half-open [start, end),
    meaning it includes start but excludes end.

    Example:
        >>> max_overlapping([[1, 3], [2, 6], [4, 8], [6, 7], [5, 7]])
        3

    Line Sweep Algorithm: convert each interval to two events (START and
    END), sort by time, then sweep through counting active intervals. At
    each start event add one, at each end event subtract one. The maximum
    count seen during the sweep is the answer.

    Walkthrough for [[1,3], [2,6], [4,8]]:
        Events: [(1,'S'), (2,'S'), (3,'E'), (4,'S'), (6,'E'), (8,'E')]

        Time 1, S: current=1  ← [1,3] starts
        Time 2, S: current=2  ← [2,6] starts (2 active now!)
        Time 3, E: current=1  ← [1,3] ends
        Time 4, S: current=2  ← [4,8] starts
        Time 6, E: current=1  ← [2,6] ends
        Time 8, E: current=0  ← [4,8] ends

        Max was 2 (at times 2-3 and 4-6)

    Time Complexity: O(n log n) - sorting the events
    Space Complexity: O(n) - storing events
    """
    events = []

    # Create events: 'S' for start, 'E' for end
    # We need labels to know if we're entering (+1) or leaving (-1) an interval
    for start, end in intervals:
        events.append((start, 'S'))
        events.append((end, 'E'))

    # Sort by time. At same time, 'E' comes before 'S' (half-open intervals)
    # 'E' < 'S' alphabetically, so this works naturally
    events.sort()

    max_overlap = 0
    current = 0

    for time, event_type in events:
        if event_type == 'S':
            current += 1
        else:
            current -= 1
        max_overlap = max(max_overlap, current)

    return max_overlap


# -----------------------------------------------------------------------------
# Tests for max_overlapping
# -----------------------------------------------------------------------------

def test_max_overlapping_example():
    assert max_overlapping([[1, 3], [2, 6], [4, 8], [6, 7], [5, 7]]) == 3


def test_max_overlapping_single():
    assert max_overlapping([[1, 5]]) == 1


def test_max_overlapping_no_overlap():
    assert max_overlapping([[1, 2], [3, 4], [5, 6]]) == 1


def test_max_overlapping_all_overlap():
    assert max_overlapping([[1, 10], [2, 9], [3, 8], [4, 7]]) == 4


def test_max_overlapping_touching_half_open():
    # Half-open: [1, 3) and [3, 5) don't overlap at point 3
    assert max_overlapping([[1, 3], [3, 5]]) == 1


def test_max_overlapping_same_start():
    assert max_overlapping([[1, 5], [1, 3], [1, 4]]) == 3


def test_max_overlapping_same_end():
    assert max_overlapping([[1, 5], [2, 5], [3, 5]]) == 3


def find_overlaps(intervals1: list[list[int]], intervals2: list[list[int]]) -> list[list[int]]:
    """
    Identify All Interval Overlaps

    Return all overlaps between two arrays of intervals. Each input array
    is sorted by start value and contains no overlapping intervals within itself.

    Example:
        >>> find_overlaps([[1, 4], [5, 6], [9, 10]], [[2, 7], [8, 9]])
        [[2, 4], [5, 6], [9, 9]]

    Use two pointers, one for each list. Compare the current intervals: if
    they overlap, record the intersection as [max(starts), min(ends)]. Then
    advance whichever pointer's interval ends first, since that interval
    can't overlap with anything further in the other list.

    Walkthrough for intervals1=[[1,4],[5,6],[9,10]], intervals2=[[2,7],[8,9]]:
        i=0, j=0: [1,4] vs [2,7] → overlap? 1<=7 and 2<=4 → yes
                   intersection = [max(1,2), min(4,7)] = [2,4]
                   4 <= 7 → advance i
        i=1, j=0: [5,6] vs [2,7] → overlap? 5<=7 and 2<=6 → yes
                   intersection = [max(5,2), min(6,7)] = [5,6]
                   6 <= 7 → advance i
        i=2, j=0: [9,10] vs [2,7] → overlap? 9<=7? no
                   7 <= 10 → advance j
        i=2, j=1: [9,10] vs [8,9] → overlap? 9<=9 and 8<=10 → yes
                   intersection = [max(9,8), min(10,9)] = [9,9]
                   9 <= 10 → advance j
        j=2: out of bounds → done

        Answer: [[2,4], [5,6], [9,9]]

    Time Complexity: O(n + m) - single pass through both lists
    Space Complexity: O(k) - where k is number of overlaps
    """
    i, j = 0, 0
    result = []

    while i < len(intervals1) and j < len(intervals2):
        a_start, a_end = intervals1[i]
        b_start, b_end = intervals2[j]

        # Check if they overlap: a starts before b ends AND b starts before a ends
        if a_start <= b_end and b_start <= a_end:
            # Intersection: [max of starts, min of ends]
            result.append([max(a_start, b_start), min(a_end, b_end)])

        # Advance the one that ends first (it can't overlap with anything else)
        if a_end <= b_end:
            i += 1
        else:
            j += 1

    return result


# -----------------------------------------------------------------------------
# Tests for find_overlaps
# -----------------------------------------------------------------------------

def test_find_overlaps_example():
    assert find_overlaps([[1, 4], [5, 6], [9, 10]], [[2, 7], [8, 9]]) == [[2, 4], [5, 6], [9, 9]]


def test_find_overlaps_no_overlap():
    assert find_overlaps([[1, 2], [5, 6]], [[3, 4], [7, 8]]) == []


def test_find_overlaps_full_overlap():
    assert find_overlaps([[1, 10]], [[2, 4], [6, 8]]) == [[2, 4], [6, 8]]


def test_find_overlaps_single_point():
    assert find_overlaps([[1, 3]], [[3, 5]]) == [[3, 3]]


def test_find_overlaps_empty_first():
    assert find_overlaps([], [[1, 2]]) == []


def test_find_overlaps_empty_second():
    assert find_overlaps([[1, 2]], []) == []


def test_find_overlaps_multiple():
    assert find_overlaps([[0, 2], [5, 10], [13, 23]], [[1, 5], [8, 12], [15, 20]]) == [[1, 2], [5, 5], [8, 10], [15, 20]]


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
