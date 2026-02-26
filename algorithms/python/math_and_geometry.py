"""
Math and Geometry Pattern

A collection of math, geometry, and string manipulation problems.

Techniques:
- Spiral traversal: shrinking boundaries (top, bottom, left, right)
- Digit manipulation: % 10 gets last digit, // 10 removes it
- Slope comparison: use reduced fractions (GCD) to avoid float errors
- Roman numerals: greedy with lookup table including subtractive forms
"""

from math import gcd


def spiral_traversal(matrix: list[list[int]]) -> list[int]:
    """
    Spiral Traversal

    Return elements of a matrix in clockwise spiral order.

    Example:
        [[ 0,  1,  2,  3,  4],
         [ 5,  6,  7,  8,  9],
         [10, 11, 12, 13, 14],
         [15, 16, 17, 18, 19]]

        → [0,1,2,3,4, 9,14,19, 18,17,16,15, 10,5, 6,7,8, 13, 12,11]

    Time Complexity: O(m*n) - visit every cell
    Space Complexity: O(1) - besides output

    Use four boundaries: top, bottom, left, right.
    Each pass walks one edge and shrinks that boundary inward.

        → → → → →     top row (left to right), then top++
                ↓     right col (top to bottom), then right--
        ← ← ← ← ←     bottom row (right to left), then bottom--
        ↑               left col (bottom to top), then left++

    Repeat until boundaries cross.
    """
    if not matrix:
        return []

    result = []
    top, bottom = 0, len(matrix) - 1
    left, right = 0, len(matrix[0]) - 1

    while top <= bottom and left <= right:
        # → right along top
        for i in range(left, right + 1):
            result.append(matrix[top][i])
        top += 1

        # ↓ down along right
        for i in range(top, bottom + 1):
            result.append(matrix[i][right])
        right -= 1

        # ← left along bottom (if rows remain)
        if top <= bottom:
            for i in range(right, left - 1, -1):
                result.append(matrix[bottom][i])
            bottom -= 1

        # ↑ up along left (if cols remain)
        if left <= right:
            for i in range(bottom, top - 1, -1):
                result.append(matrix[i][left])
            left += 1

    return result


# -----------------------------------------------------------------------------
# Tests for spiral_traversal
# -----------------------------------------------------------------------------

def test_spiral_traversal_example():
    matrix = [[0,1,2,3,4],[5,6,7,8,9],[10,11,12,13,14],[15,16,17,18,19]]
    assert spiral_traversal(matrix) == [0,1,2,3,4,9,14,19,18,17,16,15,10,5,6,7,8,13,12,11]


def test_spiral_traversal_single_row():
    assert spiral_traversal([[1,2,3,4]]) == [1,2,3,4]


def test_spiral_traversal_square():
    assert spiral_traversal([[1,2,3],[4,5,6],[7,8,9]]) == [1,2,3,6,9,8,7,4,5]


def test_spiral_traversal_empty():
    assert spiral_traversal([]) == []


def reverse_integer(n: int) -> int:
    """
    Reverse 32-Bit Integer

    Reverse digits. Return 0 if result overflows 32-bit signed range.

    Example:
        >>> reverse_integer(420)
        24
        >>> reverse_integer(-15)
        -51

    Time Complexity: O(log n) - process each digit
    Space Complexity: O(1)

    Extract digits with % 10 and // 10, build reversed number.

    Example 420:
        420 % 10 = 0, 420 // 10 = 42 → reversed = 0
        42 % 10 = 2,  42 // 10 = 4   → reversed = 2
        4 % 10 = 4,   4 // 10 = 0    → reversed = 24

    Python ints are unbounded, so check overflow at the end.
    """
    INT_MIN, INT_MAX = -(2**31), 2**31 - 1

    sign = -1 if n < 0 else 1
    n = abs(n)

    reversed_n = 0
    while n > 0:
        digit = n % 10
        n //= 10
        reversed_n = reversed_n * 10 + digit

    reversed_n *= sign

    if reversed_n < INT_MIN or reversed_n > INT_MAX:
        return 0

    return reversed_n


# -----------------------------------------------------------------------------
# Tests for reverse_integer
# -----------------------------------------------------------------------------

def test_reverse_integer_example1():
    assert reverse_integer(420) == 24


def test_reverse_integer_example2():
    assert reverse_integer(-15) == -51


def test_reverse_integer_zero():
    assert reverse_integer(0) == 0


def test_reverse_integer_trailing_zeros():
    assert reverse_integer(1200) == 21


def test_reverse_integer_overflow_positive():
    assert reverse_integer(1534236469) == 0


def test_reverse_integer_overflow_negative():
    assert reverse_integer(-1563847412) == 0


def max_collinear_points(points: list[list[int]]) -> int:
    """
    Maximum Collinear Points

    Find the maximum number of points that lie on the same straight line.

    Example:
        points = [[1,1],[1,3],[2,2],[3,1],[3,3],[4,4]]
        → 4  (the diagonal: (1,1),(2,2),(3,3),(4,4))

    Time Complexity: O(n^2)
    Space Complexity: O(n) - slope map per focal point

    For each "focal" point, calculate slope to every other point.
    Points with the same slope from the focal point are collinear.

    Slope trick: use reduced fraction (rise/gcd, run/gcd) instead of
    float division to avoid precision errors.
    Normalize sign so (-1,-2) and (1,2) are treated the same.
    Vertical lines: use (1, 0) as special marker.
    """
    n = len(points)
    if n <= 2:
        return n

    best = 0

    for i in range(n):
        slopes = {}
        for j in range(n):
            if i == j:
                continue

            rise = points[j][1] - points[i][1]
            run = points[j][0] - points[i][0]

            if run == 0:
                slope = (1, 0)  # vertical line
            else:
                g = gcd(abs(rise), abs(run))
                rise, run = rise // g, run // g
                # Normalize: keep run positive
                if run < 0:
                    rise, run = -rise, -run
                slope = (rise, run)

            slopes[slope] = slopes.get(slope, 0) + 1
            best = max(best, slopes[slope])

    return best + 1  # +1 for the focal point itself


# -----------------------------------------------------------------------------
# Tests for max_collinear_points
# -----------------------------------------------------------------------------

def test_max_collinear_points_example():
    assert max_collinear_points([[1,1],[1,3],[2,2],[3,1],[3,3],[4,4]]) == 4


def test_max_collinear_points_single():
    assert max_collinear_points([[0,0]]) == 1


def test_max_collinear_points_two():
    assert max_collinear_points([[0,0],[1,1]]) == 2


def test_max_collinear_points_horizontal():
    assert max_collinear_points([[1,1],[2,1],[3,1],[4,1]]) == 4


def test_max_collinear_points_vertical():
    assert max_collinear_points([[1,1],[1,2],[1,3],[1,4]]) == 4


def test_max_collinear_points_no_three():
    assert max_collinear_points([[0,0],[1,1],[2,0]]) == 2


def roman_to_int(s: str) -> int:
    """
    Roman to Integer

    Convert Roman numeral string to integer.
    I=1, V=5, X=10, L=50, C=100, D=500, M=1000

    Example:
        >>> roman_to_int("MCMXCIV")
        1994
        # M=1000, CM=900, XC=90, IV=4

    Time Complexity: O(n)
    Space Complexity: O(1)

    Rule: if current value < next value, it's a subtraction pair.
        IV → 5-1=4,  IX → 10-1=9,  XL → 50-10=40, etc.
    Otherwise just add.
    """
    values = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}
    total = 0

    for i in range(len(s)):
        curr = values[s[i]]
        next_val = values[s[i + 1]] if i + 1 < len(s) else 0

        if curr < next_val:
            total -= curr  # subtraction: e.g. I before V
        else:
            total += curr

    return total


# -----------------------------------------------------------------------------
# Tests for roman_to_int
# -----------------------------------------------------------------------------

def test_roman_to_int_example1():
    assert roman_to_int("III") == 3


def test_roman_to_int_example2():
    assert roman_to_int("LVIII") == 58


def test_roman_to_int_example3():
    assert roman_to_int("MCMXCIV") == 1994


def test_roman_to_int_iv():
    assert roman_to_int("IV") == 4


def test_roman_to_int_ix():
    assert roman_to_int("IX") == 9


def test_roman_to_int_single():
    assert roman_to_int("M") == 1000


def int_to_roman(num: int) -> str:
    """
    Integer to Roman

    Convert integer to Roman numeral string.

    Example:
        >>> int_to_roman(1994)
        'MCMXCIV'
        # 1000=M, 900=CM, 90=XC, 4=IV

    Time Complexity: O(1) - bounded by 3999
    Space Complexity: O(1)

    Greedy: include subtractive forms (CM, CD, XC, XL, IX, IV)
    in the lookup table. Walk through largest to smallest,
    subtracting and appending.

    Example 1994:
        1994 >= 1000 → "M",    remaining=994
        994 >= 900   → "CM",   remaining=94
        94 >= 90     → "XC",   remaining=4
        4 >= 4       → "IV",   remaining=0
        Result: "MCMXCIV"
    """
    symbols = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"),  (90, "XC"),  (50, "L"),  (40, "XL"),
        (10, "X"),   (9, "IX"),   (5, "V"),   (4, "IV"),
        (1, "I"),
    ]

    result = []
    for value, symbol in symbols:
        while num >= value:
            result.append(symbol)
            num -= value

    return "".join(result)


# -----------------------------------------------------------------------------
# Tests for int_to_roman
# -----------------------------------------------------------------------------

def test_int_to_roman_example1():
    assert int_to_roman(3749) == "MMMDCCXLIX"


def test_int_to_roman_example2():
    assert int_to_roman(58) == "LVIII"


def test_int_to_roman_example3():
    assert int_to_roman(1994) == "MCMXCIV"


def test_int_to_roman_subtractive():
    assert int_to_roman(4) == "IV"
    assert int_to_roman(9) == "IX"
    assert int_to_roman(40) == "XL"
    assert int_to_roman(90) == "XC"
    assert int_to_roman(400) == "CD"
    assert int_to_roman(900) == "CM"


def test_int_to_roman_single():
    assert int_to_roman(1) == "I"
    assert int_to_roman(1000) == "M"


def test_int_to_roman_max():
    assert int_to_roman(3999) == "MMMCMXCIX"


def length_of_last_word(s: str) -> int:
    """
    Length of Last Word

    Return the length of the last word in a string.

    Example:
        >>> length_of_last_word("   fly me   to   the moon  ")
        4

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    return len(s.split()[-1]) if s.split() else 0


# -----------------------------------------------------------------------------
# Tests for length_of_last_word
# -----------------------------------------------------------------------------

def test_length_of_last_word_example1():
    assert length_of_last_word("Hello World") == 5


def test_length_of_last_word_example2():
    assert length_of_last_word("   fly me   to   the moon  ") == 4


def test_length_of_last_word_example3():
    assert length_of_last_word("luffy is still joyboy") == 6


def test_length_of_last_word_single():
    assert length_of_last_word("a") == 1


def test_length_of_last_word_trailing_spaces():
    assert length_of_last_word("hello   ") == 5


def longest_common_prefix(strs: list[str]) -> str:
    """
    Longest Common Prefix

    Find the longest common prefix among an array of strings.

    Example:
        >>> longest_common_prefix(["flower", "flow", "flight"])
        'fl'

    Time Complexity: O(n * m) - n strings, m = shortest length
    Space Complexity: O(1)

    Compare char by char using the first string as reference.
    Stop when any string differs or runs out of characters.
    """
    if not strs:
        return ""

    for i, c in enumerate(strs[0]):
        for s in strs[1:]:
            if i >= len(s) or s[i] != c:
                return strs[0][:i]

    return strs[0]


# -----------------------------------------------------------------------------
# Tests for longest_common_prefix
# -----------------------------------------------------------------------------

def test_longest_common_prefix_example1():
    assert longest_common_prefix(["flower", "flow", "flight"]) == "fl"


def test_longest_common_prefix_example2():
    assert longest_common_prefix(["dog", "racecar", "car"]) == ""


def test_longest_common_prefix_single():
    assert longest_common_prefix(["alone"]) == "alone"


def test_longest_common_prefix_identical():
    assert longest_common_prefix(["test", "test", "test"]) == "test"


def test_longest_common_prefix_empty_string():
    assert longest_common_prefix(["", "b"]) == ""


def reverse_words(s: str) -> str:
    """
    Reverse Words in a String

    Reverse word order. Strip extra spaces.

    Example:
        >>> reverse_words("  hello world  ")
        'world hello'

    Time Complexity: O(n)
    Space Complexity: O(n)
    """
    return " ".join(s.split()[::-1])


# -----------------------------------------------------------------------------
# Tests for reverse_words
# -----------------------------------------------------------------------------

def test_reverse_words_example1():
    assert reverse_words("the sky is blue") == "blue is sky the"


def test_reverse_words_example2():
    assert reverse_words("  hello world  ") == "world hello"


def test_reverse_words_multiple_spaces():
    assert reverse_words("a   good   example") == "example good a"


def test_reverse_words_single():
    assert reverse_words("word") == "word"


def test_reverse_words_empty():
    assert reverse_words("") == ""


def zigzag_convert(s: str, num_rows: int) -> str:
    """
    Zigzag Conversion

    Write string in zigzag pattern across num_rows, then read line by line.

    Example (num_rows=3):
        P   A   H   N
        A P L S I I G
        Y   I   R

        "PAYPALISHIRING" → "PAHNAPLSIIGYIR"

    Time Complexity: O(n)
    Space Complexity: O(n)

    Simulate: bounce a row index up and down.
    Row goes 0,1,2,1,0,1,2,1,0... for num_rows=3.
    Collect chars into each row, then concatenate.
    """
    if num_rows == 1 or num_rows >= len(s):
        return s

    rows = [''] * num_rows
    curr_row = 0
    going_down = True

    for c in s:
        rows[curr_row] += c

        if curr_row == 0:
            going_down = True
        elif curr_row == num_rows - 1:
            going_down = False

        curr_row += 1 if going_down else -1

    return ''.join(rows)


# -----------------------------------------------------------------------------
# Tests for zigzag_convert
# -----------------------------------------------------------------------------

def test_zigzag_convert_example1():
    assert zigzag_convert("PAYPALISHIRING", 3) == "PAHNAPLSIIGYIR"


def test_zigzag_convert_example2():
    assert zigzag_convert("PAYPALISHIRING", 4) == "PINALSIGYAHRPI"


def test_zigzag_convert_single_row():
    assert zigzag_convert("A", 1) == "A"


def test_zigzag_convert_two_rows():
    assert zigzag_convert("ABCD", 2) == "ACBD"


def test_zigzag_convert_more_rows_than_chars():
    assert zigzag_convert("AB", 5) == "AB"


def str_str(haystack: str, needle: str) -> int:
    """
    Find First Occurrence in String

    Return index of first occurrence of needle in haystack, or -1.

    Example:
        >>> str_str("sadbutsad", "sad")
        0
        >>> str_str("leetcode", "leeto")
        -1

    Time Complexity: O(n * m) worst case
    Space Complexity: O(1)
    """
    idx = haystack.find(needle)
    return idx


# -----------------------------------------------------------------------------
# Tests for str_str
# -----------------------------------------------------------------------------

def test_str_str_example1():
    assert str_str("sadbutsad", "sad") == 0


def test_str_str_example2():
    assert str_str("leetcode", "leeto") == -1


def test_str_str_middle():
    assert str_str("hello", "ll") == 2


def test_str_str_empty_needle():
    assert str_str("hello", "") == 0


def test_str_str_full_match():
    assert str_str("abc", "abc") == 0


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
