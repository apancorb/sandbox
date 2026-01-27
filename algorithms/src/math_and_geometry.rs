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

/// Maximum Collinear Points
///
/// Given a set of points in a two-dimensional plane, determine the maximum number
/// of points that lie along the same straight line.
///
/// # Example
///
/// ```text
///   Y
/// 4 |       •
/// 3 | •   •
/// 2 |   •
/// 1 | •   •
///   +----------→ X
///     1 2 3 4
///
/// Input: points = [[1, 1], [1, 3], [2, 2], [3, 1], [3, 3], [4, 4]]
/// Output: 4
/// ```
///
/// # Constraints
///
/// - The input won't contain duplicate points.
pub fn max_collinear_points(points: &[[i32; 2]]) -> usize {
    let mut res = 0;

    // Try each point as the "focal point"
    for i in 0..points.len() {
        res = res.max(max_points_from_focal_point(i, points));
    }

    res
}

fn max_points_from_focal_point(focal_idx: usize, points: &[[i32; 2]]) -> usize {
    use std::collections::HashMap;

    let mut slopes_map: HashMap<(i32, i32), usize> = HashMap::new();
    let mut max_points = 0;

    // Calculate slope from focal point to every other point
    for j in 0..points.len() {
        if j != focal_idx {
            let slope = get_slope(&points[focal_idx], &points[j]);

            // Count how many points share this slope
            let count = slopes_map.entry(slope).or_insert(0);
            *count += 1;

            max_points = max_points.max(*count);
        }
    }

    // +1 to include the focal point itself
    max_points + 1
}

fn get_slope(p1: &[i32; 2], p2: &[i32; 2]) -> (i32, i32) {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }

    let rise = p2[1] - p1[1];
    let run = p2[0] - p1[0];

    // Vertical line: can't divide by 0, use special marker
    if run == 0 {
        return (1, 0);
    }

    // Reduce fraction so (2,4) and (1,2) become the same
    let g = gcd(rise, run);
    let mut reduced = (rise / g, run / g);

    // Normalize sign: keep run positive so (-1,-2) and (1,2) match
    if reduced.1 < 0 {
        reduced = (-reduced.0, -reduced.1);
    }

    reduced
}

/// Roman to Integer
///
/// Convert a Roman numeral string to an integer.
///
/// Symbol values: I=1, V=5, X=10, L=50, C=100, D=500, M=1000
///
/// Subtraction rules:
/// - I before V (5) or X (10) makes 4 or 9
/// - X before L (50) or C (100) makes 40 or 90
/// - C before D (500) or M (1000) makes 400 or 900
///
/// # Example 1
///
/// ```text
/// Input: "III"
/// Output: 3
/// ```
///
/// # Example 2
///
/// ```text
/// Input: "MCMXCIV"
/// Output: 1994
/// Explanation: M=1000, CM=900, XC=90, IV=4
/// ```
pub fn roman_to_int(s: &str) -> i32 {
    fn value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    let chars: Vec<char> = s.chars().collect();
    let mut total = 0;
    let mut i = 0;

    while i < chars.len() {
        let curr = value(chars[i]);
        let next = if i + 1 < chars.len() { value(chars[i + 1]) } else { 0 };

        if curr < next {
            total += next - curr;
            i += 2;
        } else {
            total += curr;
            i += 1;
        }
    }

    total
}

/// Integer to Roman
///
/// Convert an integer to a Roman numeral string.
///
/// # Example 1
///
/// ```text
/// Input: 3749
/// Output: "MMMDCCXLIX"
/// ```
///
/// # Example 2
///
/// ```text
/// Input: 1994
/// Output: "MCMXCIV"
/// Explanation: M=1000, CM=900, XC=90, IV=4
/// ```
//
// Greedy approach: include subtractive forms in lookup table.
// Walk through with 1994:
//
//   remaining = 1994, result = ""
//
//   1000: 1994 >= 1000? YES → result = "M", remaining = 994
//   900:  994 >= 900?   YES → result = "MCM", remaining = 94
//   90:   94 >= 90?     YES → result = "MCMXC", remaining = 4
//   4:    4 >= 4?       YES → result = "MCMXCIV", remaining = 0
//
//   Result: "MCMXCIV"
//
pub fn int_to_roman(num: i32) -> String {
    let symbols = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();
    let mut remaining = num;

    for (value, symbol) in symbols {
        while remaining >= value {
            result.push_str(symbol);
            remaining -= value;
        }
    }

    result
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

    #[test]
    fn test_max_collinear_points_example() {
        let points = [[1, 1], [1, 3], [2, 2], [3, 1], [3, 3], [4, 4]];
        assert_eq!(max_collinear_points(&points), 4);
    }

    #[test]
    fn test_max_collinear_points_single() {
        let points = [[0, 0]];
        assert_eq!(max_collinear_points(&points), 1);
    }

    #[test]
    fn test_max_collinear_points_two() {
        let points = [[0, 0], [1, 1]];
        assert_eq!(max_collinear_points(&points), 2);
    }

    #[test]
    fn test_max_collinear_points_horizontal() {
        let points = [[1, 1], [2, 1], [3, 1], [4, 1]];
        assert_eq!(max_collinear_points(&points), 4);
    }

    #[test]
    fn test_max_collinear_points_vertical() {
        let points = [[1, 1], [1, 2], [1, 3], [1, 4]];
        assert_eq!(max_collinear_points(&points), 4);
    }

    #[test]
    fn test_max_collinear_points_no_three_collinear() {
        // Triangle - no 3 points on same line
        let points = [[0, 0], [1, 1], [2, 0]];
        assert_eq!(max_collinear_points(&points), 2);
    }

    #[test]
    fn test_roman_to_int_example1() {
        assert_eq!(roman_to_int("III"), 3);
    }

    #[test]
    fn test_roman_to_int_example2() {
        assert_eq!(roman_to_int("LVIII"), 58);
    }

    #[test]
    fn test_roman_to_int_example3() {
        assert_eq!(roman_to_int("MCMXCIV"), 1994);
    }

    #[test]
    fn test_roman_to_int_subtraction_iv() {
        assert_eq!(roman_to_int("IV"), 4);
    }

    #[test]
    fn test_roman_to_int_subtraction_ix() {
        assert_eq!(roman_to_int("IX"), 9);
    }

    #[test]
    fn test_roman_to_int_single() {
        assert_eq!(roman_to_int("M"), 1000);
    }

    #[test]
    fn test_int_to_roman_example1() {
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
    }

    #[test]
    fn test_int_to_roman_example2() {
        assert_eq!(int_to_roman(58), "LVIII");
    }

    #[test]
    fn test_int_to_roman_example3() {
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }

    #[test]
    fn test_int_to_roman_subtractive() {
        assert_eq!(int_to_roman(4), "IV");
        assert_eq!(int_to_roman(9), "IX");
        assert_eq!(int_to_roman(40), "XL");
        assert_eq!(int_to_roman(90), "XC");
        assert_eq!(int_to_roman(400), "CD");
        assert_eq!(int_to_roman(900), "CM");
    }

    #[test]
    fn test_int_to_roman_single() {
        assert_eq!(int_to_roman(1), "I");
        assert_eq!(int_to_roman(1000), "M");
    }

    #[test]
    fn test_int_to_roman_max() {
        assert_eq!(int_to_roman(3999), "MMMCMXCIX");
    }
}
