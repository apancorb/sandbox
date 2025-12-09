/// Find the Insertion Index
///
/// You are given a sorted array that contains unique values, along with an integer target.
/// - If the array contains the target value, return its index.
/// - Otherwise, return the insertion index. This is the index where the target would be if it
///   were inserted in order, maintaining the sorted sequence of the array.
///
/// # Examples
///
/// Example 1:
/// ```text
/// Input: nums = [1, 2, 4, 5, 7, 8, 9], target = 4
/// Output: 2
/// ```
///
/// Example 2:
/// ```text
/// Input: nums = [1, 2, 4, 5, 7, 8, 9], target = 6
/// Output: 4
/// Explanation: 6 would be inserted at index 4 to be positioned between 5 and 7:
/// [1, 2, 4, 5, 6, 7, 8, 9].
/// ```
pub fn find_insertion_index(nums: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = (right + left) / 2;
        if nums[mid] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

/// First and Last Occurrences of a Number
///
/// Given an array of integers sorted in non-decreasing order, return the first and last indexes
/// of a target number. If the target is not found, return [-1, -1].
///
/// # Example
///
/// ```text
/// Input: nums = [1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], target = 4
/// Output: [3, 5]
/// Explanation: The first and last occurrences of number 4 are indexes 3 and 5, respectively.
/// ```
pub fn find_first_and_last(nums: &[i32], target: i32) -> [i32; 2] {
    if nums.is_empty() {
        return [-1, -1];
    }

    let find_lower_bound = |nums: &[i32], target: i32| -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (right + left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid - 1;
            } else {
                right = mid;
            }
        }

        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    };
    let find_upper_bound = |nums: &[i32], target: i32| -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = ((right + left) / 2) + 1;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid - 1;
            } else {
                left = mid;
            }
        }

        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    };

    [
        find_lower_bound(nums, target),
        find_upper_bound(nums, target),
    ]
}

pub fn cutting_wood(heights: &[i32], k: i32) -> usize {
    let has_enough_wood = |heights: &[i32], target_height: i32, k: i32| -> bool {
        let mut wood_collected = 0;

        for &height in heights {
            if height > target_height {
                wood_collected += height - target_height;
            }
        }

        wood_collected >= k
    };

    let mut left = 0;
    let mut right = *heights.iter().max().unwrap() as usize;

    while left <= right {
        let mid = ((right + left) / 2) + 1;

        if has_enough_wood(heights, mid as i32, k) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

/// Find the Target in a Rotated Sorted Array
///
/// A rotated sorted array is an array of numbers sorted in ascending order, in which a portion
/// of the array is moved from the beginning to the end. For example, a possible rotation of
/// [1, 2, 3, 4, 5] is [3, 4, 5, 1, 2], where the first two numbers are moved to the end.
///
/// Given a rotated sorted array of unique numbers, return the index of a target value.
/// If the target value is not present, return -1.
///
/// # Example
///
/// ```text
/// Input: nums = [8, 9, 1, 2, 3, 4, 5, 6, 7], target = 1
/// Output: 2
/// ```
pub fn find_in_rotated_array(nums: &[i32], target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = (right + left) / 2;

        if nums[mid] == target {
            return mid as i32;
        } else if nums[left] <= nums[mid] {
            if nums[left] <= target && target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    if nums[left] == target {
        left as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_insertion_index_found() {
        assert_eq!(find_insertion_index(&[1, 2, 4, 5, 7, 8, 9], 4), 2);
    }

    #[test]
    fn test_find_insertion_index_not_found() {
        assert_eq!(find_insertion_index(&[1, 2, 4, 5, 7, 8, 9], 6), 4);
    }

    #[test]
    fn test_find_insertion_index_insert_at_start() {
        assert_eq!(find_insertion_index(&[2, 4, 6, 8], 1), 0);
    }

    #[test]
    fn test_find_insertion_index_insert_at_end() {
        assert_eq!(find_insertion_index(&[2, 4, 6, 8], 10), 4);
    }

    #[test]
    fn test_find_insertion_index_first_element() {
        assert_eq!(find_insertion_index(&[1, 3, 5, 7], 1), 0);
    }

    #[test]
    fn test_find_insertion_index_last_element() {
        assert_eq!(find_insertion_index(&[1, 3, 5, 7], 7), 3);
    }

    #[test]
    fn test_find_insertion_index_empty_array() {
        assert_eq!(find_insertion_index(&[], 5), 0);
    }

    #[test]
    fn test_find_insertion_index_single_element_found() {
        assert_eq!(find_insertion_index(&[5], 5), 0);
    }

    #[test]
    fn test_find_insertion_index_single_element_insert_before() {
        assert_eq!(find_insertion_index(&[5], 3), 0);
    }

    #[test]
    fn test_find_insertion_index_single_element_insert_after() {
        assert_eq!(find_insertion_index(&[5], 7), 1);
    }

    #[test]
    fn test_find_insertion_index_negative_numbers() {
        assert_eq!(find_insertion_index(&[-10, -5, 0, 5, 10], -3), 2);
    }

    #[test]
    fn test_find_insertion_index_two_elements() {
        assert_eq!(find_insertion_index(&[1, 3], 2), 1);
    }

    // find_first_and_last tests

    #[test]
    fn test_find_first_and_last_example() {
        assert_eq!(
            find_first_and_last(&[1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], 4),
            [3, 5]
        );
    }

    #[test]
    fn test_find_first_and_last_not_found() {
        assert_eq!(find_first_and_last(&[1, 2, 3, 5, 6], 4), [-1, -1]);
    }

    #[test]
    fn test_find_first_and_last_single_occurrence() {
        assert_eq!(find_first_and_last(&[1, 2, 3, 4, 5], 3), [2, 2]);
    }

    #[test]
    fn test_find_first_and_last_all_same() {
        assert_eq!(find_first_and_last(&[4, 4, 4, 4, 4], 4), [0, 4]);
    }

    #[test]
    fn test_find_first_and_last_at_start() {
        assert_eq!(find_first_and_last(&[1, 1, 1, 2, 3, 4], 1), [0, 2]);
    }

    #[test]
    fn test_find_first_and_last_at_end() {
        assert_eq!(find_first_and_last(&[1, 2, 3, 4, 4, 4], 4), [3, 5]);
    }

    #[test]
    fn test_find_first_and_last_empty_array() {
        assert_eq!(find_first_and_last(&[], 4), [-1, -1]);
    }

    #[test]
    fn test_find_first_and_last_single_element_found() {
        assert_eq!(find_first_and_last(&[4], 4), [0, 0]);
    }

    #[test]
    fn test_find_first_and_last_single_element_not_found() {
        assert_eq!(find_first_and_last(&[5], 4), [-1, -1]);
    }

    #[test]
    fn test_find_first_and_last_two_occurrences() {
        assert_eq!(find_first_and_last(&[1, 2, 2, 3], 2), [1, 2]);
    }

    // cutting_wood tests

    #[test]
    fn test_cutting_wood_example() {
        assert_eq!(cutting_wood(&[2, 6, 3, 8], 7), 3);
    }

    #[test]
    fn test_cutting_wood_exact_cut() {
        assert_eq!(cutting_wood(&[5, 5, 5, 5], 4), 4);
    }

    #[test]
    fn test_cutting_wood_all_same_height() {
        assert_eq!(cutting_wood(&[10, 10, 10], 15), 5);
    }

    #[test]
    fn test_cutting_wood_single_tree() {
        assert_eq!(cutting_wood(&[10], 5), 5);
    }

    #[test]
    fn test_cutting_wood_large_k() {
        assert_eq!(cutting_wood(&[20, 15, 10, 17], 24), 10);
    }

    // find_in_rotated_array tests

    #[test]
    fn test_find_in_rotated_array_example() {
        assert_eq!(find_in_rotated_array(&[8, 9, 1, 2, 3, 4, 5, 6, 7], 1), 2);
    }

    #[test]
    fn test_find_in_rotated_array_not_found() {
        assert_eq!(find_in_rotated_array(&[8, 9, 1, 2, 3, 4, 5, 6, 7], 10), -1);
    }

    #[test]
    fn test_find_in_rotated_array_first_element() {
        assert_eq!(find_in_rotated_array(&[4, 5, 6, 7, 0, 1, 2], 4), 0);
    }

    #[test]
    fn test_find_in_rotated_array_last_element() {
        assert_eq!(find_in_rotated_array(&[4, 5, 6, 7, 0, 1, 2], 2), 6);
    }

    #[test]
    fn test_find_in_rotated_array_pivot_element() {
        assert_eq!(find_in_rotated_array(&[4, 5, 6, 7, 0, 1, 2], 7), 3);
    }

    #[test]
    fn test_find_in_rotated_array_no_rotation() {
        assert_eq!(find_in_rotated_array(&[1, 2, 3, 4, 5], 3), 2);
    }

    #[test]
    fn test_find_in_rotated_array_single_element_found() {
        assert_eq!(find_in_rotated_array(&[5], 5), 0);
    }

    #[test]
    fn test_find_in_rotated_array_single_element_not_found() {
        assert_eq!(find_in_rotated_array(&[5], 3), -1);
    }

    #[test]
    fn test_find_in_rotated_array_two_elements() {
        assert_eq!(find_in_rotated_array(&[2, 1], 1), 1);
    }

    #[test]
    fn test_find_in_rotated_array_target_in_left_half() {
        assert_eq!(find_in_rotated_array(&[6, 7, 8, 1, 2, 3, 4, 5], 8), 2);
    }

    #[test]
    fn test_find_in_rotated_array_target_in_right_half() {
        assert_eq!(find_in_rotated_array(&[6, 7, 8, 1, 2, 3, 4, 5], 3), 5);
    }
}
