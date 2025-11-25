/// Pair Sum - Sorted
///
/// Given an array of integers sorted in ascending order and a target value, return the indexes
/// of any pair of numbers in the array that sum to the target. The order of the indexes in the
/// result doesn't matter. If no pair is found, return an empty array.
///
/// # Examples
///
/// Example 1:
/// ```
/// Input: nums = [-5, -2, 3, 4, 6], target = 7
/// Output: [2, 3]
/// Explanation: nums[2] + nums[3] = 3 + 4 = 7
/// ```
///
/// Example 2:
/// ```
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
/// ```
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
/// ```
/// Input: s = "a]dog I a panic in a pagoda..."
/// Output: true
/// ```
///
/// Example 2:
/// ```
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
}
