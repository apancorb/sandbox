/// Spiral Traversal
///
/// Return the elements of a matrix in clockwise spiral order.
///
/// # Example
///
/// ```text
/// Input:
///   [[ 0,  1,  2,  3,  4],
///    [ 5,  6,  7,  8,  9],
///    [10, 11, 12, 13, 14],
///    [15, 16, 17, 18, 19]]
///
/// Output: [0, 1, 2, 3, 4, 9, 14, 19, 18, 17, 16, 15, 10, 5, 6, 7, 8, 13, 12, 11]
/// ```
pub fn spiral_traversal(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();

    let mut top = 0;
    let mut bottom = matrix.len() - 1;
    let mut left = 0;
    let mut right = matrix[0].len() - 1;

    while top <= bottom && left <= right {
        // Move from left to right along the top boundry
        for i in left..=right {
            result.push(matrix[top][i]);
        }
        top += 1;

        // Move from top to bottom along the right boundry
        for i in top..=bottom {
            result.push(matrix[i][right]);
        }
        right -= 1;

        // Check that the bottom boundary hasn't passed the top boundary
        // before moving from right to left along the bottom boundary.
        if top <= bottom {
            for i in (left..=right).rev() {
                result.push(matrix[bottom][i]);
            }
            bottom -= 1;
        }

        // Check that the left boundary hasn't passed the right boundary
        // before moving from bottom to top along the left boundary.
        if left <= right {
            for i in (top..=bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }
    }

    result
}

/// Reverse 32-Bit Integer
///
/// Reverse the digits of a signed 32-bit integer. If the reversed integer overflows
/// (i.e., is outside the range [-2^31, 2^31 - 1]), return 0. Assume the environment
/// only allows you to store integers within the signed 32-bit integer range.
///
/// # Example 1
///
/// ```text
/// Input: n = 420
/// Output: 24
/// ```
///
/// # Example 2
///
/// ```text
/// Input: n = -15
/// Output: -51
/// ```
// Time: O(log n) - we process each digit once, and n has log₁₀(n) digits.
//       For 32-bit integers (max 10 digits), this is effectively O(1).
// Space: O(1) - only a few variables, no extra data structures.
pub fn reverse_integer(mut n: i32) -> i32 {
    let mut reversed_n: i32 = 0;

    while n != 0 {
        let digit = n % 10;
        n /= 10;

        reversed_n = match reversed_n
            .checked_mul(10)
            .and_then(|r| r.checked_add(digit))
        {
            Some(r) => r,
            None => return 0,
        };
    }

    reversed_n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_traversal_example() {
        let matrix = vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
        ];
        assert_eq!(
            spiral_traversal(&matrix),
            vec![
                0, 1, 2, 3, 4, 9, 14, 19, 18, 17, 16, 15, 10, 5, 6, 7, 8, 13, 12, 11
            ]
        );
    }

    #[test]
    fn test_spiral_traversal_single_row() {
        let matrix = vec![vec![1, 2, 3, 4]];
        assert_eq!(spiral_traversal(&matrix), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_spiral_traversal_square() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_traversal(&matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_spiral_traversal_empty() {
        let matrix: Vec<Vec<i32>> = vec![];
        assert_eq!(spiral_traversal(&matrix), vec![]);
    }

    #[test]
    fn test_reverse_integer_example1() {
        assert_eq!(reverse_integer(420), 24);
    }

    #[test]
    fn test_reverse_integer_example2() {
        assert_eq!(reverse_integer(-15), -51);
    }

    #[test]
    fn test_reverse_integer_zero() {
        assert_eq!(reverse_integer(0), 0);
    }

    #[test]
    fn test_reverse_integer_with_trailing_zeros() {
        assert_eq!(reverse_integer(1200), 21);
    }

    #[test]
    fn test_reverse_integer_overflow_positive() {
        // 2147483647 reversed would overflow
        assert_eq!(reverse_integer(1534236469), 0);
    }

    #[test]
    fn test_reverse_integer_overflow_negative() {
        // -2147483648 reversed would overflow
        assert_eq!(reverse_integer(-1563847412), 0);
    }
}
