"""
Backtracking Pattern

A collection of algorithm problems using backtracking.

Backtracking = build a solution incrementally, undo (backtrack) when stuck.

Template:
    def backtrack(candidate, ...):
        if is_complete(candidate):
            results.append(candidate.copy())
            return

        for choice in choices:
            if is_valid(choice):
                candidate.append(choice)      # choose
                backtrack(candidate, ...)      # explore
                candidate.pop()                # un-choose (backtrack)

Key patterns:
    Permutations: pick any unused element at each step → n! results
    Subsets:      include or exclude each element → 2^n results
    N-Queens:     place one queen per row, track attacked cols/diags
"""


def permutations(nums: list[int]) -> list[list[int]]:
    """
    Find All Permutations

    Return all permutations of a list of unique integers.

    Example:
        >>> sorted(permutations([4, 5, 6]))
        [[4, 5, 6], [4, 6, 5], [5, 4, 6], [5, 6, 4], [6, 4, 5], [6, 5, 4]]

    At each level, try every unused number. Use a visited set to skip
    numbers already in the current permutation.

    Tree for [4, 5, 6]:
                        []
              /          |          \\
           [4]          [5]         [6]
          /   \\       /   \\       /   \\
       [4,5] [4,6] [5,4] [5,6] [6,4] [6,5]
         |     |     |     |     |     |
      [4,5,6] ...   ...   ...   ...  [6,5,4]

    Time Complexity: O(n! * n) -- n! permutations, each takes O(n) to copy
    Space Complexity: O(n) recursion depth + O(n) visited set
    """
    result = []

    def backtrack(candidate, visited):
        if len(candidate) == len(nums):
            result.append(candidate[:])  # copy current permutation
            return

        for num in nums:
            if num not in visited:
                candidate.append(num)
                visited.add(num)
                backtrack(candidate, visited)
                candidate.pop()          # undo choice
                visited.remove(num)      # undo visit

    backtrack([], set())
    return result


# -----------------------------------------------------------------------------
# Tests for permutations
# -----------------------------------------------------------------------------

def test_permutations_example():
    result = sorted(permutations([4, 5, 6]))
    expected = sorted([[4,5,6],[4,6,5],[5,4,6],[5,6,4],[6,4,5],[6,5,4]])
    assert result == expected


def test_permutations_single():
    assert permutations([1]) == [[1]]


def test_permutations_two():
    assert sorted(permutations([1, 2])) == [[1, 2], [2, 1]]


def test_permutations_empty():
    assert permutations([]) == [[]]


def test_permutations_count():
    # 4! = 24 permutations
    assert len(permutations([1, 2, 3, 4])) == 24


def subsets(nums: list[int]) -> list[list[int]]:
    """
    Find All Subsets

    Return all subsets (the power set) of a list of unique integers.

    Example:
        >>> sorted([sorted(s) for s in subsets([4, 5, 6])])
        [[], [4], [4, 5], [4, 5, 6], [4, 6], [5], [5, 6], [6]]

    At each index, make a binary choice: include or exclude nums[i].
    Then recurse to the next index. When we reach the end, save the subset.

    Tree for [4, 5, 6]:
                          []
                   /              \\
              include 4          exclude 4
               [4]                  []
             /      \\            /      \\
         inc 5    exc 5      inc 5    exc 5
         [4,5]    [4]        [5]       []
         /  \\    /  \\      /  \\    /  \\
       +6  -6  +6  -6    +6  -6  +6  -6
    [4,5,6][4,5][4,6][4] [5,6][5] [6] []

    Time Complexity: O(2^n * n) -- 2^n subsets, each up to O(n) to copy
    Space Complexity: O(n) recursion depth
    """
    result = []

    def backtrack(i, current):
        if i == len(nums):
            result.append(current[:])  # copy current subset
            return

        # Include nums[i]
        current.append(nums[i])
        backtrack(i + 1, current)
        current.pop()  # backtrack

        # Exclude nums[i]
        backtrack(i + 1, current)

    backtrack(0, [])
    return result


# -----------------------------------------------------------------------------
# Tests for subsets
# -----------------------------------------------------------------------------

def test_subsets_example():
    result = sorted([sorted(s) for s in subsets([4, 5, 6])])
    expected = sorted([[], [4], [4,5], [4,5,6], [4,6], [5], [5,6], [6]])
    assert result == expected


def test_subsets_empty():
    assert subsets([]) == [[]]


def test_subsets_single():
    result = sorted(subsets([1]))
    assert result == [[], [1]]


def test_subsets_two():
    result = sorted([sorted(s) for s in subsets([1, 2])])
    assert result == [[], [1], [1, 2], [2]]


def test_subsets_count():
    # 2^4 = 16 subsets
    assert len(subsets([1, 2, 3, 4])) == 16


def n_queens(n: int) -> int:
    """
    N Queens

    Place n queens on an n*n board so no two attack each other.
    Return the number of valid configurations.

    Example:
        >>> n_queens(4)
        2

    Place one queen per row (they can't share a row). For each row,
    try every column. Skip if column, diagonal, or anti-diagonal
    is already occupied.

    Example n=4, one solution:
        . Q . .    row 0, col 1
        . . . Q    row 1, col 3
        Q . . .    row 2, col 0
        . . Q .    row 3, col 2

    Time Complexity: O(n!) -- pruning reduces branching at each row
    Space Complexity: O(n) for the three sets + recursion

    How diagonals work:
        Diagonal (top-left to bottom-right): r - c is constant
            (e.g. (0,1) and (1,2) both = -1)
        Anti-diagonal (top-right to bottom-left): r + c is constant
            (e.g. (0,1) and (1,0) both = 1)
    """
    count = 0
    cols = set()
    diags = set()       # r - c
    anti_diags = set()  # r + c

    def backtrack(r):
        nonlocal count
        if r == n:
            count += 1
            return

        for c in range(n):
            if c in cols or (r - c) in diags or (r + c) in anti_diags:
                continue  # this square is attacked

            cols.add(c)
            diags.add(r - c)
            anti_diags.add(r + c)

            backtrack(r + 1)

            cols.remove(c)
            diags.remove(r - c)
            anti_diags.remove(r + c)

    backtrack(0)
    return count


# -----------------------------------------------------------------------------
# Tests for n_queens
# -----------------------------------------------------------------------------

def test_n_queens_4():
    assert n_queens(4) == 2


def test_n_queens_1():
    assert n_queens(1) == 1


def test_n_queens_2():
    assert n_queens(2) == 0


def test_n_queens_3():
    assert n_queens(3) == 0


def test_n_queens_8():
    assert n_queens(8) == 92


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
