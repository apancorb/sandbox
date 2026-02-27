"""
Stacks Pattern

A collection of algorithm problems solved using stacks.
"""


def is_valid_parentheses(s: str) -> bool:
    """
    Valid Parenthesis Expression

    Given a string representing an expression of parentheses containing the
    characters '(', ')', '[', ']', '{', or '}', determine if the expression
    forms a valid sequence of parentheses.

    A sequence of parentheses is valid if every opening parenthesis has a
    corresponding closing parenthesis, and no closing parenthesis appears
    before its matching opening parenthesis.

    Examples:
        >>> is_valid_parentheses("([]{})")
        True
        >>> is_valid_parentheses("([]{)}")
        False

    Push opening brackets onto stack. For closing brackets, check if top
    of stack matches. If not, or stack empty, invalid.

    Example "([]{})" step by step:
        '(' → push, stack=['(']
        '[' → push, stack=['(', '[']
        ']' → pop '[', matches → stack=['(']
        '{' → push, stack=['(', '{']
        '}' → pop '{', matches → stack=['(']
        ')' → pop '(', matches → stack=[]
        Stack empty → True ✓

    Time Complexity: O(n) - single pass through the string
    Space Complexity: O(n) - stack can hold up to n/2 opening brackets
    """
    stack = []
    pairs = {')': '(', '}': '{', ']': '['}

    for c in s:
        if not c in pairs:
            stack.append(c)
        else:
            if not stack or stack.pop() != pairs[c]:
                return False

    return len(stack) == 0


# -----------------------------------------------------------------------------
# Tests for is_valid_parentheses
# -----------------------------------------------------------------------------

def test_is_valid_parentheses_example_1():
    assert is_valid_parentheses("([]{})") is True


def test_is_valid_parentheses_example_2():
    assert is_valid_parentheses("([]{)}") is False


def test_is_valid_parentheses_empty():
    assert is_valid_parentheses("") is True


def test_is_valid_parentheses_single_pair():
    assert is_valid_parentheses("()") is True
    assert is_valid_parentheses("[]") is True
    assert is_valid_parentheses("{}") is True


def test_is_valid_parentheses_nested():
    assert is_valid_parentheses("({[]})") is True


def test_is_valid_parentheses_sequential():
    assert is_valid_parentheses("()[]{}") is True


def test_is_valid_parentheses_unmatched_open():
    assert is_valid_parentheses("(") is False
    assert is_valid_parentheses("(()") is False


def test_is_valid_parentheses_unmatched_close():
    assert is_valid_parentheses(")") is False
    assert is_valid_parentheses("())") is False


def test_is_valid_parentheses_wrong_order():
    assert is_valid_parentheses(")(") is False
    assert is_valid_parentheses("([)]") is False


def test_is_valid_parentheses_mismatched_types():
    assert is_valid_parentheses("(]") is False
    assert is_valid_parentheses("[}") is False
    assert is_valid_parentheses("{)") is False


def test_is_valid_parentheses_complex_valid():
    assert is_valid_parentheses("{[()]}()[{}]") is True


def test_is_valid_parentheses_complex_invalid():
    assert is_valid_parentheses("{[(])}") is False


def next_largest_to_right(nums: list[int]) -> list[int]:
    """
    Next Largest Number to the Right

    Given an integer array nums, return an output array res where, for each
    value nums[i], res[i] is the first number to the right that's larger than
    nums[i]. If no larger number exists to the right of nums[i], set res[i] to -1.

    Example:
        >>> next_largest_to_right([5, 2, 4, 6, 1])
        [6, 4, 6, -1, -1]

    Iterate from right to left. Maintain a stack of candidates. Pop smaller
    elements (they can't be the answer for any future element). Top of stack
    is the answer.

    Example [5, 2, 4, 6, 1]:
        i=4 (1): stack=[], no answer → res[4]=-1, push 1, stack=[1]
        i=3 (6): pop 1 (1 <= 6), stack=[], no answer → res[3]=-1, push 6, stack=[6]
        i=2 (4): stack=[6], top=6 > 4 → res[2]=6, push 4, stack=[6, 4]
        i=1 (2): stack=[6, 4], top=4 > 2 → res[1]=4, push 2, stack=[6, 4, 2]
        i=0 (5): pop 2 (2 <= 5), pop 4 (4 <= 5), top=6 > 5 → res[0]=6, push 5
        Result: [6, 4, 6, -1, -1] ✓

    Time Complexity: O(n) - each element pushed/popped at most once
    Space Complexity: O(n) - stack and result array
    """
    res = [-1] * len(nums)
    stack = []

    for i in range(len(nums) - 1, -1, -1):
        while stack and stack[-1] <= nums[i]:
            stack.pop()

        if stack:
            res[i] = stack[-1]

        stack.append(nums[i])

    return res


# -----------------------------------------------------------------------------
# Tests for next_largest_to_right
# -----------------------------------------------------------------------------

def test_next_largest_to_right_example():
    assert next_largest_to_right([5, 2, 4, 6, 1]) == [6, 4, 6, -1, -1]


def test_next_largest_to_right_empty():
    assert next_largest_to_right([]) == []


def test_next_largest_to_right_single():
    assert next_largest_to_right([5]) == [-1]


def test_next_largest_to_right_increasing():
    assert next_largest_to_right([1, 2, 3, 4, 5]) == [2, 3, 4, 5, -1]


def test_next_largest_to_right_decreasing():
    assert next_largest_to_right([5, 4, 3, 2, 1]) == [-1, -1, -1, -1, -1]


def test_next_largest_to_right_all_same():
    assert next_largest_to_right([3, 3, 3, 3]) == [-1, -1, -1, -1]


def test_next_largest_to_right_two_elements():
    assert next_largest_to_right([1, 2]) == [2, -1]
    assert next_largest_to_right([2, 1]) == [-1, -1]


def test_next_largest_to_right_duplicates():
    assert next_largest_to_right([2, 1, 2, 4, 3]) == [4, 2, 4, -1, -1]


def evaluate_expression(s: str) -> int:
    """
    Evaluate Expression

    Given a string representing a mathematical expression containing integers,
    parentheses, addition, and subtraction operators, evaluate and return the
    result of the expression.

    Example:
        >>> evaluate_expression("18-(7+(2-4))")
        13

    Track current result and sign. When hitting '(', push result and sign
    onto stack and reset. When hitting ')', pop and combine with outer context.

    Example "18-(7+(2-4))":
        '1' → curr_num=1
        '8' → curr_num=18
        '-' → res=18, sign=-1, curr_num=0
        '(' → push res=18, push sign=-1, stack=[18, -1], res=0, sign=1
        '7' → curr_num=7
        '+' → res=7, sign=1, curr_num=0
        '(' → push res=7, push sign=1, stack=[18, -1, 7, 1], res=0, sign=1
        '2' → curr_num=2
        '-' → res=2, sign=-1, curr_num=0
        '4' → curr_num=4
        ')' → res=2+4*(-1)=-2, pop sign=1, pop prev=7, res=7+(-2)*1=5
        ')' → res=5+0*1=5, pop sign=-1, pop prev=18, res=18+5*(-1)=13
        Answer: 13 ✓

    Time Complexity: O(n) - single pass through the string
    Space Complexity: O(n) - stack for nested parentheses
    """
    # res accumulates the running total for current scope
    # curr_num builds multi-digit numbers character by character
    # sign tracks whether to add or subtract the next number
    # stack saves outer context when we enter parentheses
    res = 0
    curr_num = 0
    sign = 1
    stack = []

    for c in s:
        if c.isdigit():
            # Build number digit by digit: "18" -> 1*10 + 8 = 18
            curr_num = curr_num * 10 + int(c)

        elif c == '+' or c == '-':
            # We've finished a number, so add it to result with its sign
            res += curr_num * sign
            # Reset for the next number
            curr_num = 0
            # The operator tells us the sign of the NEXT number
            sign = 1 if c == '+' else -1

        elif c == '(':
            # Entering a sub-expression: save current state
            # We push result first, then sign (so we pop sign first later)
            stack.append(res)
            stack.append(sign)
            # Start fresh inside the parentheses
            res = 0
            sign = 1

        elif c == ')':
            # Finish the current number inside parens
            res += curr_num * sign
            # Pop the sign that was BEFORE the '(' - it applies to this whole group
            prev_sign = stack.pop()
            # Pop the result from before we entered the parens
            prev_res = stack.pop()
            # Combine: outer_result + (sign_before_paren * inner_result)
            res = prev_res + res * prev_sign
            # Reset for whatever comes after the ')'
            curr_num = 0
            sign = 1

    # Don't forget the last number (no operator after it to trigger addition)
    return res + curr_num * sign


# -----------------------------------------------------------------------------
# Tests for evaluate_expression
# -----------------------------------------------------------------------------

def test_evaluate_expression_example():
    assert evaluate_expression("18-(7+(2-4))") == 13


def test_evaluate_expression_simple_addition():
    assert evaluate_expression("1+2+3") == 6


def test_evaluate_expression_simple_subtraction():
    assert evaluate_expression("10-3-2") == 5


def test_evaluate_expression_single_number():
    assert evaluate_expression("42") == 42


def test_evaluate_expression_with_spaces():
    assert evaluate_expression("1 + 2 - 3") == 0


def test_evaluate_expression_nested_parens():
    assert evaluate_expression("((1+2))") == 3


def test_evaluate_expression_negation_by_paren():
    assert evaluate_expression("1-(2+3)") == -4


def test_evaluate_expression_double_negation():
    assert evaluate_expression("1-(-2)") == 3


def test_evaluate_expression_complex():
    assert evaluate_expression("2-(5-6)") == 3


def test_evaluate_expression_leading_minus():
    assert evaluate_expression("-1+2") == 1


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
