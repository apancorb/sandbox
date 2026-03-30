"""
Fast and Slow Pointers Pattern (Floyd's Tortoise and Hare)

A collection of algorithm problems using two pointers moving at different speeds.

Core idea: slow moves 1 step, fast moves 2 steps.
- If there's a cycle, they WILL meet (fast gains 1 step per iteration)
- If no cycle, fast hits the end first

Common uses:
- Detecting repeating patterns (happy number, duplicate finder)
"""


def is_happy_number(n: int) -> bool:
    """
    Happy Number

    A happy number eventually reaches 1 when you repeatedly sum the
    squares of its digits. An unhappy number loops forever.

    Example:
        >>> is_happy_number(23)
        True
        # 2² + 3² = 13 → 1² + 3² = 10 → 1² + 0² = 1 ✓

        >>> is_happy_number(4)
        False
        # 4 → 16 → 37 → 58 → 89 → 145 → 42 → 20 → 4 (cycle!)

    Key insight: if a number isn't happy, the sequence CYCLES.
    This is just cycle detection! Use Floyd's algorithm:
        - slow computes one step
        - fast computes two steps
        - If fast hits 1 → happy
        - If slow == fast → cycle → unhappy

    digit_square_sum examples:
        123 → 1² + 2² + 3² = 1 + 4 + 9 = 14
        Get digits with % 10 and // 10:
            123 % 10 = 3, 123 // 10 = 12
            12 % 10 = 2,  12 // 10 = 1
            1 % 10 = 1,   1 // 10 = 0 → done

    Time Complexity: O(log n) per step, O(log n) steps
    Space Complexity: O(1)
    """
    def digit_square_sum(num):
        total = 0
        while num > 0:
            digit = num % 10   # last digit: 123 % 10 = 3
            num = num // 10    # remove it:  123 // 10 = 12
            total += digit * digit
        return total

    slow = n
    fast = n

    while True:
        slow = digit_square_sum(slow)
        fast = digit_square_sum(digit_square_sum(fast))
        if fast == 1:
            return True
        if slow == fast:
            return False


# -----------------------------------------------------------------------------
# Tests for is_happy_number
# -----------------------------------------------------------------------------

def test_is_happy_number_23():
    assert is_happy_number(23) == True


def test_is_happy_number_1():
    assert is_happy_number(1) == True


def test_is_happy_number_7():
    assert is_happy_number(7) == True


def test_is_happy_number_19():
    assert is_happy_number(19) == True


def test_is_unhappy_number_2():
    assert is_happy_number(2) == False


def test_is_unhappy_number_4():
    assert is_happy_number(4) == False


def test_is_unhappy_number_20():
    assert is_happy_number(20) == False


def test_is_happy_number_100():
    assert is_happy_number(100) == True


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
