"""
Linked Lists Pattern

A collection of algorithm problems using linked lists.

In Python, linked lists use a simple node class:
    class ListNode:
        def __init__(self, val=0, next=None):
            self.val = val
            self.next = next

Common techniques:
- Dummy node: create a fake head to simplify edge cases (empty list, head removal)
- Two pointers: fast/slow for finding midpoints, kth from end
- In-place reversal: prev/curr/next pointer juggling

Helper functions:
    from_list([1,2,3]) → 1→2→3→None     (build linked list from array)
    to_list(head)       → [1,2,3]         (convert back for easy testing)
"""

from collections import OrderedDict


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def from_list(values: list[int]) -> ListNode | None:
    """Build linked list from array. Returns head node or None."""
    if not values:
        return None
    head = ListNode(values[0])
    curr = head
    for val in values[1:]:
        curr.next = ListNode(val)
        curr = curr.next
    return head


def to_list(head: ListNode | None) -> list[int]:
    """Convert linked list to array for easy comparison."""
    result = []
    while head:
        result.append(head.val)
        head = head.next
    return result


def reverse_list(head: ListNode | None) -> ListNode | None:
    """
    Linked List Reversal

    Reverse a singly linked list.

    Example:
        >>> to_list(reverse_list(from_list([1, 2, 3, 4, 5])))
        [5, 4, 3, 2, 1]

    Time Complexity: O(n)
    Space Complexity: O(1)

    Use three pointers: prev, curr, next.
    At each step, flip curr's pointer to point backwards.

    Example [1, 2, 3]:
        Start:  prev=None, curr=1→2→3

        Step 1: save next=2→3
                flip: 1→None (curr.next = prev)
                advance: prev=1, curr=2→3

        Step 2: save next=3
                flip: 2→1→None
                advance: prev=2→1, curr=3

        Step 3: save next=None
                flip: 3→2→1→None
                advance: prev=3→2→1, curr=None

        Done! return prev = 3→2→1
    """
    prev = None
    curr = head

    while curr:
        next_node = curr.next   # save next
        curr.next = prev        # flip pointer
        prev = curr             # advance prev
        curr = next_node        # advance curr

    return prev


# -----------------------------------------------------------------------------
# Tests for reverse_list
# -----------------------------------------------------------------------------

def test_reverse_list_basic():
    assert to_list(reverse_list(from_list([1, 2, 3, 4, 5]))) == [5, 4, 3, 2, 1]


def test_reverse_list_two():
    assert to_list(reverse_list(from_list([1, 2]))) == [2, 1]


def test_reverse_list_single():
    assert to_list(reverse_list(from_list([42]))) == [42]


def test_reverse_list_empty():
    assert to_list(reverse_list(None)) == []


def test_reverse_list_negative():
    assert to_list(reverse_list(from_list([-1, -2, -3]))) == [-3, -2, -1]


def test_reverse_list_duplicates():
    assert to_list(reverse_list(from_list([1, 1, 2, 2, 3]))) == [3, 2, 2, 1, 1]


def test_reverse_list_large():
    values = list(range(1, 101))
    assert to_list(reverse_list(from_list(values))) == list(range(100, 0, -1))


def reverse_between(head: ListNode | None, left: int, right: int) -> ListNode | None:
    """
    Reverse Linked List II

    Reverse nodes from position left to right (1-indexed).

    Example:
        >>> to_list(reverse_between(from_list([1, 2, 3, 4, 5]), 2, 4))
        [1, 4, 3, 2, 5]

    Time Complexity: O(n)
    Space Complexity: O(1)

    Three phases:
        1. Walk to node before position left
        2. Reverse the sublist from left to right
        3. Reconnect: before→reversed, tail of reversed→rest

    Example [1, 2, 3, 4, 5], left=2, right=4:
        Phase 1: walk to node 1 (before position 2)
            before → [1]

        Phase 2: reverse positions 2-4 (nodes 2,3,4)
            [2, 3, 4] becomes [4, 3, 2]

        Phase 3: reconnect
            [1] → [4, 3, 2] → [5]
            Result: [1, 4, 3, 2, 5]
    """
    dummy = ListNode(0, head)

    # Phase 1: walk to node before left
    before = dummy
    for _ in range(left - 1):
        before = before.next

    # Phase 2: reverse from left to right
    prev = None
    curr = before.next
    for _ in range(right - left + 1):
        next_node = curr.next
        curr.next = prev
        prev = curr
        curr = next_node

    # Phase 3: reconnect
    # before.next is the old left node (now tail of reversed section)
    before.next.next = curr   # tail → rest of list
    before.next = prev        # before → new head of reversed section

    return dummy.next


# -----------------------------------------------------------------------------
# Tests for reverse_between
# -----------------------------------------------------------------------------

def test_reverse_between_example():
    assert to_list(reverse_between(from_list([1, 2, 3, 4, 5]), 2, 4)) == [1, 4, 3, 2, 5]


def test_reverse_between_single():
    assert to_list(reverse_between(from_list([5]), 1, 1)) == [5]


def test_reverse_between_full():
    assert to_list(reverse_between(from_list([1, 2, 3, 4, 5]), 1, 5)) == [5, 4, 3, 2, 1]


def test_reverse_between_first_two():
    assert to_list(reverse_between(from_list([1, 2, 3, 4, 5]), 1, 2)) == [2, 1, 3, 4, 5]


def test_reverse_between_last_two():
    assert to_list(reverse_between(from_list([1, 2, 3, 4, 5]), 4, 5)) == [1, 2, 3, 5, 4]


def test_reverse_between_two_elements():
    assert to_list(reverse_between(from_list([3, 5]), 1, 2)) == [5, 3]


def remove_kth_from_end(head: ListNode | None, k: int) -> ListNode | None:
    """
    Remove the Kth Last Node

    Remove the kth node from the end of the list.

    Example:
        >>> to_list(remove_kth_from_end(from_list([1, 2, 3, 4, 5]), 2))
        [1, 2, 3, 5]
        # Removed 4 (2nd from end)

    Time Complexity: O(n)
    Space Complexity: O(1)

    Two approaches:
        1. Count length, then walk to (len - k - 1) to find node before target
        2. Two pointers: advance fast by k, then move both until fast hits end

    Using approach 1 with dummy node:
        Example [1, 2, 3, 4, 5], k=2:
            len = 5
            target is at position (5 - 2) = 3 (0-indexed)
            walk to position 2 (node before target): node 3
            skip: 3.next = 3.next.next (skip node 4)
            Result: [1, 2, 3, 5]
    """
    dummy = ListNode(0, head)

    # Count length
    length = 0
    curr = head
    while curr:
        length += 1
        curr = curr.next

    # Walk to node BEFORE the target (len - k steps from dummy)
    curr = dummy
    for _ in range(length - k):
        curr = curr.next

    # Skip over target node
    curr.next = curr.next.next

    return dummy.next


# -----------------------------------------------------------------------------
# Tests for remove_kth_from_end
# -----------------------------------------------------------------------------

def test_remove_kth_from_end_middle():
    assert to_list(remove_kth_from_end(from_list([1, 2, 3, 4, 5]), 2)) == [1, 2, 3, 5]


def test_remove_kth_from_end_last():
    assert to_list(remove_kth_from_end(from_list([1, 2, 3, 4, 5]), 1)) == [1, 2, 3, 4]


def test_remove_kth_from_end_first():
    assert to_list(remove_kth_from_end(from_list([1, 2, 3, 4, 5]), 5)) == [2, 3, 4, 5]


def test_remove_kth_from_end_single():
    assert to_list(remove_kth_from_end(from_list([1]), 1)) == []


def test_remove_kth_from_end_two_remove_first():
    assert to_list(remove_kth_from_end(from_list([1, 2]), 2)) == [2]


def test_remove_kth_from_end_two_remove_last():
    assert to_list(remove_kth_from_end(from_list([1, 2]), 1)) == [1]


def delete_duplicates(head: ListNode | None) -> ListNode | None:
    """
    Remove Duplicates from Sorted List II

    Given a sorted linked list, delete ALL nodes that have duplicate values.
    Only keep nodes with unique values.

    Example:
        >>> to_list(delete_duplicates(from_list([1, 2, 3, 3, 4, 4, 5])))
        [1, 2, 5]

    Time Complexity: O(n)
    Space Complexity: O(1)

    Use a dummy node. For each group of nodes, check if it's a duplicate run.
    If yes, skip the entire group. If no, keep it and advance.

    Example [1, 2, 3, 3, 4, 4, 5]:
        dummy → 1 → 2 → 3 → 3 → 4 → 4 → 5
        ^prev

        prev=dummy: curr=1, next=2 (different) → keep 1, prev=1
        prev=1:     curr=2, next=3 (different) → keep 2, prev=2
        prev=2:     curr=3, next=3 (SAME!) → skip all 3s → prev.next=4
        prev=2:     curr=4, next=4 (SAME!) → skip all 4s → prev.next=5
        prev=2:     curr=5, next=None      → keep 5, prev=5

        Result: [1, 2, 5]
    """
    dummy = ListNode(0, head)
    prev = dummy

    while prev.next:
        curr = prev.next
        # Check if this starts a duplicate run
        is_dup = False
        while curr.next and curr.val == curr.next.val:
            curr.next = curr.next.next  # skip duplicate
            is_dup = True

        if is_dup:
            # Skip curr entirely (it was part of duplicates)
            prev.next = curr.next
        else:
            # Keep curr, advance prev
            prev = prev.next

    return dummy.next


# -----------------------------------------------------------------------------
# Tests for delete_duplicates
# -----------------------------------------------------------------------------

def test_delete_duplicates_example1():
    assert to_list(delete_duplicates(from_list([1, 2, 3, 3, 4, 4, 5]))) == [1, 2, 5]


def test_delete_duplicates_example2():
    assert to_list(delete_duplicates(from_list([1, 1, 1, 2, 3]))) == [2, 3]


def test_delete_duplicates_all_same():
    assert to_list(delete_duplicates(from_list([1, 1, 1]))) == []


def test_delete_duplicates_no_dups():
    assert to_list(delete_duplicates(from_list([1, 2, 3]))) == [1, 2, 3]


def test_delete_duplicates_empty():
    assert to_list(delete_duplicates(None)) == []


def test_delete_duplicates_single():
    assert to_list(delete_duplicates(from_list([1]))) == [1]


def rotate_list(head: ListNode | None, k: int) -> ListNode | None:
    """
    Rotate List

    Rotate the list to the right by k places.

    Example:
        >>> to_list(rotate_list(from_list([1, 2, 3, 4, 5]), 2))
        [4, 5, 1, 2, 3]

    Time Complexity: O(n)
    Space Complexity: O(1)

    Steps:
        1. Count length, reduce k by len (k % len handles k > len)
        2. Find new tail at position (len - k - 1)
        3. Split: new_head = new_tail.next
        4. Connect old tail to old head

    Example [1, 2, 3, 4, 5], k=2:
        len=5, k=2%5=2
        new_tail at position (5-2-1)=2 → node 3
        Split: [1, 2, 3] | [4, 5]
        Connect: [4, 5] → [1, 2, 3]
        Result: [4, 5, 1, 2, 3]
    """
    if not head:
        return None

    # Count length and find tail
    length = 1
    tail = head
    while tail.next:
        length += 1
        tail = tail.next

    # k % len handles k > len
    k = k % length
    if k == 0:
        return head

    # Find new tail at position (len - k - 1)
    new_tail = head
    for _ in range(length - k - 1):
        new_tail = new_tail.next

    # Split and reconnect
    new_head = new_tail.next
    new_tail.next = None
    tail.next = head

    return new_head


# -----------------------------------------------------------------------------
# Tests for rotate_list
# -----------------------------------------------------------------------------

def test_rotate_list_example1():
    assert to_list(rotate_list(from_list([1, 2, 3, 4, 5]), 2)) == [4, 5, 1, 2, 3]


def test_rotate_list_example2():
    assert to_list(rotate_list(from_list([0, 1, 2]), 4)) == [2, 0, 1]


def test_rotate_list_empty():
    assert to_list(rotate_list(None, 5)) == []


def test_rotate_list_single():
    assert to_list(rotate_list(from_list([1]), 3)) == [1]


def test_rotate_list_k_zero():
    assert to_list(rotate_list(from_list([1, 2, 3]), 0)) == [1, 2, 3]


def test_rotate_list_k_equals_len():
    assert to_list(rotate_list(from_list([1, 2, 3]), 3)) == [1, 2, 3]


def partition_list(head: ListNode | None, x: int) -> ListNode | None:
    """
    Partition List

    Rearrange so all nodes < x come before nodes >= x.
    Preserve original relative order within each group.

    Example:
        >>> to_list(partition_list(from_list([1, 4, 3, 2, 5, 2]), 3))
        [1, 2, 2, 4, 3, 5]

    Time Complexity: O(n)
    Space Complexity: O(1) - reuse existing nodes

    Build two separate chains: "less" and "greater or equal".
    Then connect less_tail → greater_head.

    Example [1, 4, 3, 2, 5, 2], x=3:
        1 < 3 → less:    [1]
        4 >= 3 → greater: [4]
        3 >= 3 → greater: [4, 3]
        2 < 3 → less:    [1, 2]
        5 >= 3 → greater: [4, 3, 5]
        2 < 3 → less:    [1, 2, 2]

        Connect: [1, 2, 2] → [4, 3, 5]
        Result: [1, 2, 2, 4, 3, 5]
    """
    less_dummy = ListNode(0)
    greater_dummy = ListNode(0)
    less = less_dummy
    greater = greater_dummy

    curr = head
    while curr:
        if curr.val < x:
            less.next = curr
            less = less.next
        else:
            greater.next = curr
            greater = greater.next
        curr = curr.next

    # Connect less chain → greater chain
    greater.next = None       # terminate greater chain
    less.next = greater_dummy.next

    return less_dummy.next


# -----------------------------------------------------------------------------
# Tests for partition_list
# -----------------------------------------------------------------------------

def test_partition_list_example1():
    assert to_list(partition_list(from_list([1, 4, 3, 2, 5, 2]), 3)) == [1, 2, 2, 4, 3, 5]


def test_partition_list_example2():
    assert to_list(partition_list(from_list([2, 1]), 2)) == [1, 2]


def test_partition_list_empty():
    assert to_list(partition_list(None, 5)) == []


def test_partition_list_all_less():
    assert to_list(partition_list(from_list([1, 2, 3]), 5)) == [1, 2, 3]


def test_partition_list_all_greater():
    assert to_list(partition_list(from_list([5, 6, 7]), 3)) == [5, 6, 7]


def test_partition_list_single():
    assert to_list(partition_list(from_list([1]), 2)) == [1]


def merge_two_lists(l1: ListNode | None, l2: ListNode | None) -> ListNode | None:
    """
    Merge Two Sorted Lists

    Merge two sorted linked lists into one sorted list.

    Example:
        >>> to_list(merge_two_lists(from_list([1, 2, 4]), from_list([1, 3, 4])))
        [1, 1, 2, 3, 4, 4]

    Time Complexity: O(n + m)
    Space Complexity: O(1) - reuse existing nodes

    Use a dummy node and compare heads of both lists.
    Always pick the smaller value and advance that pointer.

    Example l1=[1, 2, 4], l2=[1, 3, 4]:
        Compare 1 vs 1 → take l1's 1
        Compare 2 vs 1 → take l2's 1
        Compare 2 vs 3 → take l1's 2
        Compare 4 vs 3 → take l2's 3
        Compare 4 vs 4 → take l1's 4
        l1 exhausted   → append l2's [4]
        Result: [1, 1, 2, 3, 4, 4]
    """
    dummy = ListNode(0)
    curr = dummy

    while l1 and l2:
        if l1.val <= l2.val:
            curr.next = l1
            l1 = l1.next
        else:
            curr.next = l2
            l2 = l2.next
        curr = curr.next

    # Append remaining
    curr.next = l1 if l1 else l2

    return dummy.next


# -----------------------------------------------------------------------------
# Tests for merge_two_lists
# -----------------------------------------------------------------------------

def test_merge_two_lists_example():
    assert to_list(merge_two_lists(from_list([1, 2, 4]), from_list([1, 3, 4]))) == [1, 1, 2, 3, 4, 4]


def test_merge_two_lists_both_empty():
    assert to_list(merge_two_lists(None, None)) == []


def test_merge_two_lists_one_empty():
    assert to_list(merge_two_lists(from_list([1, 2, 3]), None)) == [1, 2, 3]


def test_merge_two_lists_other_empty():
    assert to_list(merge_two_lists(None, from_list([4, 5, 6]))) == [4, 5, 6]


def test_merge_two_lists_different_lengths():
    assert to_list(merge_two_lists(from_list([1, 2]), from_list([3, 4, 5, 6]))) == [1, 2, 3, 4, 5, 6]


class LRUCache:
    """
    LRU Cache

    Design a Least Recently Used cache with get and put operations.
    When capacity is exceeded, evict the least recently used key.

    Example:
        >>> cache = LRUCache(3)
        >>> cache.put(1, 100); cache.put(2, 250)
        >>> cache.get(2)
        250
        >>> cache.put(4, 300); cache.put(3, 200)  # evicts key 1
        >>> cache.get(1)
        -1

    Time Complexity: O(1) for both get and put
    Space Complexity: O(capacity)

    Python's OrderedDict maintains insertion order and supports
    move_to_end() for O(1) reordering. Perfect for LRU!

    How it works:
        - get(key): if exists, move to end (most recent), return value
        - put(key, val): if exists, update + move to end
                         if new + full, pop first item (least recent)
                         insert at end (most recent)

    OrderedDict internals: doubly linked list + hash map
    (same as the classic LRU implementation, just built-in)
    """

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = OrderedDict()

    def get(self, key: int) -> int:
        if key not in self.cache:
            return -1
        # Move to end = most recently used
        self.cache.move_to_end(key)
        return self.cache[key]

    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            # Update value and move to end
            self.cache.move_to_end(key)
            self.cache[key] = value
        else:
            if len(self.cache) >= self.capacity:
                # Evict least recently used (first item)
                self.cache.popitem(last=False)
            self.cache[key] = value


# -----------------------------------------------------------------------------
# Tests for LRUCache
# -----------------------------------------------------------------------------

def test_lru_cache_example():
    cache = LRUCache(3)
    cache.put(1, 100)
    cache.put(2, 250)
    assert cache.get(2) == 250
    cache.put(4, 300)
    cache.put(3, 200)  # evicts key 1
    assert cache.get(4) == 300
    assert cache.get(1) == -1


def test_lru_cache_update_existing():
    cache = LRUCache(2)
    cache.put(1, 100)
    cache.put(1, 200)
    assert cache.get(1) == 200


def test_lru_cache_get_updates_recency():
    cache = LRUCache(2)
    cache.put(1, 100)
    cache.put(2, 200)
    cache.get(1)         # makes key 1 most recently used
    cache.put(3, 300)    # should evict key 2, not key 1
    assert cache.get(1) == 100
    assert cache.get(2) == -1
    assert cache.get(3) == 300


def test_lru_cache_capacity_one():
    cache = LRUCache(1)
    cache.put(1, 100)
    cache.put(2, 200)  # evicts key 1
    assert cache.get(1) == -1
    assert cache.get(2) == 200


def test_lru_cache_get_nonexistent():
    cache = LRUCache(2)
    assert cache.get(1) == -1
    cache.put(1, 100)
    assert cache.get(2) == -1


def add_two_numbers(l1: ListNode | None, l2: ListNode | None) -> ListNode | None:
    """
    Add Two Numbers

    Two linked lists store digits in reverse order.
    Add the numbers and return the sum as a linked list.

    Example:
        >>> to_list(add_two_numbers(from_list([2, 4, 3]), from_list([5, 6, 4])))
        [7, 0, 8]
        # 342 + 465 = 807 (stored as 7→0→8)

    Time Complexity: O(max(m, n))
    Space Complexity: O(max(m, n)) - new list

    Walk both lists simultaneously, adding digits + carry.
    Like grade school addition, right to left (but lists are already reversed).

    Example [2,4,3] + [5,6,4]:
        2+5+0 = 7, carry=0 → 7
        4+6+0 = 10, carry=1 → 0
        3+4+1 = 8, carry=0 → 8
        Result: [7, 0, 8]

    Example [9,9] + [1]:
        9+1+0 = 10, carry=1 → 0
        9+0+1 = 10, carry=1 → 0
        0+0+1 = 1,  carry=0 → 1
        Result: [0, 0, 1]  (99 + 1 = 100)
    """
    dummy = ListNode(0)
    curr = dummy
    carry = 0

    while l1 or l2 or carry:
        val1 = l1.val if l1 else 0
        val2 = l2.val if l2 else 0

        total = val1 + val2 + carry
        carry = total // 10   # e.g. 15 // 10 = 1 (carry the 1)

        curr.next = ListNode(total % 10)  # e.g. 15 % 10 = 5 (keep the 5)
        curr = curr.next

        l1 = l1.next if l1 else None
        l2 = l2.next if l2 else None

    return dummy.next


# -----------------------------------------------------------------------------
# Tests for add_two_numbers
# -----------------------------------------------------------------------------

def test_add_two_numbers_example1():
    assert to_list(add_two_numbers(from_list([2, 4, 3]), from_list([5, 6, 4]))) == [7, 0, 8]


def test_add_two_numbers_zeros():
    assert to_list(add_two_numbers(from_list([0]), from_list([0]))) == [0]


def test_add_two_numbers_long_carry():
    assert to_list(add_two_numbers(from_list([9, 9, 9, 9, 9, 9, 9]), from_list([9, 9, 9, 9]))) == [8, 9, 9, 9, 0, 0, 0, 1]


def test_add_two_numbers_different_lengths():
    assert to_list(add_two_numbers(from_list([1, 2, 3]), from_list([4, 5]))) == [5, 7, 3]


def test_add_two_numbers_carry_propagates():
    assert to_list(add_two_numbers(from_list([9, 9]), from_list([1]))) == [0, 0, 1]


def copy_random_list(head):
    """
    Copy List with Random Pointer

    Each node has val, next, and a random pointer (can point to any node or None).
    Create a deep copy of the list.

    Time Complexity: O(n) - two passes
    Space Complexity: O(n) - hash map of old→new nodes

    Two-pass approach:
        Pass 1: Create all new nodes, map old_node → new_node
        Pass 2: Wire up next and random pointers using the map

    Example: A(random→C) → B(random→A) → C(random→B)
        Pass 1: create A', B', C'
                map = {A: A', B: B', C: C'}
        Pass 2: A'.next = map[A.next] = B'
                A'.random = map[A.random] = C'
                B'.next = C', B'.random = A'
                C'.next = None, C'.random = B'
    """
    if not head:
        return None

    # Pass 1: create all new nodes, map old → new
    old_to_new = {}
    curr = head
    while curr:
        old_to_new[curr] = RandomListNode(curr.val)
        curr = curr.next

    # Pass 2: wire up next and random pointers
    curr = head
    while curr:
        clone = old_to_new[curr]
        clone.next = old_to_new.get(curr.next)
        clone.random = old_to_new.get(curr.random)
        curr = curr.next

    return old_to_new[head]


class RandomListNode:
    def __init__(self, val=0, next=None, random=None):
        self.val = val
        self.next = next
        self.random = random


# -----------------------------------------------------------------------------
# Tests for copy_random_list
# -----------------------------------------------------------------------------

def test_copy_random_list_empty():
    assert copy_random_list(None) is None


def test_copy_random_list_single():
    node = RandomListNode(7)
    copied = copy_random_list(node)
    assert copied is not None
    assert copied.val == 7
    assert copied is not node  # deep copy


def test_copy_random_list_with_random():
    node1 = RandomListNode(1)
    node2 = RandomListNode(2)
    node1.next = node2
    node1.random = node2
    node2.random = node2  # self-reference

    copied = copy_random_list(node1)
    assert copied.val == 1
    assert copied.next.val == 2
    assert copied.random is copied.next       # 1's random → copied 2
    assert copied.next.random is copied.next   # 2's random → itself
    assert copied is not node1                 # deep copy


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
