/// Sum Between Range
///
/// Given an integer array, write a function which returns the sum of values between two
/// indexes.
///
/// # Example
///
/// ```text
/// nums = [3, -7, 6, 0, -2, 5]
///
/// sum_range(0, 3) = 3 + (-7) + 6 + 0 = 2
/// sum_range(2, 4) = 6 + 0 + (-2) = 4
/// sum_range(2, 2) = 6
///
/// Input: nums = [3, -7, 6, 0, -2, 5], [sum_range(0, 3), sum_range(2, 4), sum_range(2, 2)]
/// Output: [2, 4, 6]
/// ```
///
/// # Constraints
///
/// - nums contains at least one element
/// - Each sum_range operation will query a valid range of the input array
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
