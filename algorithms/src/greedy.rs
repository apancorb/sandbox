/// Jump to the End
///
/// You are given an integer array in which you're originally positioned at index 0. Each number
/// in the array represents the maximum jump distance from the current index. Determine if
/// it's possible to reach the end of the array.
///
/// # Example 1
///
/// ```text
/// Input: nums = [3, 2, 0, 2, 5]
///
/// Output: true
/// ```
///
/// # Example 2
///
/// ```text
/// Input: nums = [2, 1, 0, 3]
///
/// Output: false
/// ```
///
/// # Constraints
///
/// - There is at least one element in nums.
/// - All integers in nums are non-negative integers.
pub fn jump_to_end(nums: &[usize]) -> bool {
    let mut destination = nums.len() - 1;

    for i in (0..nums.len() - 1).rev() {
        if nums[i] + i >= destination {
            destination = i;
        }
    }

    destination == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_to_end_example1() {
        assert_eq!(jump_to_end(&[3, 2, 0, 2, 5]), true);
    }

    #[test]
    fn test_jump_to_end_example2() {
        assert_eq!(jump_to_end(&[2, 1, 0, 3]), false);
    }

    #[test]
    fn test_jump_to_end_single() {
        assert_eq!(jump_to_end(&[0]), true);
    }

    #[test]
    fn test_jump_to_end_two_elements() {
        assert_eq!(jump_to_end(&[1, 0]), true);
        assert_eq!(jump_to_end(&[0, 1]), false);
    }

    #[test]
    fn test_jump_to_end_all_ones() {
        assert_eq!(jump_to_end(&[1, 1, 1, 1]), true);
    }

    #[test]
    fn test_jump_to_end_large_jump() {
        assert_eq!(jump_to_end(&[5, 0, 0, 0, 0]), true);
    }
}
