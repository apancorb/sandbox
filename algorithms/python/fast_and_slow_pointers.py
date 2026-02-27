"""
Fast and Slow Pointers Pattern (Floyd's Tortoise and Hare)

A collection of algorithm problems using two pointers moving at different speeds.

Core idea: slow moves 1 step, fast moves 2 steps.
- If there's a cycle, they WILL meet (fast gains 1 step per iteration)
- If no cycle, fast hits the end first

Common uses:
- Cycle detection in linked lists or sequences
- Finding the middle of a linked list
- Detecting repeating patterns (happy number, duplicate finder)
"""


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def has_cycle(head: ListNode | None) -> bool:
    """
    Linked List Loop

    Determine if a linked list contains a cycle.

    Example:
        1 → 2 → 3 → 4
            ^         |
            |_________|
        → True (node 4 points back to node 2)

    Floyd's Cycle Detection:
        - slow moves 1 step, fast moves 2 steps
        - If no cycle: fast reaches None → return False
        - If cycle: fast gains 1 step per iteration on slow
          gap closes: k → k-1 → k-2 → ... → 0 (they meet!)

    Analogy: two runners on a circular track.
    The faster one always laps the slower one.

    Example 1→2→3→4→(back to 2):
        slow=1, fast=1
        slow=2, fast=3
        slow=3, fast=3→4→2, wait let me redo...

        slow=1, fast=1
        Step 1: slow=2, fast=3
        Step 2: slow=3, fast=3 (4→2, then 2→3)...
        Actually: fast moves 2 steps: 3→4, 4→2 → fast=2
        Step 2: slow=3, fast=2
        Step 3: slow=4, fast=4 → MATCH! cycle detected

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    slow = head
    fast = head

    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
        if slow is fast:
            return True

    return False


# -----------------------------------------------------------------------------
# Tests for has_cycle
# -----------------------------------------------------------------------------

def test_has_cycle_with_cycle():
    n1, n2, n3, n4 = ListNode(1), ListNode(2), ListNode(3), ListNode(4)
    n1.next, n2.next, n3.next, n4.next = n2, n3, n4, n2  # 4→2 cycle
    assert has_cycle(n1) == True


def test_has_cycle_no_cycle():
    n1, n2, n3 = ListNode(1), ListNode(2), ListNode(3)
    n1.next, n2.next = n2, n3
    assert has_cycle(n1) == False


def test_has_cycle_single_no_cycle():
    assert has_cycle(ListNode(1)) == False


def test_has_cycle_single_self_loop():
    n1 = ListNode(1)
    n1.next = n1
    assert has_cycle(n1) == True


def test_has_cycle_empty():
    assert has_cycle(None) == False


def test_has_cycle_two_nodes_cycle():
    n1, n2 = ListNode(1), ListNode(2)
    n1.next, n2.next = n2, n1  # 2→1 cycle
    assert has_cycle(n1) == True


def test_has_cycle_two_nodes_no_cycle():
    n1, n2 = ListNode(1), ListNode(2)
    n1.next = n2
    assert has_cycle(n1) == False


def test_has_cycle_long_list_cycle_at_end():
    nodes = [ListNode(i) for i in range(1, 6)]
    for i in range(4):
        nodes[i].next = nodes[i + 1]
    nodes[4].next = nodes[2]  # 5→3 cycle
    assert has_cycle(nodes[0]) == True


def find_middle(head: ListNode | None) -> ListNode | None:
    """
    Linked List Midpoint

    Find the middle node. If two middles, return the second one.

    Example:
        >>> 1 → 2 → 3 → 4 → 5 → 6 → 7
        Middle = 4

        >>> 1 → 2 → 3 → 4 → 5 → 6
        Middle = 4 (second of the two middles)

    When fast reaches the end, slow is at the middle.
    Fast moves 2x speed, so when fast travels n steps, slow travels n/2.

    Example [1, 2, 3, 4, 5]:
        slow=1, fast=1
        Step 1: slow=2, fast=3
        Step 2: slow=3, fast=5
        fast.next is None → stop. slow=3 ✓

    Example [1, 2, 3, 4, 5, 6]:
        slow=1, fast=1
        Step 1: slow=2, fast=3
        Step 2: slow=3, fast=5
        Step 3: slow=4, fast=None (5.next=6, 6.next=None)
        fast is None → stop. slow=4 ✓ (second middle)

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    slow = head
    fast = head

    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next

    return slow


# -----------------------------------------------------------------------------
# Tests for find_middle
# -----------------------------------------------------------------------------

def test_find_middle_odd():
    nodes = [ListNode(i) for i in range(1, 8)]  # 1-7
    for i in range(6):
        nodes[i].next = nodes[i + 1]
    assert find_middle(nodes[0]).val == 4


def test_find_middle_even():
    nodes = [ListNode(i) for i in range(1, 7)]  # 1-6
    for i in range(5):
        nodes[i].next = nodes[i + 1]
    assert find_middle(nodes[0]).val == 4


def test_find_middle_single():
    assert find_middle(ListNode(42)).val == 42


def test_find_middle_two():
    n1, n2 = ListNode(1), ListNode(2)
    n1.next = n2
    assert find_middle(n1).val == 2


def test_find_middle_three():
    n1, n2, n3 = ListNode(1), ListNode(2), ListNode(3)
    n1.next, n2.next = n2, n3
    assert find_middle(n1).val == 2


def test_find_middle_four():
    nodes = [ListNode(i) for i in range(1, 5)]  # 1-4
    for i in range(3):
        nodes[i].next = nodes[i + 1]
    assert find_middle(nodes[0]).val == 3


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
