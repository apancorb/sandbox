"""
Hash Maps and Sets Pattern

A collection of algorithm problems using hash maps and sets.
"""
import random


def pair_sum(nums: list[int], target: int) -> list[int]:
    """
    Pair Sum - Unsorted

    Given an array of integers, return the indexes of any two numbers that
    add up to a target. The order of the indexes doesn't matter. If no pair
    is found, return an empty list.

    Example:
        >>> pair_sum([-1, 3, 4, 2], 3)
        [0, 2]
        # nums[0] + nums[2] = -1 + 4 = 3

    For each number, check if (target - num) exists in map. If yes, found
    pair. If no, store current num and its index for future lookups.

    Walkthrough for nums=[-1, 3, 4, 2], target=3:
        i=0, num=-1: complement=4, not in seen → seen={-1:0}
        i=1, num=3:  complement=0, not in seen → seen={-1:0, 3:1}
        i=2, num=4:  complement=-1, found in seen! → return [0, 2]

    Time Complexity: O(n) - single pass through array
    Space Complexity: O(n) - hash map stores up to n elements
    """
    seen = {}

    for i, num in enumerate(nums):
        complement = target - num
        if complement in seen:
            return [seen[complement], i]
        seen[num] = i

    return []


# -----------------------------------------------------------------------------
# Tests for pair_sum
# -----------------------------------------------------------------------------

def test_pair_sum_example():
    assert pair_sum([-1, 3, 4, 2], 3) == [0, 2]


def test_pair_sum_basic():
    nums = [-5, -2, 3, 4, 6]
    result = pair_sum(nums, 7)
    assert len(result) == 2
    assert nums[result[0]] + nums[result[1]] == 7


def test_pair_sum_duplicates():
    nums = [1, 1, 1]
    result = pair_sum(nums, 2)
    assert len(result) == 2
    assert result[0] != result[1]
    assert nums[result[0]] + nums[result[1]] == 2


def test_pair_sum_no_solution():
    assert pair_sum([1, 2, 3], 10) == []


def test_pair_sum_negative_numbers():
    nums = [-10, -5, 0, 5, 10]
    result = pair_sum(nums, 0)
    assert len(result) == 2
    assert nums[result[0]] + nums[result[1]] == 0


def test_pair_sum_two_elements():
    assert pair_sum([1, 9], 10) == [0, 1]


def test_pair_sum_empty():
    assert pair_sum([], 5) == []


def test_pair_sum_single():
    assert pair_sum([5], 5) == []


def test_pair_sum_large_numbers():
    nums = [-1000000, 0, 1000000]
    result = pair_sum(nums, 0)
    assert len(result) == 2
    assert nums[result[0]] + nums[result[1]] == 0


def verify_sudoku(board: list[list[int]]) -> bool:
    """
    Verify Sudoku Board

    Given a partially completed 9x9 Sudoku board, determine if the current
    state adheres to the rules:
    - Each row must contain unique numbers 1-9 (0 = empty)
    - Each column must contain unique numbers 1-9
    - Each 3x3 subgrid must contain unique numbers 1-9

    Note: Verify current state is valid, not whether board is solvable.

    Example:
        >>> board = [
        ...     [5, 3, 0, 0, 7, 0, 0, 0, 0],
        ...     [6, 0, 0, 1, 9, 5, 0, 0, 0],
        ...     [0, 9, 8, 0, 0, 0, 0, 6, 0],
        ...     [8, 0, 0, 0, 6, 0, 0, 0, 3],
        ...     [4, 0, 0, 8, 0, 3, 0, 0, 1],
        ...     [7, 0, 0, 0, 2, 0, 0, 0, 6],
        ...     [0, 6, 0, 0, 0, 0, 2, 8, 0],
        ...     [0, 0, 0, 4, 1, 9, 0, 0, 5],
        ...     [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ... ]
        >>> verify_sudoku(board)
        True

    Use sets to track seen numbers in each row, column, and 3x3 subgrid.
    For each cell, check all three constraints. The subgrid index is found
    by integer-dividing the row and column by 3 (e.g., cell (4,7) maps to
    subgrid (1,2)).

    Walkthrough for checking cell (0,0)=5:
        row 0 set: empty → add 5 → {5}
        col 0 set: empty → add 5 → {5}
        subgrid (0,0) set: empty → add 5 → {5}
        No duplicates → continue

    If row 0 had another 5 (say at col 7):
        row 0 set already has 5 → duplicate found → return False

    Time Complexity: O(1) - always 81 cells (9x9 board)
    Space Complexity: O(1) - fixed size sets (max 9 elements each)
    """
    rows = [set() for _ in range(9)]
    cols = [set() for _ in range(9)]
    subgrids = [[set() for _ in range(3)] for _ in range(3)]
    
    for i in range(9):
        for j in range(9):
            num = board[i][j]
            if num == 0:
                continue

            if num in rows[i]:
                return False
            if num in cols[j]:
                return False
            # // - is integer division
            # i // 3 and j // 3 map cell position to its 3x3 subgrid
            # e.g., cell (4, 7) -> subgrid (1, 2)
            if num in subgrids[i // 3][j // 3]:
                return False
            
            rows[i].add(num)
            cols[j].add(num)
            subgrids[i // 3][j // 3].add(num)

    return True


# -----------------------------------------------------------------------------
# Tests for verify_sudoku
# -----------------------------------------------------------------------------

def test_verify_sudoku_valid_board():
    board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]
    assert verify_sudoku(board) is True


def test_verify_sudoku_invalid_row():
    board = [
        [5, 3, 0, 0, 7, 0, 0, 3, 0],  # duplicate 3 in row
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]
    assert verify_sudoku(board) is False


def test_verify_sudoku_invalid_column():
    board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [5, 0, 0, 0, 8, 0, 0, 7, 9],  # duplicate 5 in column 0
    ]
    assert verify_sudoku(board) is False


def test_verify_sudoku_invalid_subgrid():
    board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 5, 0, 0, 0, 0, 6, 0],  # duplicate 5 in top-left 3x3
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]
    assert verify_sudoku(board) is False


def test_verify_sudoku_empty_board():
    board = [[0] * 9 for _ in range(9)]
    assert verify_sudoku(board) is True


def test_verify_sudoku_full_valid():
    board = [
        [5, 3, 4, 6, 7, 8, 9, 1, 2],
        [6, 7, 2, 1, 9, 5, 3, 4, 8],
        [1, 9, 8, 3, 4, 2, 5, 6, 7],
        [8, 5, 9, 7, 6, 1, 4, 2, 3],
        [4, 2, 6, 8, 5, 3, 7, 9, 1],
        [7, 1, 3, 9, 2, 4, 8, 5, 6],
        [9, 6, 1, 5, 3, 7, 2, 8, 4],
        [2, 8, 7, 4, 1, 9, 6, 3, 5],
        [3, 4, 5, 2, 8, 6, 1, 7, 9],
    ]
    assert verify_sudoku(board) is True


def test_verify_sudoku_single_value():
    board = [[0] * 9 for _ in range(9)]
    board[0][0] = 5
    assert verify_sudoku(board) is True


def zero_striping(matrix: list[list[int]]) -> None:
    """
    Zero Striping (Set Matrix Zeroes)

    For each zero in an m x n matrix, set its entire row and column to zero.
    Modify in-place.

    Example:
        >>> m = [[1,2,3],[4,0,6],[7,8,9]]; zero_striping(m); m
        [[1,0,3],[0,0,0],[7,0,9]]

    Use first row and first column as markers. Track separately if first
    row/col themselves need zeroing. Then apply markers, then handle first
    row/col. This avoids needing extra sets, giving us O(1) space.

    Walkthrough for [[1,2,3],[4,0,6],[7,8,9]]:
        1. Check first row/col for zeros: none found
        2. Scan rest of matrix: (1,1)=0 → mark row 1: matrix[1][0]=0
                                           mark col 1: matrix[0][1]=0
        3. Apply markers:
           row 1 marked → [4,0,6] → [0,0,0]
           col 1 marked → col 1 all → [0,0,0] for col
        4. Result: [[1,0,3],[0,0,0],[7,0,9]]

    Time Complexity: O(m*n) - traverse matrix twice
    Space Complexity: O(1) - use first row/col as markers instead of extra sets
    """
    if not matrix:
        return

    rows, cols = len(matrix), len(matrix[0])

    # Check if first row/col have zeros (before we use them as markers)
    first_row_has_zero = any(matrix[0][j] == 0 for j in range(cols))
    first_col_has_zero = any(matrix[i][0] == 0 for i in range(rows))

    # Use first row/col as markers for rest of matrix
    for i in range(1, rows):
        for j in range(1, cols):
            if matrix[i][j] == 0:
                matrix[0][j] = 0  # mark column
                matrix[i][0] = 0  # mark row

    # Zero out cells based on markers
    for i in range(1, rows):
        for j in range(1, cols):
            if matrix[0][j] == 0 or matrix[i][0] == 0:
                matrix[i][j] = 0

    # Handle first row
    if first_row_has_zero:
        for j in range(cols):
            matrix[0][j] = 0

    # Handle first col
    if first_col_has_zero:
        for i in range(rows):
            matrix[i][0] = 0


# -----------------------------------------------------------------------------
# Tests for zero_striping
# -----------------------------------------------------------------------------

def test_zero_striping_basic():
    matrix = [[1, 2, 3], [4, 0, 6], [7, 8, 9]]
    zero_striping(matrix)
    assert matrix == [[1, 0, 3], [0, 0, 0], [7, 0, 9]]


def test_zero_striping_multiple_zeros():
    matrix = [[0, 2, 3], [4, 5, 6], [7, 8, 0]]
    zero_striping(matrix)
    assert matrix == [[0, 0, 0], [0, 5, 0], [0, 0, 0]]


def test_zero_striping_no_zeros():
    matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    zero_striping(matrix)
    assert matrix == [[1, 2, 3], [4, 5, 6], [7, 8, 9]]


def test_zero_striping_all_zeros():
    matrix = [[0, 0], [0, 0]]
    zero_striping(matrix)
    assert matrix == [[0, 0], [0, 0]]


def test_zero_striping_single_zero():
    matrix = [[0]]
    zero_striping(matrix)
    assert matrix == [[0]]


def test_zero_striping_single_nonzero():
    matrix = [[5]]
    zero_striping(matrix)
    assert matrix == [[5]]


def test_zero_striping_empty():
    matrix = []
    zero_striping(matrix)
    assert matrix == []


def test_zero_striping_single_row():
    matrix = [[1, 0, 3, 4]]
    zero_striping(matrix)
    assert matrix == [[0, 0, 0, 0]]


def test_zero_striping_single_column():
    matrix = [[1], [0], [3]]
    zero_striping(matrix)
    assert matrix == [[0], [0], [0]]


def test_zero_striping_rectangular():
    matrix = [[1, 2, 3, 4], [5, 0, 7, 8], [9, 10, 11, 12]]
    zero_striping(matrix)
    assert matrix == [[1, 0, 3, 4], [0, 0, 0, 0], [9, 0, 11, 12]]


def majority_element(nums: list[int]) -> int:
    """
    Majority Element

    Given an array of size n, return the majority element. The majority
    element is the element that appears more than n/2 times.

    Examples:
        >>> majority_element([3, 2, 3])
        3
        >>> majority_element([2, 2, 1, 1, 1, 2, 2])
        2

    Boyer-Moore Voting Algorithm: treat it like an election. The majority
    candidate gains votes, others cancel out. Since majority > n/2, it
    will always survive.

    Walkthrough for [2, 2, 1, 1, 1, 2, 2]:
        i=0: candidate=2, count=1
        i=1: 2==candidate → count=2
        i=2: 1!=candidate → count=1
        i=3: 1!=candidate → count=0 → new candidate=1, count=1
        i=4: 1==candidate → count=2
        i=5: 2!=candidate → count=1
        i=6: 2!=candidate → count=0 → new candidate=2, count=1

        Answer: 2

    Time Complexity: O(n) - single pass
    Space Complexity: O(1) - only two variables
    """
    candidate = nums[0]
    count = 1

    for i in range(1, len(nums)):
        if nums[i] == candidate:
            count += 1
        else:
            count -= 1

        if count == 0:
            candidate = nums[i]
            count = 1

    return candidate


# -----------------------------------------------------------------------------
# Tests for majority_element
# -----------------------------------------------------------------------------

def test_majority_element_example1():
    assert majority_element([3, 2, 3]) == 3


def test_majority_element_example2():
    assert majority_element([2, 2, 1, 1, 1, 2, 2]) == 2


def test_majority_element_single():
    assert majority_element([1]) == 1


def test_majority_element_two_same():
    assert majority_element([5, 5]) == 5


def test_majority_element_all_same():
    assert majority_element([7, 7, 7, 7, 7]) == 7


def test_majority_element_at_end():
    assert majority_element([1, 2, 3, 3, 3]) == 3


class RandomizedSet:
    """
    Insert Delete GetRandom O(1)

    A data structure that supports insert, remove, and getRandom in O(1)
    average time.

    - insert(val): Insert if not present. Returns True if inserted.
    - remove(val): Remove if present. Returns True if removed.
    - get_random(): Return a random element (uniform probability).

    Example:
        >>> s = RandomizedSet()
        >>> s.insert(1)  # True
        >>> s.remove(2)  # False (not present)
        >>> s.insert(2)  # True
        >>> s.get_random()  # 1 or 2

    Use a list for O(1) random access and a dict mapping val->index.
    For removal, swap with last element to maintain O(1) delete. The
    trick is that lists allow O(1) pop from the end, so swapping the
    target to the last position lets us remove without shifting.

    Walkthrough for insert(1), insert(2), remove(1):
        insert(1): values=[1], val_to_index={1:0}
        insert(2): values=[1,2], val_to_index={1:0, 2:1}
        remove(1): idx=0, last_val=2
                   swap: values=[2,2], update val_to_index={1:0, 2:0}
                   pop:  values=[2], delete key 1 → val_to_index={2:0}

    Time Complexity: O(1) average for all operations
    Space Complexity: O(n) - storing n elements
    """

    def __init__(self):
        self.val_to_index = {}  # val -> index in list
        self.values = []        # for O(1) random access

    def insert(self, val: int) -> bool:
        if val in self.val_to_index:
            return False
        self.values.append(val)
        self.val_to_index[val] = len(self.values) - 1
        return True

    def remove(self, val: int) -> bool:
        if val not in self.val_to_index:
            return False

        # Swap with last element for O(1) removal
        idx = self.val_to_index[val]
        last_val = self.values[-1]

        self.values[idx] = last_val
        self.val_to_index[last_val] = idx

        self.values.pop()
        del self.val_to_index[val]
        return True

    def get_random(self) -> int:
        return random.choice(self.values)


# -----------------------------------------------------------------------------
# Tests for RandomizedSet
# -----------------------------------------------------------------------------

def test_randomized_set_example():
    s = RandomizedSet()
    assert s.insert(1) is True
    assert s.remove(2) is False
    assert s.insert(2) is True
    assert s.get_random() in [1, 2]
    assert s.remove(1) is True
    assert s.insert(2) is False
    assert s.get_random() == 2


def test_randomized_set_insert_duplicate():
    s = RandomizedSet()
    assert s.insert(5) is True
    assert s.insert(5) is False


def test_randomized_set_remove_nonexistent():
    s = RandomizedSet()
    assert s.remove(10) is False


def test_randomized_set_insert_remove_insert():
    s = RandomizedSet()
    assert s.insert(1) is True
    assert s.remove(1) is True
    assert s.insert(1) is True
    assert s.get_random() == 1


def test_randomized_set_multiple():
    s = RandomizedSet()
    s.insert(10)
    s.insert(20)
    s.insert(30)
    s.remove(20)
    assert s.get_random() in [10, 30]


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
