"""
Arrays Pattern

A collection of algorithm problems involving array and string manipulation.

Sections:
- Two Pointers: problems using converging/diverging pointer pairs on sorted data
- Sliding Windows: fixed and variable-size window techniques
- Prefix Sums: precomputed cumulative sums for range queries
"""


# =============================================================================
# Two Pointers
# =============================================================================


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

    The two pointers start at opposite ends. If sum is too small, move left
    pointer right (increase sum). If sum is too large, move right pointer
    left (decrease sum). Works because the array is sorted.

    Example walkthrough for nums=[-5, -2, 3, 4, 6], target=7:
        left=0, right=4: -5 + 6 = 1 < 7 → move left
        left=1, right=4: -2 + 6 = 4 < 7 → move left
        left=2, right=4:  3 + 6 = 9 > 7 → move right
        left=2, right=3:  3 + 4 = 7 = 7 ✓ → return [2, 3]

    Time Complexity: O(n) - single pass through the array with two pointers
    Space Complexity: O(1) - only using two pointer variables
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

    For each element, we use two pointers to find pairs that sum to its
    negation. Skip duplicates at each level to avoid duplicate triplets.

    Example walkthrough for nums=[0, -1, 2, -3, 1]:
        sorted: [-3, -1, 0, 1, 2]

        i=0, nums[i]=-3, target=3:
            left=1, right=4: -1 + 2 = 1 < 3 → move left
            left=2, right=4:  0 + 2 = 2 < 3 → move left
            left=3, right=4:  1 + 2 = 3 = 3 ✓ → found [-3, 1, 2]

        i=1, nums[i]=-1, target=1:
            left=2, right=4: 0 + 2 = 2 > 1 → move right
            left=2, right=3: 0 + 1 = 1 = 1 ✓ → found [-1, 0, 1]

        i=2, nums[i]=0 > 0 → break

        Answer: [[-3, 1, 2], [-1, 0, 1]]

    Time Complexity: O(n^2) - sorting is O(n log n), then for each element
        we do a two-pointer search O(n), giving O(n^2) total
    Space Complexity: O(1) - excluding the output, only using pointers
        (sorting may use O(n) depending on implementation)
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


def is_palindrome_valid(s: str) -> bool:
    """
    Is Palindrome Valid

    A palindrome is a sequence of characters that reads the same forward and
    backward. Given a string, determine if it's a palindrome after removing
    all non-alphanumeric characters.

    Examples:
        >>> is_palindrome_valid("a dog, a panic in a pagoda")
        True
        >>> is_palindrome_valid("abc123")
        False

    Use two pointers from both ends, skip non-alphanumeric chars,
    compare case-insensitively.

    Example walkthrough for s="a dog, a panic in a pagoda":
        left=0 'a', right=25 'a' → match, move inward
        left=1 ' ' → skip, left=2 'd'
        right=24 'd' → match, move inward
        left=3 'o', right=23 'o' → match
        left=4 'g', right=22 'g' → match
        ... continues matching all alphanumeric chars symmetrically ...
        All pairs match → True

    Time Complexity: O(n) - single pass with two pointers
    Space Complexity: O(1) - only pointer variables
    """
    if not s:
        return True

    left = 0
    right = len(s) - 1

    while left < right:
        while left < right and not s[left].isalnum():
            left += 1

        while left < right and not s[right].isalnum():
            right -= 1

        if s[left].lower() != s[right].lower():
            return False

        left += 1
        right -= 1

    return True


# -----------------------------------------------------------------------------
# Tests for is_palindrome_valid
# -----------------------------------------------------------------------------

def test_is_palindrome_valid_example_1():
    assert is_palindrome_valid("a dog, a panic in a pagoda") is True


def test_is_palindrome_valid_example_2():
    assert is_palindrome_valid("abc123") is False


def test_is_palindrome_valid_empty_string():
    assert is_palindrome_valid("") is True


def test_is_palindrome_valid_single_char():
    assert is_palindrome_valid("a") is True


def test_is_palindrome_valid_two_chars_palindrome():
    assert is_palindrome_valid("aa") is True


def test_is_palindrome_valid_two_chars_not_palindrome():
    assert is_palindrome_valid("ab") is False


def test_is_palindrome_valid_no_alphanumeric():
    assert is_palindrome_valid(" ' (?)") is True


def test_is_palindrome_valid_date_palindrome():
    assert is_palindrome_valid("12.02.2021") is True


def test_is_palindrome_valid_date_not_palindrome():
    assert is_palindrome_valid("21.02.2021") is False


def test_is_palindrome_valid_hello_world():
    assert is_palindrome_valid("hello, world!") is False


def largest_container(heights: list[int]) -> int:
    """
    Largest Container (Container With Most Water)

    You are given an array of numbers, each representing the height of a
    vertical line on a graph. A container can be formed with any pair of
    these lines, along with the x-axis of the graph. Return the amount of
    water which the largest container can hold.

    Example:
        >>> largest_container([2, 7, 8, 3, 7, 6])
        24

    Start pointers at both ends. The area is min(height) * width. Move the
    pointer with the smaller height inward, since moving the taller one
    can only decrease the area.

    Example walkthrough for heights=[2, 7, 8, 3, 7, 6]:
        left=0, right=5: min(2,6)*5 = 10, 2 < 6 → move left
        left=1, right=5: min(7,6)*4 = 24 ★, 7 > 6 → move right
        left=1, right=4: min(7,7)*3 = 21, 7 >= 7 → move right
        left=1, right=3: min(7,3)*2 = 6, 7 > 3 → move right
        left=1, right=2: min(7,8)*1 = 7, 7 < 8 → move left
        left=2, right=2: done

        Answer: 24

    Time Complexity: O(n) - single pass with two pointers
    Space Complexity: O(1) - only pointer variables
    """
    if len(heights) < 2:
        return 0

    left = 0
    right = len(heights) - 1
    max_water = 0

    while left < right:
        min_height = min(heights[left], heights[right])
        curr_water = (right - left) * min_height
        max_water = max(max_water, curr_water)

        if heights[left] < heights[right]:
            left += 1
        else:
            right -= 1

    return max_water


# -----------------------------------------------------------------------------
# Tests for largest_container
# -----------------------------------------------------------------------------

def test_largest_container_example():
    assert largest_container([2, 7, 8, 3, 7, 6]) == 24


def test_largest_container_empty():
    assert largest_container([]) == 0


def test_largest_container_single_element():
    assert largest_container([1]) == 0


def test_largest_container_no_water():
    assert largest_container([0, 1, 0]) == 0


def test_largest_container_same_heights():
    assert largest_container([3, 3, 3, 3]) == 9


def test_largest_container_increasing():
    assert largest_container([1, 2, 3]) == 2


def test_largest_container_decreasing():
    assert largest_container([3, 2, 1]) == 2


def remove_element(nums: list[int], val: int) -> int:
    """
    Remove Element

    Remove all occurrences of val in nums in-place. Return the number of
    elements not equal to val. The first k elements of nums should contain
    the non-val elements.

    Examples:
        >>> nums = [3, 2, 2, 3]; k = remove_element(nums, 3)
        >>> k, sorted(nums[:k])
        (2, [2, 2])

        >>> nums = [0, 1, 2, 2, 3, 0, 4, 2]; k = remove_element(nums, 2)
        >>> k, sorted(nums[:k])
        (5, [0, 0, 1, 3, 4])

    Use two pointers: one from start, one from end. When we find val at
    left pointer, swap with right pointer's element and shrink right.

    Example walkthrough for nums=[3, 2, 2, 3], val=3:
        left=0, right=3: nums[0]=3 == val → copy nums[3]=3, right=2
        left=0, right=2: nums[0]=3 == val → copy nums[2]=2, right=1
        left=0, right=1: nums[0]=2 != val → left=1
        left=1, right=1: nums[1]=2 != val → left=2
        left=2 > right=1 → done, k=2
        nums[:2] = [2, 2] ✓

    Time Complexity: O(n) - single pass through the array
    Space Complexity: O(1) - in-place modification
    """
    if not nums:
        return 0

    left = 0
    right = len(nums) - 1

    while left <= right:
        if nums[left] == val:
            nums[left] = nums[right]
            right -= 1
        else:
            left += 1

    return left


# -----------------------------------------------------------------------------
# Tests for remove_element
# -----------------------------------------------------------------------------

def test_remove_element_example1():
    nums = [3, 2, 2, 3]
    k = remove_element(nums, 3)
    assert k == 2
    assert sorted(nums[:k]) == [2, 2]


def test_remove_element_example2():
    nums = [0, 1, 2, 2, 3, 0, 4, 2]
    k = remove_element(nums, 2)
    assert k == 5
    assert sorted(nums[:k]) == [0, 0, 1, 3, 4]


def test_remove_element_empty():
    nums = []
    k = remove_element(nums, 1)
    assert k == 0


def test_remove_element_all_same():
    nums = [3, 3, 3, 3]
    k = remove_element(nums, 3)
    assert k == 0


def test_remove_element_none_match():
    nums = [1, 2, 3, 4]
    k = remove_element(nums, 5)
    assert k == 4
    assert sorted(nums[:k]) == [1, 2, 3, 4]


def remove_duplicates(nums: list[int]) -> int:
    """
    Remove Duplicates from Sorted Array

    Given a sorted array, remove duplicates in-place such that each unique
    element appears only once. Return the number of unique elements k.

    Examples:
        >>> nums = [1, 1, 2]; k = remove_duplicates(nums)
        >>> k, nums[:k]
        (2, [1, 2])

        >>> nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]; k = remove_duplicates(nums)
        >>> k, nums[:k]
        (5, [0, 1, 2, 3, 4])

    Use slow/fast pointers. Slow marks where to write next unique value.
    Fast scans ahead. When fast finds a new value, write it at slow position.

    Example walkthrough for nums=[0, 0, 1, 1, 1, 2, 2, 3, 3, 4]:
        slow=1
        fast=1: nums[1]=0 == nums[0]=0 → skip
        fast=2: nums[2]=1 != nums[1]=0 → write 1 at slow=1, slow=2
        fast=3: nums[3]=1 == nums[2]=1 → skip
        fast=4: nums[4]=1 == nums[3]=1 → skip
        fast=5: nums[5]=2 != nums[4]=1 → write 2 at slow=2, slow=3
        fast=6: nums[6]=2 == nums[5]=2 → skip
        fast=7: nums[7]=3 != nums[6]=2 → write 3 at slow=3, slow=4
        fast=8: nums[8]=3 == nums[7]=3 → skip
        fast=9: nums[9]=4 != nums[8]=3 → write 4 at slow=4, slow=5
        Answer: k=5, nums[:5] = [0, 1, 2, 3, 4]

    Time Complexity: O(n) - single pass through the array
    Space Complexity: O(1) - in-place modification
    """
    if not nums:
        return 0

    slow = 1

    for fast in range(1, len(nums)):
        if nums[fast] != nums[fast - 1]:
            nums[slow] = nums[fast]
            slow += 1

    return slow


# -----------------------------------------------------------------------------
# Tests for remove_duplicates
# -----------------------------------------------------------------------------

def test_remove_duplicates_example1():
    nums = [1, 1, 2]
    k = remove_duplicates(nums)
    assert k == 2
    assert nums[:k] == [1, 2]


def test_remove_duplicates_example2():
    nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]
    k = remove_duplicates(nums)
    assert k == 5
    assert nums[:k] == [0, 1, 2, 3, 4]


def test_remove_duplicates_empty():
    nums = []
    k = remove_duplicates(nums)
    assert k == 0


def test_remove_duplicates_single():
    nums = [1]
    k = remove_duplicates(nums)
    assert k == 1
    assert nums[:k] == [1]


def test_remove_duplicates_no_duplicates():
    nums = [1, 2, 3, 4, 5]
    k = remove_duplicates(nums)
    assert k == 5
    assert nums[:k] == [1, 2, 3, 4, 5]


def test_remove_duplicates_all_same():
    nums = [5, 5, 5, 5]
    k = remove_duplicates(nums)
    assert k == 1
    assert nums[:k] == [5]


def rotate(nums: list[int], k: int) -> None:
    """
    Rotate Array

    Rotate the array to the right by k steps, in-place.

    Examples:
        >>> nums = [1, 2, 3, 4, 5, 6, 7]; rotate(nums, 3); nums
        [5, 6, 7, 1, 2, 3, 4]

        >>> nums = [-1, -100, 3, 99]; rotate(nums, 2); nums
        [3, 99, -1, -100]

    Reverse entire array, then reverse first k elements, then reverse rest.
    This works because reversing twice puts elements in the right order,
    just shifted by k positions.

    Example walkthrough for nums=[1,2,3,4,5,6,7], k=3:
        Step 1 - reverse all:    [7, 6, 5, 4, 3, 2, 1]
        Step 2 - reverse [0:3]:  [5, 6, 7, 4, 3, 2, 1]
        Step 3 - reverse [3:7]:  [5, 6, 7, 1, 2, 3, 4] ✓

    Time Complexity: O(n) - reverse three times, each O(n)
    Space Complexity: O(1) - in-place reversal
    """
    if not nums:
        return

    k = k % len(nums)

    def reverse(left: int, right: int) -> None:
        while left < right:
            nums[left], nums[right] = nums[right], nums[left]
            left += 1
            right -= 1

    reverse(0, len(nums) - 1)
    reverse(0, k - 1)
    reverse(k, len(nums) - 1)


# -----------------------------------------------------------------------------
# Tests for rotate
# -----------------------------------------------------------------------------

def test_rotate_example1():
    nums = [1, 2, 3, 4, 5, 6, 7]
    rotate(nums, 3)
    assert nums == [5, 6, 7, 1, 2, 3, 4]


def test_rotate_example2():
    nums = [-1, -100, 3, 99]
    rotate(nums, 2)
    assert nums == [3, 99, -1, -100]


def test_rotate_k_zero():
    nums = [1, 2, 3]
    rotate(nums, 0)
    assert nums == [1, 2, 3]


def test_rotate_k_equals_len():
    nums = [1, 2, 3]
    rotate(nums, 3)
    assert nums == [1, 2, 3]


def test_rotate_k_greater_than_len():
    nums = [1, 2, 3]
    rotate(nums, 4)
    assert nums == [3, 1, 2]


def test_rotate_single():
    nums = [1]
    rotate(nums, 5)
    assert nums == [1]


def remove_duplicates_ii(nums: list[int]) -> int:
    """
    Remove Duplicates from Sorted Array II

    Given a sorted array, remove duplicates in-place such that each unique
    element appears at most twice. Return the number of elements k.

    Examples:
        >>> nums = [1, 1, 1, 2, 2, 3]; k = remove_duplicates_ii(nums)
        >>> k, nums[:k]
        (5, [1, 1, 2, 2, 3])

        >>> nums = [0, 0, 1, 1, 1, 1, 2, 3, 3]; k = remove_duplicates_ii(nums)
        >>> k, nums[:k]
        (7, [0, 0, 1, 1, 2, 3, 3])

    Track count of current value. Only write when count <= 2.
    Use slow/fast pointers where slow marks the write position.

    Example walkthrough for nums=[1, 1, 1, 2, 2, 3]:
        slow=1, count=1
        fast=1: 1 == 1, count=1 < 2 → write, slow=2, count=2
        fast=2: 1 == 1, count=2, not < 2 → skip
        fast=3: 2 != 1 → write, slow=3, count=1
        fast=4: 2 == 2, count=1 < 2 → write, slow=4, count=2
        fast=5: 3 != 2 → write, slow=5, count=1
        Answer: k=5, nums[:5] = [1, 1, 2, 2, 3]

    Time Complexity: O(n) - single pass through the array
    Space Complexity: O(1) - in-place modification
    """
    if not nums:
        return 0

    slow = 1
    count = 1

    for fast in range(1, len(nums)):
        if nums[fast] == nums[fast - 1]:
            if count < 2:
                nums[slow] = nums[fast]
                slow += 1
                count += 1
        else:
            nums[slow] = nums[fast]
            slow += 1
            count = 1

    return slow


# -----------------------------------------------------------------------------
# Tests for remove_duplicates_ii
# -----------------------------------------------------------------------------

def test_remove_duplicates_ii_example1():
    nums = [1, 1, 1, 2, 2, 3]
    k = remove_duplicates_ii(nums)
    assert k == 5
    assert nums[:k] == [1, 1, 2, 2, 3]


def test_remove_duplicates_ii_example2():
    nums = [0, 0, 1, 1, 1, 1, 2, 3, 3]
    k = remove_duplicates_ii(nums)
    assert k == 7
    assert nums[:k] == [0, 0, 1, 1, 2, 3, 3]


def test_remove_duplicates_ii_empty():
    nums = []
    k = remove_duplicates_ii(nums)
    assert k == 0


def test_remove_duplicates_ii_single():
    nums = [1]
    k = remove_duplicates_ii(nums)
    assert k == 1
    assert nums[:k] == [1]


def test_remove_duplicates_ii_two_same():
    nums = [1, 1]
    k = remove_duplicates_ii(nums)
    assert k == 2
    assert nums[:k] == [1, 1]


def test_remove_duplicates_ii_all_same():
    nums = [5, 5, 5, 5, 5]
    k = remove_duplicates_ii(nums)
    assert k == 2
    assert nums[:k] == [5, 5]


def trap(height: list[int]) -> int:
    """
    Trapping Rain Water

    Given n non-negative integers representing an elevation map where the
    width of each bar is 1, compute how much water it can trap after raining.

    Examples:
        >>> trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
        6

        >>> trap([4, 2, 0, 3, 2, 5])
        9

    Water at each position = min(max_left, max_right) - height.
    Use two pointers tracking max heights from each side. Process the
    smaller side since it's the bottleneck for water level.

    Example walkthrough for height=[4, 2, 0, 3, 2, 5]:
        left=0, right=5, left_max=4, right_max=5
        left_max(4) < right_max(5) → process left side
            left=1, left_max=max(4,2)=4, water += 4-2 = 2
            left=2, left_max=max(4,0)=4, water += 4-0 = 4 (total=6)
            left=3, left_max=max(4,3)=4, water += 4-3 = 1 (total=7)
        left_max(4) < right_max(5) → process left side
            left=4, left_max=max(4,2)=4, water += 4-2 = 2 (total=9)
        left_max(4) < right_max(5) → process left side
            left=5, left=right → done

        Answer: 9

    Time Complexity: O(n) - single pass with two pointers
    Space Complexity: O(1) - only pointer variables
    """
    if len(height) < 3:
        return 0

    left = 0
    right = len(height) - 1
    left_max = height[left]
    right_max = height[right]
    water = 0

    while left < right:
        if left_max < right_max:
            left += 1
            left_max = max(left_max, height[left])
            water += left_max - height[left]
        else:
            right -= 1
            right_max = max(right_max, height[right])
            water += right_max - height[right]

    return water


# -----------------------------------------------------------------------------
# Tests for trap
# -----------------------------------------------------------------------------

def test_trap_example1():
    assert trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]) == 6


def test_trap_example2():
    assert trap([4, 2, 0, 3, 2, 5]) == 9


def test_trap_empty():
    assert trap([]) == 0


def test_trap_no_trap():
    assert trap([1, 2, 3, 4, 5]) == 0  # increasing
    assert trap([5, 4, 3, 2, 1]) == 0  # decreasing


def test_trap_single_valley():
    assert trap([3, 0, 3]) == 3


def test_trap_flat():
    assert trap([2, 2, 2, 2]) == 0


# =============================================================================
# Sliding Windows
# =============================================================================


def count_anagrams(s: str, t: str) -> int:
    """
    Substring Anagrams

    Count the number of substrings in s that are anagrams of t.

    Example:
        >>> count_anagrams("caabab", "aba")
        2
        # Anagrams at index 1 ("aab") and index 2 ("aba")

    Fixed window of size len(t). Compare character frequencies.

    Example walkthrough for s="caabab", t="aba":
        expected freq for "aba": {a:2, b:1}

        i=0: window="c"     → not full yet (need size 3)
        i=1: window="ca"    → not full yet
        i=2: window="caa"   → full! freq={c:1,a:2} ≠ expected, remove 'c'
        i=3: window="aab"   → freq={a:2,b:1} = expected ✓ count=1, remove 'a'
        i=4: window="aba"   → freq={a:2,b:1} = expected ✓ count=2, remove 'a'
        i=5: window="bab"   → freq={a:1,b:2} ≠ expected

        Answer: 2

    Time Complexity: O(n) - single pass with fixed window
    Space Complexity: O(1) - fixed size arrays (26 letters)
    """
    if len(t) > len(s) or len(t) == 0:
        return 0

    # Count character frequencies in t (what we're looking for)
    expected = [0] * 26
    for c in t:
        expected[ord(c) - ord('a')] += 1

    # Current window's character frequencies
    window = [0] * 26
    count = 0

    for i, c in enumerate(s):
        # 1. EXPAND: add right character to window
        window[ord(c) - ord('a')] += 1

        # 2. CHECK: once window reaches target size
        if i >= len(t) - 1:
            # Is this window an anagram? (same char frequencies)
            if window == expected:
                count += 1

            # 3. SHRINK: remove leftmost character to slide window
            left_char = s[i - len(t) + 1]
            window[ord(left_char) - ord('a')] -= 1

    return count


# -----------------------------------------------------------------------------
# Tests for count_anagrams
# -----------------------------------------------------------------------------

def test_count_anagrams_example():
    assert count_anagrams("caabab", "aba") == 2


def test_count_anagrams_no_match():
    assert count_anagrams("abcdef", "xyz") == 0


def test_count_anagrams_all_same_char():
    assert count_anagrams("aaaa", "aa") == 3


def test_count_anagrams_exact_match():
    assert count_anagrams("abc", "abc") == 1


def test_count_anagrams_single_char():
    assert count_anagrams("ababa", "a") == 3


def test_count_anagrams_t_longer_than_s():
    assert count_anagrams("ab", "abc") == 0


def test_count_anagrams_empty_s():
    assert count_anagrams("", "abc") == 0


def test_count_anagrams_empty_t():
    assert count_anagrams("abc", "") == 0


def test_count_anagrams_multiple_matches():
    # "ab", "ba", "ab" are all anagrams of "ab"
    assert count_anagrams("abab", "ab") == 3


def test_count_anagrams_overlapping():
    assert count_anagrams("cbaebabacd", "abc") == 2


def longest_unique_substring(s: str) -> int:
    """
    Longest Substring With Unique Characters

    Find the length of the longest substring with all unique characters.

    Example:
        >>> longest_unique_substring("abcba")
        3
        # "abc" or "cba" are the longest with unique chars

    Variable window: expand right, shrink left when duplicate found.
    Track last seen position of each character.

    Example walkthrough for s="abcba":
        i=0 'a': last_seen={}, no dup → window [0,0]="a", len=1, last_seen={a:0}
        i=1 'b': no dup → window [0,1]="ab", len=2, last_seen={a:0,b:1}
        i=2 'c': no dup → window [0,2]="abc", len=3 ★, last_seen={a:0,b:1,c:2}
        i=3 'b': dup! b was at 1, jump left to 2 → window [2,3]="cb", len=2
        i=4 'a': a was at 0, but 0 < left(2) so NOT in window, no jump
                 → window [2,4]="cba", len=3

        Answer: 3

    Time Complexity: O(n) - each char visited at most twice
    Space Complexity: O(min(n, 26)) - hashmap of char positions
    """
    last_seen = {}  # char -> last index where we saw it
    max_len = 0
    left = 0

    for right, char in enumerate(s):
        # If char is duplicate AND it's inside our current window
        if char in last_seen and last_seen[char] >= left:
            # Jump left pointer past the previous occurrence
            # (no need to shrink one-by-one, we can jump directly)
            left = last_seen[char] + 1

        # Current window is [left, right], all unique
        max_len = max(max_len, right - left + 1)

        # Record/update where we saw this char
        last_seen[char] = right

    return max_len


# -----------------------------------------------------------------------------
# Tests for longest_unique_substring
# -----------------------------------------------------------------------------

def test_longest_unique_substring_example():
    assert longest_unique_substring("abcba") == 3


def test_longest_unique_substring_all_unique():
    assert longest_unique_substring("abcdef") == 6


def test_longest_unique_substring_all_same():
    assert longest_unique_substring("aaaa") == 1


def test_longest_unique_substring_empty():
    assert longest_unique_substring("") == 0


def test_longest_unique_substring_single_char():
    assert longest_unique_substring("a") == 1


def test_longest_unique_substring_two_chars():
    assert longest_unique_substring("ab") == 2


def test_longest_unique_substring_repeating_pattern():
    assert longest_unique_substring("abcabcbb") == 3


def test_longest_unique_substring_end_longest():
    assert longest_unique_substring("aabcdef") == 6


def test_longest_unique_substring_middle_longest():
    assert longest_unique_substring("aaabcdefa") == 6


def longest_uniform_substring(s: str, k: int) -> int:
    """
    Longest Uniform Substring After Replacements

    Find the longest substring where all characters are the same,
    if you can replace up to k characters.

    Example:
        >>> longest_uniform_substring("aabcdcca", 2)
        5
        # Replace 'b' and 'd' with 'c' to get "ccccc"

    Key insight: window is valid if (window_size - most_frequent_char) <= k
    That's the number of chars we need to replace.

    Example walkthrough for s="aabcdcca", k=2:
        Keep the most frequent char, replace the rest.
        Valid if: window_size - max_freq <= k

        i=0 'a': freq={a:1}, max_freq=1, window="a", replace=1-1=0 ≤2 ✓
        i=1 'a': freq={a:2}, max_freq=2, window="aa", replace=2-2=0 ≤2 ✓
        i=2 'b': freq={a:2,b:1}, max_freq=2, window="aab", replace=3-2=1 ≤2 ✓
        i=3 'c': freq={a:2,b:1,c:1}, max_freq=2, replace=4-2=2 ≤2 ✓
        i=4 'd': freq={a:2,b:1,c:1,d:1}, max_freq=2, replace=5-2=3 >2 ✗
                 shrink! remove 'a', left=1, replace=4-1=3 >2 ✗
                 shrink! remove 'a', left=2, replace=3-1=2 ≤2 ✓
        i=5 'c': freq={b:1,c:2,d:1}, max_freq=2, replace=4-2=2 ≤2 ✓
        i=6 'c': freq={b:1,c:3,d:1}, max_freq=3, replace=5-3=2 ≤2 ✓ len=5 ★
        i=7 'a': freq={a:1,b:1,c:3,d:1}, max_freq=3, replace=6-3=3 >2 ✗
                 shrink! ...

        Answer: 5 (make "ccccc" by replacing b,d with c)

    Time Complexity: O(n) - single pass
    Space Complexity: O(26) - frequency map
    """
    if not s:
        return 0

    freq = {}       # char -> count in current window
    max_freq = 0    # highest frequency of any single char in window
    left = 0
    max_len = 0

    for right, char in enumerate(s):
        # 1. EXPAND: add right char to window
        freq[char] = freq.get(char, 0) + 1
        max_freq = max(max_freq, freq[char])

        # 2. SHRINK if invalid: need more than k replacements
        # replacements = window_size - max_freq (replace everything except most common)
        while (right - left + 1) - max_freq > k:
            freq[s[left]] -= 1
            left += 1

        # 3. UPDATE: window [left, right] is valid, track max
        max_len = max(max_len, right - left + 1)

    return max_len


# -----------------------------------------------------------------------------
# Tests for longest_uniform_substring
# -----------------------------------------------------------------------------

def test_longest_uniform_substring_example():
    assert longest_uniform_substring("aabcdcca", 2) == 5


def test_longest_uniform_substring_no_replacements():
    assert longest_uniform_substring("aaabbb", 0) == 3


def test_longest_uniform_substring_all_same():
    assert longest_uniform_substring("aaaa", 2) == 4


def test_longest_uniform_substring_all_different():
    assert longest_uniform_substring("abcd", 2) == 3


def test_longest_uniform_substring_k_equals_len():
    assert longest_uniform_substring("abcd", 4) == 4


def test_longest_uniform_substring_empty():
    assert longest_uniform_substring("", 2) == 0


def test_longest_uniform_substring_single_char():
    assert longest_uniform_substring("a", 2) == 1


def test_longest_uniform_substring_two_chars():
    assert longest_uniform_substring("ab", 1) == 2


def test_longest_uniform_substring_alternating():
    assert longest_uniform_substring("ababab", 2) == 5


# =============================================================================
# Prefix Sums
# =============================================================================


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
