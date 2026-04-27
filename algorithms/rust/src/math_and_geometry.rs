/// Spiral Traversal
///
/// Return elements of a matrix in clockwise spiral order.
///
/// # Examples
///
/// ```text
/// Input:
///   [[ 0,  1,  2,  3,  4],
///    [ 5,  6,  7,  8,  9],
///    [10, 11, 12, 13, 14],
///    [15, 16, 17, 18, 19]]
///
/// Output: [0,1,2,3,4, 9,14,19, 18,17,16,15, 10,5, 6,7,8, 13, 12,11]
/// ```
///
/// Use four boundaries: top, bottom, left, right.
/// Each pass walks one edge and shrinks that boundary inward.
///
/// ```text
/// → → → → →     top row (left to right), then top++
///         ↓     right col (top to bottom), then right--
/// ← ← ← ← ←     bottom row (right to left), then bottom--
/// ↑               left col (bottom to top), then left++
/// ```
///
/// Repeat until boundaries cross.
///
/// # Complexity
///
/// - Time: O(m*n) — visit every cell
/// - Space: O(1) — besides output
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
/// Reverse digits. Return 0 if result overflows 32-bit signed range.
///
/// # Examples
///
/// ```text
/// Input: n = 420
/// Output: 24
/// ```
///
/// ```text
/// Input: n = -15
/// Output: -51
/// ```
///
/// Extract digits with % 10 and // 10, build reversed number.
/// Python ints are unbounded, so check overflow at the end. In Rust we
/// use checked arithmetic to detect overflow during construction.
///
/// Example walkthrough for 420:
///
/// ```text
/// 420 % 10 = 0, 420 // 10 = 42 → reversed = 0
/// 42 % 10 = 2,  42 // 10 = 4   → reversed = 2
/// 4 % 10 = 4,   4 // 10 = 0    → reversed = 24
/// ```
///
/// # Complexity
///
/// - Time: O(log n) — process each digit
/// - Space: O(1)
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
/// Find the maximum number of points that lie on the same straight line.
///
/// # Examples
///
/// ```text
/// Input: points = [[1,1],[1,3],[2,2],[3,1],[3,3],[4,4]]
/// Output: 4  (the diagonal: (1,1),(2,2),(3,3),(4,4))
/// ```
///
/// For each "focal" point, calculate slope to every other point.
/// Points with the same slope from the focal point are collinear.
///
/// Slope trick: use reduced fraction (rise/gcd, run/gcd) instead of
/// float division to avoid precision errors.
/// Normalize sign so (-1,-2) and (1,2) are treated the same.
/// Vertical lines: use (1, 0) as special marker.
///
/// Example walkthrough — focal point (1,1), slopes to others:
///
/// ```text
/// (1,3): rise=2, run=0 → vertical (1,0)
/// (2,2): rise=1, run=1 → slope (1,1)
/// (3,1): rise=0, run=2 → slope (0,1)
/// (3,3): rise=2, run=2 → slope (1,1)  ← same as (2,2)!
/// (4,4): rise=3, run=3 → slope (1,1)  ← same again!
/// Slope (1,1) has count 3, so 3+1 = 4 collinear points.
/// ```
///
/// # Complexity
///
/// - Time: O(n^2)
/// - Space: O(n) — slope map per focal point
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
/// Convert Roman numeral string to integer.
/// I=1, V=5, X=10, L=50, C=100, D=500, M=1000
///
/// # Examples
///
/// ```text
/// Input: "MCMXCIV"
/// Output: 1994
/// (M=1000, CM=900, XC=90, IV=4)
/// ```
///
/// Rule: if current value < next value, it's a subtraction pair.
/// IV → 5-1=4,  IX → 10-1=9,  XL → 50-10=40, etc.
/// Otherwise just add.
///
/// Example walkthrough for "MCMXCIV":
///
/// ```text
/// M(1000) < C(100)? no  → +1000, total=1000
/// C(100) < M(1000)? yes → -100,  total=900
/// M(1000) < X(10)? no   → +1000, total=1900
/// X(10) < C(100)? yes   → -10,   total=1890
/// C(100) < I(1)? no     → +100,  total=1990
/// I(1) < V(5)? yes      → -1,    total=1989
/// V(5) is last           → +5,    total=1994
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
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
/// Convert integer to Roman numeral string.
///
/// # Examples
///
/// ```text
/// Input: 1994
/// Output: "MCMXCIV"
/// (1000=M, 900=CM, 90=XC, 4=IV)
/// ```
///
/// Greedy: include subtractive forms (CM, CD, XC, XL, IX, IV)
/// in the lookup table. Walk through largest to smallest,
/// subtracting and appending.
///
/// Example walkthrough for 1994:
///
/// ```text
/// 1994 >= 1000 → "M",    remaining=994
/// 994 >= 900   → "CM",   remaining=94
/// 94 >= 90     → "XC",   remaining=4
/// 4 >= 4       → "IV",   remaining=0
/// Result: "MCMXCIV"
/// ```
///
/// # Complexity
///
/// - Time: O(1) — bounded by 3999
/// - Space: O(1)
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

/// Length of Last Word
///
/// Return the length of the last word in a string.
///
/// # Examples
///
/// ```text
/// Input: "   fly me   to   the moon  "
/// Output: 4
/// ```
///
/// Split the string on whitespace, which automatically handles leading,
/// trailing, and multiple spaces. The last element of the split result
/// is the last word, so just return its length.
///
/// Example walkthrough for "   fly me   to   the moon  ":
///
/// ```text
/// split() → ["fly", "me", "to", "the", "moon"]
/// last word → "moon"
/// len("moon") → 4
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
pub fn length_of_last_word(s: &str) -> usize {
    s.split_whitespace().last().map_or(0, |w| w.len())
}

/// Longest Common Prefix
///
/// Find the longest common prefix among an array of strings.
///
/// # Examples
///
/// ```text
/// Input: ["flower", "flow", "flight"]
/// Output: "fl"
/// ```
///
/// Compare char by char using the first string as reference.
/// Stop when any string differs or runs out of characters.
///
/// Example walkthrough for ["flower", "flow", "flight"]:
///
/// ```text
/// i=0: 'f' == 'f' == 'f' → match
/// i=1: 'l' == 'l' == 'l' → match
/// i=2: 'o' == 'o' != 'i' → stop, return "fl"
/// ```
///
/// # Complexity
///
/// - Time: O(n * m) — n strings, m = shortest length
/// - Space: O(1)
pub fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first = strs[0];

    for (i, c) in first.chars().enumerate() {
        for s in &strs[1..] {
            if i >= s.len() || s.chars().nth(i) != Some(c) {
                return first[..i].to_string();
            }
        }
    }

    first.to_string()
}

/// Reverse Words in a String
///
/// Reverse word order. Strip extra spaces.
///
/// # Examples
///
/// ```text
/// Input: "  hello world  "
/// Output: "world hello"
/// ```
///
/// Split the string on whitespace to get a list of words, then reverse
/// the list and join with single spaces. Python's split() handles all
/// the edge cases with multiple and trailing spaces automatically.
///
/// Example walkthrough for "  hello world  ":
///
/// ```text
/// split() → ["hello", "world"]
/// [::-1]  → ["world", "hello"]
/// join    → "world hello"
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(n)
pub fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

/// Zigzag Conversion
///
/// Write string in zigzag pattern across num_rows, then read line by line.
///
/// # Examples
///
/// ```text
/// Input: s = "PAYPALISHIRING", num_rows = 3
///
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
///
/// Output: "PAHNAPLSIIGYIR"
/// ```
///
/// Simulate: bounce a row index up and down.
/// Row goes 0,1,2,1,0,1,2,1,0... for num_rows=3.
/// Collect chars into each row, then concatenate.
///
/// Example walkthrough for "PAYPALISHIRING" with num_rows=3:
///
/// ```text
/// P → row 0 (down)    row 0: [P]
/// A → row 1 (down)    row 1: [A]
/// Y → row 2 (bounce)  row 2: [Y]
/// P → row 1 (up)      row 1: [A,P]
/// A → row 0 (bounce)  row 0: [P,A]
/// L → row 1 (down)    row 1: [A,P,L]
/// ...continues...
/// Result: "PAHN" + "APLSIIG" + "YIR" = "PAHNAPLSIIGYIR"
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(n)
pub fn zigzag_convert(s: &str, num_rows: usize) -> String {
    if num_rows == 1 || num_rows >= s.len() {
        return s.to_string();
    }

    // Create a row for each level of the zigzag
    let mut rows: Vec<String> = vec![String::new(); num_rows];
    let mut current_row = 0;
    let mut going_down = true;

    for c in s.chars() {
        rows[current_row].push(c);

        // At top or bottom, reverse direction
        if current_row == 0 {
            going_down = true;
        } else if current_row == num_rows - 1 {
            going_down = false;
        }

        if going_down {
            current_row += 1;
        } else {
            current_row -= 1;
        }
    }

    // Concatenate all rows
    rows.concat()
}

/// Find First Occurrence in String
///
/// Return index of first occurrence of needle in haystack, or -1.
///
/// # Examples
///
/// ```text
/// Input: haystack = "sadbutsad", needle = "sad"
/// Output: 0
/// ```
///
/// ```text
/// Input: haystack = "leetcode", needle = "leeto"
/// Output: -1
/// ```
///
/// Use the built-in string find method, which slides a window of
/// the needle's length across the haystack checking for a match. This
/// is the simplest approach and handles all edge cases cleanly.
///
/// Example walkthrough:
///
/// ```text
/// "sadbutsad", needle="sad":
///     i=0: "sad" == "sad"? yes → return 0
///
/// "leetcode", needle="leeto":
///     i=0: "leetc" == "leeto"? no
///     i=1: "eetco" == "leeto"? no
///     ...no match found → return -1
/// ```
///
/// # Complexity
///
/// - Time: O(n * m) worst case
/// - Space: O(1)
pub fn str_str(haystack: &str, needle: &str) -> i32 {
    haystack.find(needle).map_or(-1, |i| i as i32)
}

// =============================================================================
// Number Theory
// =============================================================================

/// Happy Number
///
/// A happy number eventually reaches 1 when you repeatedly sum the
/// squares of its digits. An unhappy number loops forever.
///
/// # Examples
///
/// ```text
/// Input: n = 23
/// Output: true
/// 2² + 3² = 13 → 1² + 3² = 10 → 1² + 0² = 1 ✓
/// ```
///
/// ```text
/// Input: n = 4
/// Output: false
/// 4 → 16 → 37 → 58 → 89 → 145 → 42 → 20 → 4 (cycle!)
/// ```
///
/// Key insight: if a number isn't happy, the sequence CYCLES.
/// This is just cycle detection! Use Floyd's algorithm:
/// - slow computes one step
/// - fast computes two steps
/// - If fast hits 1 → happy
/// - If slow == fast → cycle → unhappy
///
/// digit_square_sum examples:
///
/// ```text
/// 123 → 1² + 2² + 3² = 1 + 4 + 9 = 14
/// Get digits with % 10 and // 10:
///     123 % 10 = 3, 123 // 10 = 12
///     12 % 10 = 2,  12 // 10 = 1
///     1 % 10 = 1,   1 // 10 = 0 → done
/// ```
///
/// # Complexity
///
/// - Time: O(log n) per step, O(log n) steps
/// - Space: O(1)
pub fn is_happy_number(num: u32) -> bool {
    let get_next_num = |mut curr_num: u32| -> u32 {
        let mut next_num = 0;
        while curr_num > 0 {
            let digit = curr_num % 10;
            curr_num = curr_num / 10;
            next_num += digit * digit;
        }
        next_num
    };

    let mut slow = num;
    let mut fast = num;

    loop {
        slow = get_next_num(slow);
        fast = get_next_num(get_next_num(fast));
        if fast == 1 {
            return true;
        } else if slow == fast {
            return false;
        }
    }
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

    #[test]
    fn test_length_of_last_word_example1() {
        assert_eq!(length_of_last_word("Hello World"), 5);
    }

    #[test]
    fn test_length_of_last_word_example2() {
        assert_eq!(length_of_last_word("   fly me   to   the moon  "), 4);
    }

    #[test]
    fn test_length_of_last_word_example3() {
        assert_eq!(length_of_last_word("luffy is still joyboy"), 6);
    }

    #[test]
    fn test_length_of_last_word_single() {
        assert_eq!(length_of_last_word("a"), 1);
    }

    #[test]
    fn test_length_of_last_word_trailing_spaces() {
        assert_eq!(length_of_last_word("hello   "), 5);
    }

    #[test]
    fn test_longest_common_prefix_example1() {
        assert_eq!(longest_common_prefix(&["flower", "flow", "flight"]), "fl");
    }

    #[test]
    fn test_longest_common_prefix_example2() {
        assert_eq!(longest_common_prefix(&["dog", "racecar", "car"]), "");
    }

    #[test]
    fn test_longest_common_prefix_single() {
        assert_eq!(longest_common_prefix(&["alone"]), "alone");
    }

    #[test]
    fn test_longest_common_prefix_identical() {
        assert_eq!(longest_common_prefix(&["test", "test", "test"]), "test");
    }

    #[test]
    fn test_longest_common_prefix_empty_string() {
        assert_eq!(longest_common_prefix(&["", "b"]), "");
    }

    #[test]
    fn test_reverse_words_example1() {
        assert_eq!(reverse_words("the sky is blue"), "blue is sky the");
    }

    #[test]
    fn test_reverse_words_example2() {
        assert_eq!(reverse_words("  hello world  "), "world hello");
    }

    #[test]
    fn test_reverse_words_multiple_spaces() {
        assert_eq!(reverse_words("a   good   example"), "example good a");
    }

    #[test]
    fn test_reverse_words_single() {
        assert_eq!(reverse_words("word"), "word");
    }

    #[test]
    fn test_reverse_words_empty() {
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn test_zigzag_convert_example1() {
        assert_eq!(zigzag_convert("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn test_zigzag_convert_example2() {
        assert_eq!(zigzag_convert("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
    }

    #[test]
    fn test_zigzag_convert_single_row() {
        assert_eq!(zigzag_convert("A", 1), "A");
    }

    #[test]
    fn test_zigzag_convert_two_rows() {
        assert_eq!(zigzag_convert("ABCD", 2), "ACBD");
    }

    #[test]
    fn test_zigzag_convert_more_rows_than_chars() {
        assert_eq!(zigzag_convert("AB", 5), "AB");
    }

    #[test]
    fn test_str_str_example1() {
        assert_eq!(str_str("sadbutsad", "sad"), 0);
    }

    #[test]
    fn test_str_str_example2() {
        assert_eq!(str_str("leetcode", "leeto"), -1);
    }

    #[test]
    fn test_str_str_middle() {
        assert_eq!(str_str("hello", "ll"), 2);
    }

    #[test]
    fn test_str_str_empty_needle() {
        assert_eq!(str_str("hello", ""), 0);
    }

    #[test]
    fn test_str_str_full_match() {
        assert_eq!(str_str("abc", "abc"), 0);
    }

    // -------------------------------------------------------------------------
    // Number Theory tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_is_happy_number_23() {
        assert!(is_happy_number(23));
    }

    #[test]
    fn test_is_happy_number_1() {
        assert!(is_happy_number(1));
    }

    #[test]
    fn test_is_happy_number_7() {
        // 7 -> 49 -> 97 -> 130 -> 10 -> 1
        assert!(is_happy_number(7));
    }

    #[test]
    fn test_is_happy_number_19() {
        // 19 -> 82 -> 68 -> 100 -> 1
        assert!(is_happy_number(19));
    }

    #[test]
    fn test_is_unhappy_number_2() {
        assert!(!is_happy_number(2));
    }

    #[test]
    fn test_is_unhappy_number_4() {
        // 4 -> 16 -> 37 -> 58 -> 89 -> 145 -> 42 -> 20 -> 4 (cycle)
        assert!(!is_happy_number(4));
    }

    #[test]
    fn test_is_unhappy_number_20() {
        assert!(!is_happy_number(20));
    }

    #[test]
    fn test_is_happy_number_100() {
        // 100 -> 1
        assert!(is_happy_number(100));
    }
}
