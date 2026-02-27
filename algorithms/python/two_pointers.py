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


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
