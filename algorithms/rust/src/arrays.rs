//! Arrays Pattern
//!
//! A collection of algorithm problems involving array and string manipulation.
//!
//! Sections:
//! - Two Pointers: problems using converging/diverging pointer pairs on sorted data
//! - Sliding Windows: fixed and variable-size window techniques
//! - Prefix Sums: precomputed cumulative sums for range queries

use std::collections::HashMap;

// =============================================================================
// Two Pointers
// =============================================================================

/// Pair Sum - Sorted
///
/// Given an array of integers sorted in ascending order and a target value,
/// return the indexes of any pair of numbers in the array that sum to the
/// target. The order of the indexes in the result doesn't matter. If no pair
/// is found, return an empty array.
///
/// # Examples
///
/// ```text
/// Input: nums = [-5, -2, 3, 4, 6], target = 7
/// Output: [2, 3]
///
/// Input: nums = [1, 1, 1], target = 2
/// Output: [0, 1]   (any valid pair like [0, 1], [0, 2], [1, 2])
/// ```
///
/// The two pointers start at opposite ends. If sum is too small, move left
/// pointer right (increase sum). If sum is too large, move right pointer
/// left (decrease sum). Works because the array is sorted.
///
/// Example walkthrough for nums=[-5, -2, 3, 4, 6], target=7:
///
/// ```text
/// left=0, right=4: -5 + 6 = 1 < 7 → move left
/// left=1, right=4: -2 + 6 = 4 < 7 → move left
/// left=2, right=4:  3 + 6 = 9 > 7 → move right
/// left=2, right=3:  3 + 4 = 7 = 7 ✓ → return [2, 3]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the array with two pointers
/// - Space: O(1) — only using two pointer variables
pub fn pair_sum(nums: &[i32], target: i32) -> Vec<usize> {
    if nums.len() < 2 {
        return vec![];
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return vec![left, right];
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    vec![]
}

/// Triplet Sum (3Sum)
///
/// Given an array of integers, return all triplets [a, b, c] such that
/// a + b + c = 0. The solution must not contain duplicate triplets
/// (e.g., [1, 2, 3] and [2, 3, 1] are considered duplicate triplets).
/// If no such triplets are found, return an empty list.
///
/// Each triplet can be arranged in any order, and the output can be
/// returned in any order.
///
/// # Examples
///
/// ```text
/// Input:  nums = [0, -1, 2, -3, 1]
/// Output: [[-3, 1, 2], [-1, 0, 1]]
/// ```
///
/// For each element, we use two pointers to find pairs that sum to its
/// negation. Skip duplicates at each level to avoid duplicate triplets.
///
/// Example walkthrough for nums=[0, -1, 2, -3, 1]:
///
/// ```text
/// sorted: [-3, -1, 0, 1, 2]
///
/// i=0, nums[i]=-3, target=3:
///     left=1, right=4: -1 + 2 = 1 < 3 → move left
///     left=2, right=4:  0 + 2 = 2 < 3 → move left
///     left=3, right=4:  1 + 2 = 3 = 3 ✓ → found [-3, 1, 2]
///
/// i=1, nums[i]=-1, target=1:
///     left=2, right=4: 0 + 2 = 2 > 1 → move right
///     left=2, right=3: 0 + 1 = 1 = 1 ✓ → found [-1, 0, 1]
///
/// i=2, nums[i]=0 > 0 → break (early termination disabled here since 0 is not >0,
///                              continues but nothing new is found)
///
/// Answer: [[-3, 1, 2], [-1, 0, 1]]
/// ```
///
/// # Complexity
///
/// - Time: O(n^2) — sorting is O(n log n), then for each element we do a
///   two-pointer search O(n), giving O(n^2) total
/// - Space: O(1) — excluding the output, only using pointers (sorting may use
///   O(n) depending on implementation)
pub fn triplet_sum(nums: &mut [i32]) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }

    let pair_sum_sorted = |subset: &[i32], target: i32| -> Vec<(i32, i32)> {
        let mut left = 0;
        let mut right = subset.len() - 1;
        let mut result = vec![];

        while left < right {
            let sum = subset[left] + subset[right];
            if sum == target {
                result.push((subset[left], subset[right]));
                left += 1;

                while left < right && subset[left - 1] == subset[left] {
                    left += 1;
                }
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        result
    };

    let mut result = vec![];
    nums.sort();

    for i in 0..nums.len() {
        let val = nums[i];

        if val > 0 {
            break;
        }

        if i > 0 && val == nums[i - 1] {
            continue;
        }

        let pairs = pair_sum_sorted(&nums[i + 1..], -val);

        for pair in pairs {
            result.push(vec![val, pair.0, pair.1]);
        }
    }

    result
}

/// Is Palindrome Valid
///
/// A palindrome is a sequence of characters that reads the same forward and
/// backward. Given a string, determine if it's a palindrome after removing
/// all non-alphanumeric characters.
///
/// # Examples
///
/// ```text
/// Input:  s = "a dog, a panic in a pagoda"
/// Output: true
///
/// Input:  s = "abc123"
/// Output: false
/// ```
///
/// Use two pointers from both ends, skip non-alphanumeric chars,
/// compare case-insensitively.
///
/// Example walkthrough for s="a dog, a panic in a pagoda":
///
/// ```text
/// left=0 'a', right=25 'a' → match, move inward
/// left=1 ' ' → skip, left=2 'd'
/// right=24 'd' → match, move inward
/// left=3 'o', right=23 'o' → match
/// left=4 'g', right=22 'g' → match
/// ... continues matching all alphanumeric chars symmetrically ...
/// All pairs match → True
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass with two pointers
/// - Space: O(1) — only pointer variables
pub fn is_palindrome_valid(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();

    if s.is_empty() {
        return true;
    }

    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        while left < right && !s[left].is_alphanumeric() {
            left += 1;
        }

        while left < right && !s[right].is_alphanumeric() {
            right -= 1;
        }

        if s[left].to_ascii_lowercase() != s[right].to_ascii_lowercase() {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

/// Largest Container (Container With Most Water)
///
/// You are given an array of numbers, each representing the height of a
/// vertical line on a graph. A container can be formed with any pair of
/// these lines, along with the x-axis of the graph. Return the amount of
/// water which the largest container can hold.
///
/// # Examples
///
/// ```text
/// Input:  heights = [2, 7, 8, 3, 7, 6]
/// Output: 24
/// ```
///
/// Start pointers at both ends. The area is min(height) * width. Move the
/// pointer with the smaller height inward, since moving the taller one
/// can only decrease the area.
///
/// Example walkthrough for heights=[2, 7, 8, 3, 7, 6]:
///
/// ```text
/// left=0, right=5: min(2,6)*5 = 10, 2 < 6 → move left
/// left=1, right=5: min(7,6)*4 = 24 ★, 7 > 6 → move right
/// left=1, right=4: min(7,7)*3 = 21, 7 >= 7 → move right
/// left=1, right=3: min(7,3)*2 = 6, 7 > 3 → move right
/// left=1, right=2: min(7,8)*1 = 7, 7 < 8 → move left
/// left=2, right=2: done
///
/// Answer: 24
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass with two pointers
/// - Space: O(1) — only pointer variables
pub fn largest_container(heights: &[u32]) -> u32 {
    if heights.is_empty() {
        return 0;
    }

    let mut left = 0;
    let mut right = heights.len() - 1;
    let mut max_water = 0;

    while left < right {
        let min_height = heights[right].min(heights[left]);
        let curr_water = (right - left) as u32 * min_height;
        max_water = max_water.max(curr_water);

        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_water
}

/// Remove Element
///
/// Remove all occurrences of val in nums in-place. Return the number of
/// elements not equal to val. The first k elements of nums should contain
/// the non-val elements.
///
/// # Examples
///
/// ```text
/// Input:  nums = [3, 2, 2, 3], val = 3
/// Output: k = 2, nums[..k] = [2, 2]
///
/// Input:  nums = [0, 1, 2, 2, 3, 0, 4, 2], val = 2
/// Output: k = 5, nums[..k] = [0, 0, 1, 3, 4] (any order)
/// ```
///
/// Use two pointers: one from start, one from end. When we find val at
/// left pointer, swap with right pointer's element and shrink right.
///
/// Example walkthrough for nums=[3, 2, 2, 3], val=3:
///
/// ```text
/// left=0, right=3: nums[0]=3 == val → copy nums[3]=3, right=2
/// left=0, right=2: nums[0]=3 == val → copy nums[2]=2, right=1
/// left=0, right=1: nums[0]=2 != val → left=1
/// left=1, right=1: nums[1]=2 != val → left=2
/// left=2 > right=1 → done, k=2
/// nums[:2] = [2, 2] ✓
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the array
/// - Space: O(1) — in-place modification
pub fn remove_element(nums: &mut [i32], val: i32) -> usize {
    let mut p1 = 0;
    let mut p2 = nums.len();

    while p1 < p2 {
        if nums[p1] == val {
            nums[p1] = nums[p2 - 1];
            p2 -= 1;
        } else {
            p1 += 1;
        }
    }

    p1
}

/// Remove Duplicates from Sorted Array
///
/// Given a sorted array, remove duplicates in-place such that each unique
/// element appears only once. Return the number of unique elements k.
///
/// # Examples
///
/// ```text
/// Input:  nums = [1, 1, 2]
/// Output: k = 2, nums[..k] = [1, 2]
///
/// Input:  nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]
/// Output: k = 5, nums[..k] = [0, 1, 2, 3, 4]
/// ```
///
/// Use slow/fast pointers. Slow marks where to write next unique value.
/// Fast scans ahead. When fast finds a new value, write it at slow position.
///
/// Example walkthrough for nums=[0, 0, 1, 1, 1, 2, 2, 3, 3, 4]:
///
/// ```text
/// slow=1
/// fast=1: nums[1]=0 == nums[0]=0 → skip
/// fast=2: nums[2]=1 != nums[1]=0 → write 1 at slow=1, slow=2
/// fast=3: nums[3]=1 == nums[2]=1 → skip
/// fast=4: nums[4]=1 == nums[3]=1 → skip
/// fast=5: nums[5]=2 != nums[4]=1 → write 2 at slow=2, slow=3
/// fast=6: nums[6]=2 == nums[5]=2 → skip
/// fast=7: nums[7]=3 != nums[6]=2 → write 3 at slow=3, slow=4
/// fast=8: nums[8]=3 == nums[7]=3 → skip
/// fast=9: nums[9]=4 != nums[8]=3 → write 4 at slow=4, slow=5
/// Answer: k=5, nums[:5] = [0, 1, 2, 3, 4]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the array
/// - Space: O(1) — in-place modification
pub fn remove_duplicates(nums: &mut [i32]) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let mut p1 = 1;
    let mut prev_val = nums[0];

    for p2 in 1..nums.len() {
        if nums[p2] != prev_val {
            nums[p1] = nums[p2];
            p1 += 1;
            prev_val = nums[p2];
        }
    }

    p1
}

/// Rotate Array
///
/// Rotate the array to the right by k steps, in-place.
///
/// # Examples
///
/// ```text
/// Input:  nums = [1, 2, 3, 4, 5, 6, 7], k = 3
/// Output: [5, 6, 7, 1, 2, 3, 4]
///
/// Input:  nums = [-1, -100, 3, 99], k = 2
/// Output: [3, 99, -1, -100]
/// ```
///
/// Reverse entire array, then reverse first k elements, then reverse rest.
/// This works because reversing twice puts elements in the right order,
/// just shifted by k positions.
///
/// (Note: this implementation uses an alternative approach — rotating one
/// step at a time, k times — for simplicity. The walkthrough below shows
/// the canonical reverse-based approach since it's the algorithm taught
/// alongside this problem.)
///
/// Example walkthrough for nums=[1,2,3,4,5,6,7], k=3:
///
/// ```text
/// Step 1 - reverse all:    [7, 6, 5, 4, 3, 2, 1]
/// Step 2 - reverse [0:3]:  [5, 6, 7, 4, 3, 2, 1]
/// Step 3 - reverse [3:7]:  [5, 6, 7, 1, 2, 3, 4] ✓
/// ```
///
/// # Complexity
///
/// - Time: O(n*k) — this implementation rotates one step at a time, k times.
///   The reverse-based approach is O(n).
/// - Space: O(1) — in-place
pub fn rotate(nums: &mut [i32], k: usize) {
    if nums.is_empty() {
        return;
    }

    for _ in 0..k {
        let mut prev = nums[nums.len() - 1];
        for i in 0..nums.len() {
            let tmp = nums[i];
            nums[i] = prev;
            prev = tmp;
        }
    }
}

/// Remove Duplicates from Sorted Array II
///
/// Given a sorted array, remove duplicates in-place such that each unique
/// element appears at most twice. Return the number of elements k.
///
/// # Examples
///
/// ```text
/// Input:  nums = [1, 1, 1, 2, 2, 3]
/// Output: k = 5, nums[..k] = [1, 1, 2, 2, 3]
///
/// Input:  nums = [0, 0, 1, 1, 1, 1, 2, 3, 3]
/// Output: k = 7, nums[..k] = [0, 0, 1, 1, 2, 3, 3]
/// ```
///
/// Track count of current value. Only write when count <= 2.
/// Use slow/fast pointers where slow marks the write position.
///
/// Example walkthrough for nums=[1, 1, 1, 2, 2, 3]:
///
/// ```text
/// slow=1, count=1
/// fast=1: 1 == 1, count=1 < 2 → write, slow=2, count=2
/// fast=2: 1 == 1, count=2, not < 2 → skip
/// fast=3: 2 != 1 → write, slow=3, count=1
/// fast=4: 2 == 2, count=1 < 2 → write, slow=4, count=2
/// fast=5: 3 != 2 → write, slow=5, count=1
/// Answer: k=5, nums[:5] = [1, 1, 2, 2, 3]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the array
/// - Space: O(1) — in-place modification
pub fn remove_duplicates_ii(nums: &mut [i32]) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let mut p1 = 1;
    let mut prev_val = nums[0];
    let mut count = 1;

    for p2 in 1..nums.len() {
        if prev_val == nums[p2] && count == 1 {
            nums[p1] = nums[p2];
            count += 1;
            p1 += 1;
        } else if prev_val != nums[p2] {
            nums[p1] = nums[p2];
            p1 += 1;
            count = 1;
            prev_val = nums[p2];
        }
        // If same value and count >= 2, skip
    }

    p1
}

/// Trapping Rain Water
///
/// Given n non-negative integers representing an elevation map where the
/// width of each bar is 1, compute how much water it can trap after raining.
///
/// # Examples
///
/// ```text
/// Input:  height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
/// Output: 6
///
/// Input:  height = [4, 2, 0, 3, 2, 5]
/// Output: 9
/// ```
///
/// Water at each position = min(max_left, max_right) - height.
/// Use two pointers tracking max heights from each side. Process the
/// smaller side since it's the bottleneck for water level.
///
/// Example walkthrough for height=[4, 2, 0, 3, 2, 5]:
///
/// ```text
/// left=0, right=5, left_max=4, right_max=5
/// left_max(4) < right_max(5) → process left side
///     left=1, left_max=max(4,2)=4, water += 4-2 = 2
///     left=2, left_max=max(4,0)=4, water += 4-0 = 4 (total=6)
///     left=3, left_max=max(4,3)=4, water += 4-3 = 1 (total=7)
/// left_max(4) < right_max(5) → process left side
///     left=4, left_max=max(4,2)=4, water += 4-2 = 2 (total=9)
/// left_max(4) < right_max(5) → process left side
///     left=5, left=right → done
///
/// Answer: 9
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass with two pointers
/// - Space: O(1) — only pointer variables
pub fn trap(height: &[i32]) -> i32 {
    // Need at least 3 bars to trap water (two walls + valley)
    // Also prevents underflow: height.len() - 1 panics if empty
    if height.len() < 3 {
        return 0;
    }

    let mut left = 0;
    let mut right = height.len() - 1;
    let mut left_max = height[left];
    let mut right_max = height[right];
    let mut water = 0;

    while left < right {
        if left_max < right_max {
            // Left side is the bottleneck
            left += 1;
            left_max = left_max.max(height[left]);
            water += left_max - height[left];
        } else {
            // Right side is the bottleneck
            right -= 1;
            right_max = right_max.max(height[right]);
            water += right_max - height[right];
        }
    }

    water
}

// =============================================================================
// Sliding Windows
// =============================================================================

/// Substring Anagrams
///
/// Count the number of substrings in s that are anagrams of t.
///
/// # Examples
///
/// ```text
/// Input:  s = "caabab", t = "aba"
/// Output: 2
/// (Anagrams at index 1 ("aab") and index 2 ("aba"))
/// ```
///
/// Fixed window of size len(t). Compare character frequencies.
///
/// Example walkthrough for s="caabab", t="aba":
///
/// ```text
/// expected freq for "aba": {a:2, b:1}
///
/// i=0: window="c"     → not full yet (need size 3)
/// i=1: window="ca"    → not full yet
/// i=2: window="caa"   → full! freq={c:1,a:2} ≠ expected, remove 'c'
/// i=3: window="aab"   → freq={a:2,b:1} = expected ✓ count=1, remove 'a'
/// i=4: window="aba"   → freq={a:2,b:1} = expected ✓ count=2, remove 'a'
/// i=5: window="bab"   → freq={a:1,b:2} ≠ expected
///
/// Answer: 2
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass with fixed window
/// - Space: O(1) — fixed size arrays (26 letters)
pub fn count_anagrams(s: &str, t: &str) -> usize {
    if t.len() > s.len() {
        return 0;
    }

    let mut expected = [0; 26];
    let mut window = [0; 26];

    let mut left = 0;
    let mut right = 0;
    let mut res = 0;

    for c in t.chars() {
        expected[c as usize - 'a' as usize] += 1;
    }

    while right < s.len() {
        let right_char = s.chars().nth(right).unwrap();
        window[right_char as usize - 'a' as usize] += 1;

        if right - left + 1 == t.len() {
            if expected == window {
                res += 1;
            }
            let left_char = s.chars().nth(left).unwrap();
            window[left_char as usize - 'a' as usize] -= 1;
            left += 1;
        }

        right += 1;
    }

    res
}

/// Longest Substring With Unique Characters
///
/// Find the length of the longest substring with all unique characters.
///
/// # Examples
///
/// ```text
/// Input:  s = "abcba"
/// Output: 3
/// ("abc" or "cba" are the longest with unique chars)
/// ```
///
/// Variable window: expand right, shrink left when duplicate found.
/// Track last seen position of each character.
///
/// Example walkthrough for s="abcba":
///
/// ```text
/// i=0 'a': last_seen={}, no dup → window [0,0]="a", len=1, last_seen={a:0}
/// i=1 'b': no dup → window [0,1]="ab", len=2, last_seen={a:0,b:1}
/// i=2 'c': no dup → window [0,2]="abc", len=3 ★, last_seen={a:0,b:1,c:2}
/// i=3 'b': dup! b was at 1, jump left to 2 → window [2,3]="cb", len=2
/// i=4 'a': a was at 0, but 0 < left(2) so NOT in window, no jump
///          → window [2,4]="cba", len=3
///
/// Answer: 3
/// ```
///
/// # Complexity
///
/// - Time: O(n) — each char visited at most twice
/// - Space: O(min(n, 26)) — hashmap of char positions
pub fn longest_unique_substring(s: &str) -> usize {
    let mut map = HashMap::new();
    let mut max = 0;
    let mut left = 0;
    let mut right = 0;

    while right < s.len() {
        let char = s.chars().nth(right).unwrap();
        if let Some(&prev_index) = map.get(&char) {
            left = left.max(prev_index + 1);
        }

        max = max.max(right - left + 1);
        map.insert(char, right);
        right += 1;
    }

    max
}

/// Longest Uniform Substring After Replacements
///
/// Find the longest substring where all characters are the same,
/// if you can replace up to k characters.
///
/// # Examples
///
/// ```text
/// Input:  s = "aabcdcca", k = 2
/// Output: 5
/// (Replace 'b' and 'd' with 'c' to get "ccccc")
/// ```
///
/// Key insight: window is valid if (window_size - most_frequent_char) <= k.
/// That's the number of chars we need to replace.
///
/// Example walkthrough for s="aabcdcca", k=2:
///
/// ```text
/// Keep the most frequent char, replace the rest.
/// Valid if: window_size - max_freq <= k
///
/// i=0 'a': freq={a:1}, max_freq=1, window="a", replace=1-1=0 ≤2 ✓
/// i=1 'a': freq={a:2}, max_freq=2, window="aa", replace=2-2=0 ≤2 ✓
/// i=2 'b': freq={a:2,b:1}, max_freq=2, window="aab", replace=3-2=1 ≤2 ✓
/// i=3 'c': freq={a:2,b:1,c:1}, max_freq=2, replace=4-2=2 ≤2 ✓
/// i=4 'd': freq={a:2,b:1,c:1,d:1}, max_freq=2, replace=5-2=3 >2 ✗
///          shrink! remove 'a', left=1, replace=4-1=3 >2 ✗
///          shrink! remove 'a', left=2, replace=3-1=2 ≤2 ✓
/// i=5 'c': freq={b:1,c:2,d:1}, max_freq=2, replace=4-2=2 ≤2 ✓
/// i=6 'c': freq={b:1,c:3,d:1}, max_freq=3, replace=5-3=2 ≤2 ✓ len=5 ★
/// i=7 'a': freq={a:1,b:1,c:3,d:1}, max_freq=3, replace=6-3=3 >2 ✗
///          shrink! ...
///
/// Answer: 5 (make "ccccc" by replacing b,d with c)
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass
/// - Space: O(26) — frequency map
pub fn longest_uniform_substring(s: &str, k: usize) -> usize {
    let s: Vec<char> = s.chars().collect();

    if s.is_empty() {
        return 0;
    }

    let mut max = 0;
    let mut right = 0;
    let mut left = 0;
    let mut highest_freq = 0;
    let mut freqs = HashMap::new();

    while right < s.len() {
        let curr_freq = freqs.entry(&s[right]).and_modify(|c| *c += 1).or_insert(1);
        highest_freq = highest_freq.max(*curr_freq);

        let num_chars_to_replace = right - left + 1 - highest_freq;
        if num_chars_to_replace > k {
            freqs.entry(&s[left]).and_modify(|c| *c -= 1);
            left += 1;
        }

        max = right - left + 1;
        right += 1;
    }

    max
}

// =============================================================================
// Prefix Sums
// =============================================================================

/// Sum Between Range
///
/// Given an integer array, return the sum of values between two indexes.
///
/// # Examples
///
/// ```text
/// Input:  rs = RangeSum([3, -7, 6, 0, -2, 5])
///         rs.sum_range(0, 3)  // 3 + (-7) + 6 + 0
/// Output: 2
///
/// Input:  rs.sum_range(2, 4)  // 6 + 0 + (-2)
/// Output: 4
///
/// Input:  rs.sum_range(2, 2)  // just 6
/// Output: 6
/// ```
///
/// Build prefix sums array where prefix[i] = sum of nums[0..i].
/// Then sum(left, right) = prefix[right] - prefix[left-1]. This
/// turns any range sum query into a single subtraction.
///
/// Example walkthrough for nums=[3, -7, 6, 0, -2, 5]:
///
/// ```text
/// Build prefix: [3, -4, 2, 2, 0, 5]
///
/// sum_range(0, 3) → prefix[3] = 2 (left=0, return directly)
/// sum_range(2, 4) → prefix[4] - prefix[1] = 0 - (-4) = 4
/// sum_range(2, 2) → prefix[2] - prefix[1] = 2 - (-4) = 6
/// ```
///
/// # Complexity
///
/// - Time: O(n) preprocessing, O(1) per query
/// - Space: O(n) — prefix array
pub struct RangeSum {
    prefix_sums: Vec<i32>,
}

impl RangeSum {
    pub fn new(nums: &[i32]) -> Self {
        let mut sum = 0;
        let mut prefix_sums = vec![0; nums.len()];

        for (i, num) in nums.iter().enumerate() {
            sum += num;
            prefix_sums[i] = sum;
        }

        Self { prefix_sums }
    }

    pub fn sum_range(&self, left: usize, right: usize) -> i32 {
        if left == 0 {
            self.prefix_sums[right]
        } else {
            self.prefix_sums[right] - self.prefix_sums[left - 1]
        }
    }
}

/// K-Sum Subarrays
///
/// Find the number of subarrays that sum to k.
///
/// # Examples
///
/// ```text
/// Input:  nums = [1, 2, -1, 1, 2], k = 3
/// Output: 3
/// ([1,2] at 0-1, [1,2,-1,1] at 0-3, [1,2] at 3-4)
/// ```
///
/// Build a prefix sums array with a leading zero so that the sum of any
/// subarray nums[i..j] equals prefix[j+1] - prefix[i]. Then check all
/// pairs (i, j) to count how many equal k.
///
/// Example walkthrough for nums=[1, 2, -1, 1, 2], k=3:
///
/// ```text
/// prefix = [0, 1, 3, 2, 3, 5]
///
/// Check all pairs prefix[j] - prefix[i]:
///     prefix[2]-prefix[0] = 3-0 = 3 ✓  → subarray [1,2]
///     prefix[4]-prefix[0] = 3-0 = 3 ✓  → subarray [1,2,-1,1]
///     prefix[5]-prefix[3] = 5-2 = 3 ✓  → subarray [1,2]
///
/// Answer: 3
/// ```
///
/// Note: Can be optimized to O(n) with hashmap, but this version
/// is clearer for interviews.
///
/// # Complexity
///
/// - Time: O(n^2) — check all subarrays using prefix sums
/// - Space: O(n) — prefix array
pub fn k_sum_subarrays(nums: &[i32], k: i32) -> usize {
    let mut prefix_sum = vec![0];
    let mut sum = 0;
    for num in nums {
        sum += num;
        prefix_sum.push(sum);
    }

    let mut ans = 0;
    for j in 1..prefix_sum.len() {
        for i in 1..=j {
            if prefix_sum[j] - prefix_sum[i - 1] == k {
                ans += 1;
            }
        }
    }

    ans
}

/// Product Array Without Current Element
///
/// Return an array where result[i] = product of all elements except nums[i].
///
/// # Examples
///
/// ```text
/// Input:  nums = [2, 3, 1, 4, 5]
/// Output: [60, 40, 120, 30, 24]
/// (result[0] = 3*1*4*5 = 60, everything except 2)
/// ```
///
/// Strategy: For each position, we need product of LEFT side * RIGHT side.
/// Build prefix products from left, and prefix products from right. Then
/// multiply them together for each index.
///
/// Example walkthrough for nums=[2, 3, 1, 4, 5]:
///
/// ```text
/// left products:  [1, 2, 6, 6, 24]
///     left[0]=1, left[1]=2, left[2]=2*3=6, left[3]=6*1=6, left[4]=6*4=24
///
/// right products: [60, 20, 20, 5, 1]
///     right[4]=1, right[3]=5, right[2]=4*5=20, right[1]=1*20=20, right[0]=3*20=60
///
/// result = left[i] * right[i]:
///     [1*60, 2*20, 6*20, 6*5, 24*1] = [60, 40, 120, 30, 24]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — two passes
/// - Space: O(n) — for left/right arrays
pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    if nums.is_empty() {
        return Vec::new();
    }

    let mut left = vec![1; nums.len()];
    for i in 1..nums.len() {
        left[i] = left[i - 1] * nums[i - 1];
    }

    let mut right = vec![1; nums.len()];
    for i in (0..nums.len() - 1).rev() {
        right[i] = right[i + 1] * nums[i + 1];
    }

    let mut ans = vec![0; nums.len()];
    for i in 0..nums.len() {
        ans[i] = left[i] * right[i];
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Two Pointers tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pair_sum_example_1() {
        let nums = vec![-5, -2, 3, 4, 6];
        let target = 7;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_pair_sum_example_2() {
        let nums = vec![1, 1, 1];
        let target = 2;
        let result = pair_sum(&nums, target);
        // Any valid pair is acceptable: [0,1], [0,2], or [1,2]
        assert!(
            result == vec![0, 1]
                || result == vec![1, 0]
                || result == vec![0, 2]
                || result == vec![2, 0]
                || result == vec![1, 2]
                || result == vec![2, 1]
        );
    }

    #[test]
    fn test_pair_sum_no_solution() {
        let nums = vec![1, 2, 3];
        let target = 10;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_pair_sum_negative_numbers() {
        let nums = vec![-10, -5, 0, 5, 10];
        let target = 0;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![0, 4]);
    }

    #[test]
    fn test_pair_sum_two_elements() {
        let nums = vec![1, 9];
        let target = 10;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_pair_sum_empty_array() {
        let nums = vec![];
        let target = 5;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_pair_sum_single_element() {
        let nums = vec![5];
        let target = 5;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_pair_sum_large_numbers() {
        let nums = vec![-1000000, 0, 1000000];
        let target = 0;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn test_triplet_sum_example() {
        let mut nums = vec![0, -1, 2, -3, 1];
        let mut result = triplet_sum(&mut nums);
        result.sort();
        let mut expected = vec![vec![-3, 1, 2], vec![-1, 0, 1]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_triplet_sum_empty_array() {
        let mut nums: Vec<i32> = vec![];
        let result = triplet_sum(&mut nums);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_triplet_sum_single_element() {
        let mut nums = vec![0];
        let result = triplet_sum(&mut nums);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_triplet_sum_two_elements() {
        let mut nums = vec![1, -1];
        let result = triplet_sum(&mut nums);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_triplet_sum_all_zeros() {
        let mut nums = vec![0, 0, 0];
        let result = triplet_sum(&mut nums);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_triplet_sum_no_solution() {
        let mut nums = vec![1, 0, 1];
        let result = triplet_sum(&mut nums);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_triplet_sum_with_duplicates() {
        let mut nums = vec![0, 0, 1, -1, 1, -1];
        let mut result = triplet_sum(&mut nums);
        // Should only return one triplet [-1, 0, 1] without duplicates
        result.sort();
        assert_eq!(result, vec![vec![-1, 0, 1]]);
    }

    #[test]
    fn test_is_palindrome_valid_example_1() {
        assert!(is_palindrome_valid("a dog, a panic in a pagoda"));
    }

    #[test]
    fn test_is_palindrome_valid_example_2() {
        assert!(!is_palindrome_valid("abc123"));
    }

    #[test]
    fn test_is_palindrome_valid_empty_string() {
        assert!(is_palindrome_valid(""));
    }

    #[test]
    fn test_is_palindrome_valid_single_char() {
        assert!(is_palindrome_valid("a"));
    }

    #[test]
    fn test_is_palindrome_valid_two_chars_palindrome() {
        assert!(is_palindrome_valid("aa"));
    }

    #[test]
    fn test_is_palindrome_valid_two_chars_not_palindrome() {
        assert!(!is_palindrome_valid("ab"));
    }

    #[test]
    fn test_is_palindrome_valid_no_alphanumeric() {
        assert!(is_palindrome_valid(" ' (?)"));
    }

    #[test]
    fn test_is_palindrome_valid_date_palindrome() {
        assert!(is_palindrome_valid("12.02.2021"));
    }

    #[test]
    fn test_is_palindrome_valid_date_not_palindrome() {
        assert!(!is_palindrome_valid("21.02.2021"));
    }

    #[test]
    fn test_is_palindrome_valid_hello_world() {
        assert!(!is_palindrome_valid("hello, world!"));
    }

    #[test]
    fn test_largest_container_example() {
        assert_eq!(largest_container(&[2, 7, 8, 3, 7, 6]), 24);
    }

    #[test]
    fn test_largest_container_empty() {
        assert_eq!(largest_container(&[]), 0);
    }

    #[test]
    fn test_largest_container_single_element() {
        assert_eq!(largest_container(&[1]), 0);
    }

    #[test]
    fn test_largest_container_no_water() {
        assert_eq!(largest_container(&[0, 1, 0]), 0);
    }

    #[test]
    fn test_largest_container_same_heights() {
        assert_eq!(largest_container(&[3, 3, 3, 3]), 9);
    }

    #[test]
    fn test_largest_container_increasing() {
        assert_eq!(largest_container(&[1, 2, 3]), 2);
    }

    #[test]
    fn test_largest_container_decreasing() {
        assert_eq!(largest_container(&[3, 2, 1]), 2);
    }

    #[test]
    fn test_remove_element_example1() {
        let mut nums = vec![3, 2, 2, 3];
        let k = remove_element(&mut nums, 3);
        assert_eq!(k, 2);
        nums[..k].sort();
        assert_eq!(&nums[..k], &[2, 2]);
    }

    #[test]
    fn test_remove_element_example2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let k = remove_element(&mut nums, 2);
        assert_eq!(k, 5);
        nums[..k].sort();
        assert_eq!(&nums[..k], &[0, 0, 1, 3, 4]);
    }

    #[test]
    fn test_remove_element_empty() {
        let mut nums: Vec<i32> = vec![];
        let k = remove_element(&mut nums, 1);
        assert_eq!(k, 0);
    }

    #[test]
    fn test_remove_element_all_same() {
        let mut nums = vec![3, 3, 3, 3];
        let k = remove_element(&mut nums, 3);
        assert_eq!(k, 0);
    }

    #[test]
    fn test_remove_element_none_match() {
        let mut nums = vec![1, 2, 3, 4];
        let k = remove_element(&mut nums, 5);
        assert_eq!(k, 4);
        nums[..k].sort();
        assert_eq!(&nums[..k], &[1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_duplicates_example1() {
        let mut nums = vec![1, 1, 2];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k], &[1, 2]);
    }

    #[test]
    fn test_remove_duplicates_example2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_duplicates_empty() {
        let mut nums: Vec<i32> = vec![];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 0);
    }

    #[test]
    fn test_remove_duplicates_single() {
        let mut nums = vec![1];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(&nums[..k], &[1]);
    }

    #[test]
    fn test_remove_duplicates_no_duplicates() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_duplicates_all_same() {
        let mut nums = vec![5, 5, 5, 5];
        let k = remove_duplicates(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(&nums[..k], &[5]);
    }

    #[test]
    fn test_rotate_example1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rotate_example2() {
        let mut nums = vec![-1, -100, 3, 99];
        rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn test_rotate_k_zero() {
        let mut nums = vec![1, 2, 3];
        rotate(&mut nums, 0);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_k_equals_len() {
        let mut nums = vec![1, 2, 3];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![1, 2, 3]); // Full rotation = no change
    }

    #[test]
    fn test_rotate_k_greater_than_len() {
        let mut nums = vec![1, 2, 3];
        rotate(&mut nums, 4); // Same as k=1
        assert_eq!(nums, vec![3, 1, 2]);
    }

    #[test]
    fn test_rotate_single() {
        let mut nums = vec![1];
        rotate(&mut nums, 5);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_remove_duplicates_ii_example1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = remove_duplicates_ii(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k], &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_remove_duplicates_ii_example2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let k = remove_duplicates_ii(&mut nums);
        assert_eq!(k, 7);
        assert_eq!(&nums[..k], &[0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_remove_duplicates_ii_empty() {
        let mut nums: Vec<i32> = vec![];
        let k = remove_duplicates_ii(&mut nums);
        assert_eq!(k, 0);
    }

    #[test]
    fn test_remove_duplicates_ii_single() {
        let mut nums = vec![1];
        let k = remove_duplicates_ii(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(&nums[..k], &[1]);
    }

    #[test]
    fn test_remove_duplicates_ii_two_same() {
        let mut nums = vec![1, 1];
        let k = remove_duplicates_ii(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k], &[1, 1]);
    }

    #[test]
    fn test_remove_duplicates_ii_all_same() {
        let mut nums = vec![5, 5, 5, 5, 5];
        let k = remove_duplicates_ii(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k], &[5, 5]);
    }

    #[test]
    fn test_trap_example1() {
        assert_eq!(trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_trap_example2() {
        assert_eq!(trap(&[4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test_trap_empty() {
        assert_eq!(trap(&[]), 0);
    }

    #[test]
    fn test_trap_no_trap() {
        assert_eq!(trap(&[1, 2, 3, 4, 5]), 0); // increasing
        assert_eq!(trap(&[5, 4, 3, 2, 1]), 0); // decreasing
    }

    #[test]
    fn test_trap_single_valley() {
        assert_eq!(trap(&[3, 0, 3]), 3);
    }

    #[test]
    fn test_trap_flat() {
        assert_eq!(trap(&[2, 2, 2, 2]), 0);
    }

    // -------------------------------------------------------------------------
    // Sliding Windows tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_count_anagrams_example() {
        assert_eq!(count_anagrams("caabab", "aba"), 2);
    }

    #[test]
    fn test_count_anagrams_no_match() {
        assert_eq!(count_anagrams("abcdef", "xyz"), 0);
    }

    #[test]
    fn test_count_anagrams_all_same_char() {
        assert_eq!(count_anagrams("aaaa", "aa"), 3);
    }

    #[test]
    fn test_count_anagrams_exact_match() {
        assert_eq!(count_anagrams("abc", "abc"), 1);
    }

    #[test]
    fn test_count_anagrams_single_char() {
        assert_eq!(count_anagrams("ababa", "a"), 3);
    }

    #[test]
    fn test_count_anagrams_t_longer_than_s() {
        assert_eq!(count_anagrams("ab", "abc"), 0);
    }

    #[test]
    fn test_count_anagrams_empty_s() {
        assert_eq!(count_anagrams("", "abc"), 0);
    }

    #[test]
    fn test_count_anagrams_empty_t() {
        assert_eq!(count_anagrams("abc", ""), 0);
    }

    #[test]
    fn test_count_anagrams_multiple_matches() {
        // "ab", "ba", "ab" are all anagrams of "ab"
        assert_eq!(count_anagrams("abab", "ab"), 3);
    }

    #[test]
    fn test_count_anagrams_overlapping() {
        assert_eq!(count_anagrams("cbaebabacd", "abc"), 2);
    }

    #[test]
    fn test_longest_unique_substring_example() {
        assert_eq!(longest_unique_substring("abcba"), 3);
    }

    #[test]
    fn test_longest_unique_substring_all_unique() {
        assert_eq!(longest_unique_substring("abcdef"), 6);
    }

    #[test]
    fn test_longest_unique_substring_all_same() {
        assert_eq!(longest_unique_substring("aaaa"), 1);
    }

    #[test]
    fn test_longest_unique_substring_empty() {
        assert_eq!(longest_unique_substring(""), 0);
    }

    #[test]
    fn test_longest_unique_substring_single_char() {
        assert_eq!(longest_unique_substring("a"), 1);
    }

    #[test]
    fn test_longest_unique_substring_two_chars() {
        assert_eq!(longest_unique_substring("ab"), 2);
    }

    #[test]
    fn test_longest_unique_substring_repeating_pattern() {
        assert_eq!(longest_unique_substring("abcabcbb"), 3);
    }

    #[test]
    fn test_longest_unique_substring_end_longest() {
        assert_eq!(longest_unique_substring("aabcdef"), 6);
    }

    #[test]
    fn test_longest_unique_substring_middle_longest() {
        assert_eq!(longest_unique_substring("aaabcdefa"), 6);
    }

    #[test]
    fn test_longest_uniform_substring_example() {
        assert_eq!(longest_uniform_substring("aabcdcca", 2), 5);
    }

    #[test]
    fn test_longest_uniform_substring_no_replacements() {
        assert_eq!(longest_uniform_substring("aaabbb", 0), 3);
    }

    #[test]
    fn test_longest_uniform_substring_all_same() {
        assert_eq!(longest_uniform_substring("aaaa", 2), 4);
    }

    #[test]
    fn test_longest_uniform_substring_all_different() {
        assert_eq!(longest_uniform_substring("abcd", 2), 3);
    }

    #[test]
    fn test_longest_uniform_substring_k_equals_len() {
        assert_eq!(longest_uniform_substring("abcd", 4), 4);
    }

    #[test]
    fn test_longest_uniform_substring_empty() {
        assert_eq!(longest_uniform_substring("", 2), 0);
    }

    #[test]
    fn test_longest_uniform_substring_single_char() {
        assert_eq!(longest_uniform_substring("a", 2), 1);
    }

    #[test]
    fn test_longest_uniform_substring_two_chars() {
        assert_eq!(longest_uniform_substring("ab", 1), 2);
    }

    #[test]
    fn test_longest_uniform_substring_alternating() {
        assert_eq!(longest_uniform_substring("ababab", 2), 5);
    }

    // -------------------------------------------------------------------------
    // Prefix Sums tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_range_sum_example() {
        let rs = RangeSum::new(&[3, -7, 6, 0, -2, 5]);
        assert_eq!(rs.sum_range(0, 3), 2);
        assert_eq!(rs.sum_range(2, 4), 4);
        assert_eq!(rs.sum_range(2, 2), 6);
    }

    #[test]
    fn test_range_sum_single_element() {
        let rs = RangeSum::new(&[5]);
        assert_eq!(rs.sum_range(0, 0), 5);
    }

    #[test]
    fn test_range_sum_full_array() {
        let rs = RangeSum::new(&[1, 2, 3, 4, 5]);
        assert_eq!(rs.sum_range(0, 4), 15);
    }

    #[test]
    fn test_range_sum_negative() {
        let rs = RangeSum::new(&[-1, -2, -3, -4]);
        assert_eq!(rs.sum_range(0, 3), -10);
        assert_eq!(rs.sum_range(1, 2), -5);
    }

    #[test]
    fn test_range_sum_mixed() {
        let rs = RangeSum::new(&[1, -1, 1, -1, 1]);
        assert_eq!(rs.sum_range(0, 4), 1);
        assert_eq!(rs.sum_range(0, 1), 0);
        assert_eq!(rs.sum_range(0, 3), 0);
    }

    #[test]
    fn test_range_sum_multiple_queries() {
        let rs = RangeSum::new(&[10, 20, 30, 40, 50]);
        assert_eq!(rs.sum_range(0, 0), 10);
        assert_eq!(rs.sum_range(1, 1), 20);
        assert_eq!(rs.sum_range(4, 4), 50);
        assert_eq!(rs.sum_range(0, 2), 60);
        assert_eq!(rs.sum_range(2, 4), 120);
    }

    #[test]
    fn test_k_sum_subarrays_example() {
        assert_eq!(k_sum_subarrays(&[1, 2, -1, 1, 2], 3), 3);
    }

    #[test]
    fn test_k_sum_subarrays_single() {
        assert_eq!(k_sum_subarrays(&[3], 3), 1);
        assert_eq!(k_sum_subarrays(&[5], 3), 0);
    }

    #[test]
    fn test_k_sum_subarrays_all_zeros() {
        assert_eq!(k_sum_subarrays(&[0, 0, 0], 0), 6);
    }

    #[test]
    fn test_k_sum_subarrays_negative_k() {
        assert_eq!(k_sum_subarrays(&[1, -1, -1, 1], -1), 4);
    }

    #[test]
    fn test_k_sum_subarrays_whole_array() {
        assert_eq!(k_sum_subarrays(&[1, 2, 3], 6), 1);
    }

    #[test]
    fn test_k_sum_subarrays_no_match() {
        assert_eq!(k_sum_subarrays(&[1, 2, 3], 10), 0);
    }

    #[test]
    fn test_k_sum_subarrays_multiple_same() {
        assert_eq!(k_sum_subarrays(&[1, 1, 1], 2), 2);
    }

    #[test]
    fn test_product_except_self_example() {
        assert_eq!(
            product_except_self(&[2, 3, 1, 4, 5]),
            vec![60, 40, 120, 30, 24]
        );
    }

    #[test]
    fn test_product_except_self_with_zero() {
        assert_eq!(product_except_self(&[1, 2, 0, 4]), vec![0, 0, 8, 0]);
    }

    #[test]
    fn test_product_except_self_two_zeros() {
        assert_eq!(product_except_self(&[0, 2, 0, 4]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_product_except_self_two_elements() {
        assert_eq!(product_except_self(&[3, 5]), vec![5, 3]);
    }

    #[test]
    fn test_product_except_self_negatives() {
        assert_eq!(product_except_self(&[-1, 2, -3, 4]), vec![-24, 12, -8, 6]);
    }

    #[test]
    fn test_product_except_self_all_ones() {
        assert_eq!(product_except_self(&[1, 1, 1, 1]), vec![1, 1, 1, 1]);
    }
}
