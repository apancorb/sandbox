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
    todo!("Implement find_insertion_index")
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
}
