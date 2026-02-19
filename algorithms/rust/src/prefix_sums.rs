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

/// K-Sum Subarrays
///
/// Find the number of subarrays in an integer array that sum to k.
///
/// # Example
///
/// ```text
/// Input: nums = [1, 2, -1, 1, 2], k = 3
/// Input: nums = [0, 1, 2, -1, 1, 2], k = 3
/// Output: 3
/// Explanation: The subarrays that sum to 3 are:
///   [1, 2] (indices 0-1)
///   [1, 2, -1, 1] (indices 0-3)
///   [1, 2] (indices 3-4)
/// ```
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
/// Given an array of integers, return an array res so that res[i] is equal to the product of all
/// the elements of the input array except nums[i] itself.
///
/// # Example
///
/// ```text
/// Input: nums = [2, 3, 1, 4, 5]
/// Output: [60, 40, 120, 30, 24]
/// Explanation: The output value at index 0 is the product of all numbers except nums[0]
/// (3 * 1 * 4 * 5 = 60). The same logic applies to the rest of the output.
/// ```
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
