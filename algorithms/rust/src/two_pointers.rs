/// Pair Sum - Sorted
///
/// Given an array of integers sorted in ascending order and a target value, return the indexes
/// of any pair of numbers in the array that sum to the target. The order of the indexes in the
/// result doesn't matter. If no pair is found, return an empty array.
///
/// # Examples
///
/// Example 1:
/// ```text
/// Input: nums = [-5, -2, 3, 4, 6], target = 7
/// Output: [2, 3]
/// Explanation: nums[2] + nums[3] = 3 + 4 = 7
/// ```
///
/// Example 2:
/// ```text
/// Input: nums = [1, 1, 1], target = 2
/// Output: [0, 1]
/// Explanation: Other valid outputs could be [1, 0], [0, 2], [2, 0], [1, 2] or [2, 1]
/// ```
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

/// Triplet Sum
///
/// Given an array of integers, return all triplets [a, b, c] such that a + b + c = 0. The
/// solution must not contain duplicate triplets (e.g., [1, 2, 3] and [2, 3, 1] are considered
/// duplicate triplets). If no such triplets are found, return an empty array.
///
/// Each triplet can be arranged in any order, and the output can be returned in any order.
///
/// # Example
///
/// ```text
/// Input: nums = [0, -1, 2, -3, 1]
/// Output: [[-3, 1, 2], [-1, 0, 1]]
/// ```
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
/// A palindrome is a sequence of characters that reads the same forward and backward.
/// Given a string, determine if it's a palindrome after removing all non-alphanumeric
/// characters. A character is alphanumeric if it's either a letter or a number.
///
/// # Examples
///
/// Example 1:
/// ```text
/// Input: s = "a]dog I a panic in a pagoda..."
/// Output: true
/// ```
///
/// Example 2:
/// ```text
/// Input: s = "abc123"
/// Output: false
/// ```
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

/// Largest Container
///
/// You are given an array of numbers, each representing the height of a vertical line on a graph.
/// A container can be formed with any pair of these lines, along with the x-axis of the graph.
/// Return the amount of water which the largest container can hold.
///
/// # Example
///
/// ```text
/// Input: heights = [2, 7, 8, 3, 7, 6]
/// Output: 24
/// ```
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
/// Remove all occurrences of val in nums in-place. Return the number of elements
/// not equal to val. The first k elements of nums should contain the non-val elements.
///
/// # Example 1
///
/// ```text
/// Input: nums = [3, 2, 2, 3], val = 3
/// Output: 2, nums = [2, 2, _, _]
/// ```
///
/// # Example 2
///
/// ```text
/// Input: nums = [0, 1, 2, 2, 3, 0, 4, 2], val = 2
/// Output: 5, nums = [0, 1, 4, 0, 3, _, _, _]
/// ```
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
/// Given a sorted array, remove duplicates in-place such that each unique element
/// appears only once. Return the number of unique elements k.
///
/// # Example 1
///
/// ```text
/// Input: nums = [1, 1, 2]
/// Output: 2, nums = [1, 2, _]
/// ```
///
/// # Example 2
///
/// ```text
/// Input: nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]
/// Output: 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
/// ```
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
/// Rotate the array to the right by k steps.
///
/// # Example 1
///
/// ```text
/// Input: nums = [1, 2, 3, 4, 5, 6, 7], k = 3
/// Output: [5, 6, 7, 1, 2, 3, 4]
/// ```
///
/// # Example 2
///
/// ```text
/// Input: nums = [-1, -100, 3, 99], k = 2
/// Output: [3, 99, -1, -100]
/// ```
// O(n*k) time, O(1) space - rotate one step at a time
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
/// Given a sorted array, remove duplicates in-place such that each unique element
/// appears at most twice. Return the number of elements k.
///
/// # Example 1
///
/// ```text
/// Input: nums = [1, 1, 1, 2, 2, 3]
/// Output: 5, nums = [1, 1, 2, 2, 3, _]
/// ```
///
/// # Example 2
///
/// ```text
/// Input: nums = [0, 0, 1, 1, 1, 1, 2, 3, 3]
/// Output: 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]
/// ```
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
/// Given n non-negative integers representing an elevation map where the width
/// of each bar is 1, compute how much water it can trap after raining.
///
/// # Example 1
///
/// ```text
/// Input: height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
/// Output: 6
/// ```
///
/// # Example 2
///
/// ```text
/// Input: height = [4, 2, 0, 3, 2, 5]
/// Output: 9
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
