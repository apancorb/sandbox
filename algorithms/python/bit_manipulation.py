"""
Bit Manipulation Pattern

A collection of algorithm problems using bitwise operations.

Key operations:
    x & 1       → last bit (0 or 1)
    x >> 1      → shift right (divide by 2)
    x << 1      → shift left (multiply by 2)
    x ^ x       → 0 (XOR with itself cancels out)
    x ^ 0       → x (XOR with 0 is identity)
    x & mask    → extract specific bits

Common tricks:
    x & (x-1)   → clear lowest set bit
    x ^ y       → bits that differ between x and y
    XOR all     → duplicates cancel, unique survives
"""


def hamming_weights(n: int) -> list[int]:
    """
    Hamming Weights of Integers

    Return array where element i = number of 1-bits in binary of i,
    for all integers 0 to n.

    Example:
        >>> hamming_weights(7)
        [0, 1, 1, 2, 1, 2, 2, 3]

        0=000→0, 1=001→1, 2=010→1, 3=011→2,
        4=100→1, 5=101→2, 6=110→2, 7=111→3

    For each number, count set bits by checking last bit (& 1)
    and shifting right (>> 1) until zero.

    Example count_bits(5):
        5 = 101
        101 & 1 = 1, count=1, shift → 10
        10 & 1 = 0, count=1, shift → 1
        1 & 1 = 1, count=2, shift → 0
        Done! 2 bits set

    Time Complexity: O(n * bits) where bits ≤ 32
    Space Complexity: O(n)
    """
    def count_bits(x):
        count = 0
        while x > 0:
            count += x & 1  # last bit: 1 or 0
            x >>= 1         # remove last bit
        return count

    return [count_bits(i) for i in range(n + 1)]


# -----------------------------------------------------------------------------
# Tests for hamming_weights
# -----------------------------------------------------------------------------

def test_hamming_weights_example():
    assert hamming_weights(7) == [0, 1, 1, 2, 1, 2, 2, 3]


def test_hamming_weights_zero():
    assert hamming_weights(0) == [0]


def test_hamming_weights_one():
    assert hamming_weights(1) == [0, 1]


def test_hamming_weights_two():
    assert hamming_weights(2) == [0, 1, 1]


def test_hamming_weights_fifteen():
    assert hamming_weights(15) == [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4]


def lonely_integer(nums: list[int]) -> int:
    """
    Lonely Integer

    Every number appears twice except one. Find it.

    Example:
        >>> lonely_integer([1, 3, 3, 2, 1])
        2

    XOR trick: a ^ a = 0 and a ^ 0 = a
    XOR all numbers together → duplicates cancel out, unique survives.

    Example [1, 3, 3, 2, 1]:
        0 ^ 1 = 1
        1 ^ 3 = 2  (binary: 01 ^ 11 = 10)
        2 ^ 3 = 1  (binary: 10 ^ 11 = 01)  ← 3s cancelled!
        1 ^ 2 = 3  (binary: 01 ^ 10 = 11)
        3 ^ 1 = 2  (binary: 11 ^ 01 = 10)  ← 1s cancelled!
        Answer: 2

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    result = 0
    for num in nums:
        result ^= num
    return result


# -----------------------------------------------------------------------------
# Tests for lonely_integer
# -----------------------------------------------------------------------------

def test_lonely_integer_example():
    assert lonely_integer([1, 3, 3, 2, 1]) == 2


def test_lonely_integer_single():
    assert lonely_integer([42]) == 42


def test_lonely_integer_at_start():
    assert lonely_integer([5, 1, 1, 2, 2]) == 5


def test_lonely_integer_at_end():
    assert lonely_integer([1, 1, 2, 2, 7]) == 7


def test_lonely_integer_negative():
    assert lonely_integer([-1, 2, 2, -1, -3]) == -3


def swap_odd_even_bits(n: int) -> int:
    """
    Swap Odd and Even Bits

    Swap every pair of adjacent bits in a 32-bit integer.

    Example:
        41 = 1 0 1 0 0 1
             ↓ ↑ ↓ ↑ ↓ ↑
        22 = 0 1 0 1 1 0

    Use bitmasks to isolate even and odd bits separately, then shift
    them into each other's positions and merge with OR. This swaps
    every adjacent pair in one shot without any loops.

    Example n=41 (101001):
        even bits: 101001 & 010101 = 000001 → shift left  → 000010
        odd bits:  101001 & 101010 = 101000 → shift right → 010100
        merge: 000010 | 010100 = 010110 = 22 ✓

    Time Complexity: O(1)
    Space Complexity: O(1)

    Masks (32-bit):
        EVEN_MASK = 0x55555555 = 0101 0101 ... (selects even bits)
        ODD_MASK  = 0xAAAAAAAA = 1010 1010 ... (selects odd bits)

    Steps:
        1. Extract even bits: n & EVEN_MASK
        2. Extract odd bits:  n & ODD_MASK
        3. Shift even bits left by 1 (move to odd positions)
        4. Shift odd bits right by 1 (move to even positions)
        5. Merge with OR
    """
    EVEN_MASK = 0x55555555  # 0101...
    ODD_MASK = 0xAAAAAAAA   # 1010...

    even_bits = n & EVEN_MASK
    odd_bits = n & ODD_MASK

    return (even_bits << 1) | (odd_bits >> 1)


# -----------------------------------------------------------------------------
# Tests for swap_odd_even_bits
# -----------------------------------------------------------------------------

def test_swap_odd_even_bits_example1():
    assert swap_odd_even_bits(41) == 22


def test_swap_odd_even_bits_example2():
    assert swap_odd_even_bits(23) == 43


def test_swap_odd_even_bits_zero():
    assert swap_odd_even_bits(0) == 0


def test_swap_odd_even_bits_all_ones():
    assert swap_odd_even_bits(0b1111) == 0b1111


def test_swap_odd_even_bits_alternating():
    assert swap_odd_even_bits(0b10101010) == 0b01010101


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
