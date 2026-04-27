use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Singly linked list node
#[derive(Debug, PartialEq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    /// Helper to create a linked list from a vector
    pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &val in values.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    /// Helper to convert a linked list to a vector
    pub fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut current = head;
        while let Some(node) = current {
            result.push(node.val);
            current = &node.next;
        }
        result
    }
}

/// Linked List Reversal
///
/// Reverse a singly linked list.
///
/// # Examples
///
/// ```text
/// Input:  1 -> 2 -> 3 -> 4 -> 5 -> None
/// Output: 5 -> 4 -> 3 -> 2 -> 1 -> None
/// ```
///
/// Use three pointers: prev, curr, next. At each step, flip curr's pointer to
/// point backwards.
///
/// Example walkthrough for [1, 2, 3]:
///
/// ```text
/// Start:  prev=None, curr=1->2->3
///
/// Step 1: save next=2->3
///         flip: 1->None (curr.next = prev)
///         advance: prev=1, curr=2->3
///
/// Step 2: save next=3
///         flip: 2->1->None
///         advance: prev=2->1, curr=3
///
/// Step 3: save next=None
///         flip: 3->2->1->None
///         advance: prev=3->2->1, curr=None
///
/// Done! return prev = 3->2->1
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the list
/// - Space: O(1) — only three pointer variables
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    if head.as_ref().unwrap().next.is_none() {
        return head;
    }

    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        curr = next;
    }

    prev
}

/// Reverse Linked List II
///
/// Reverse nodes from position left to right (1-indexed).
///
/// # Examples
///
/// ```text
/// Input:  head = [1, 2, 3, 4, 5], left = 2, right = 4
/// Output: [1, 4, 3, 2, 5]
/// ```
///
/// Three phases:
///   1. Walk to node before position left
///   2. Reverse the sublist from left to right
///   3. Reconnect: before -> reversed, tail of reversed -> rest
///
/// Example walkthrough for [1, 2, 3, 4, 5], left=2, right=4:
///
/// ```text
/// Phase 1: walk to node 1 (before position 2)
///     before -> [1]
///
/// Phase 2: reverse positions 2-4 (nodes 2,3,4)
///     [2, 3, 4] becomes [4, 3, 2]
///
/// Phase 3: reconnect
///     [1] -> [4, 3, 2] -> [5]
///     Result: [1, 4, 3, 2, 5]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the list
/// - Space: O(1) — pointers only
pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: usize,
    right: usize,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });

    // Phase 1: find node before reversal section
    let mut before = &mut dummy;
    for _ in 0..left - 1 {
        before = before.next.as_mut().unwrap();
    }

    // Phase 2: reverse from left to right
    let mut curr = before.next.take();
    let mut prev: Option<Box<ListNode>> = None;

    for _ in 0..=(right - left) {
        let mut node = curr.unwrap();
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    // Phase 3: reconnect
    // Find tail of reversed section and connect to remaining
    let mut tail = &mut prev;
    while tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    tail.as_mut().unwrap().next = curr;

    before.next = prev;

    dummy.next
}

/// Remove the Kth Last Node
///
/// Remove the kth node from the end of the list.
///
/// # Examples
///
/// ```text
/// Input:  1 -> 2 -> 3 -> 4 -> 5, k = 2
/// Output: 1 -> 2 -> 3 -> 5
/// (Removed 4, which is the 2nd from end)
/// ```
///
/// Two approaches:
///   1. Count length, then walk to (len - k - 1) to find node before target
///   2. Two pointers: advance fast by k, then move both until fast hits end
///
/// This implementation uses approach 1 with a dummy node.
///
/// Example walkthrough for [1, 2, 3, 4, 5], k=2:
///
/// ```text
/// len = 5
/// target is at position (5 - 2) = 3 (0-indexed)
/// walk to position 2 (node before target): node 3
/// skip: 3.next = 3.next.next (skip node 4)
/// Result: [1, 2, 3, 5]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — two passes through the list
/// - Space: O(1) — pointers only
pub fn remove_kth_from_end(head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });

    // Count length
    let mut len = 0;
    let mut curr = &dummy.next;
    while let Some(node) = curr {
        len += 1;
        curr = &node.next;
    }

    // Navigate to node BEFORE target
    let mut curr = &mut dummy;
    for _ in 0..(len - k) {
        curr = curr.next.as_mut().unwrap();
    }

    // Remove: skip over the target node
    curr.next = curr.next.take().unwrap().next;

    dummy.next
}

/// Remove Duplicates from Sorted List II
///
/// Given a sorted linked list, delete ALL nodes that have duplicate values.
/// Only keep nodes with unique values.
///
/// # Examples
///
/// ```text
/// Input:  [1, 2, 3, 3, 4, 4, 5]
/// Output: [1, 2, 5]
/// ```
///
/// Use a dummy node. For each group of nodes, check if it's a duplicate run.
/// If yes, skip the entire group. If no, keep it and advance.
///
/// Example walkthrough for [1, 2, 3, 3, 4, 4, 5]:
///
/// ```text
/// dummy -> 1 -> 2 -> 3 -> 3 -> 4 -> 4 -> 5
/// ^prev
///
/// prev=dummy: curr=1, next=2 (different) -> keep 1, prev=1
/// prev=1:     curr=2, next=3 (different) -> keep 2, prev=2
/// prev=2:     curr=3, next=3 (SAME!) -> skip all 3s -> prev.next=4
/// prev=2:     curr=4, next=4 (SAME!) -> skip all 4s -> prev.next=5
/// prev=2:     curr=5, next=None       -> keep 5, prev=5
///
/// Result: [1, 2, 5]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the list
/// - Space: O(1) — pointers only
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut prev = &mut dummy;

    while let Some(ref mut curr) = prev.next {
        // Check if this starts a duplicate run
        let mut is_dup = false;
        while curr.next.is_some() && curr.val == curr.next.as_ref().unwrap().val {
            curr.next = curr.next.take().unwrap().next;
            is_dup = true;
        }

        if is_dup {
            // Skip curr entirely (it was part of duplicates)
            prev.next = curr.next.take();
        } else {
            // Keep curr, move prev forward
            prev = prev.next.as_mut().unwrap();
        }
    }

    dummy.next
}

/// Rotate List
///
/// Rotate the list to the right by k places.
///
/// # Examples
///
/// ```text
/// Input:  head = [1, 2, 3, 4, 5], k = 2
/// Output: [4, 5, 1, 2, 3]
/// ```
///
/// Steps:
///   1. Count length, reduce k by len (k % len handles k > len)
///   2. Find new tail at position (len - k - 1)
///   3. Split: new_head = new_tail.next
///   4. Connect old tail to old head
///
/// Example walkthrough for [1, 2, 3, 4, 5], k=2:
///
/// ```text
/// len=5, k=2%5=2
/// new_tail at position (5-2-1)=2 -> node 3
/// Split: [1, 2, 3] | [4, 5]
/// Connect: [4, 5] -> [1, 2, 3]
/// Result: [4, 5, 1, 2, 3]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — count length plus walk to new tail
/// - Space: O(1) — pointers only
pub fn rotate_list(mut head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    // Count length
    let mut len = 1;
    let mut curr = head.as_ref().unwrap();
    while curr.next.is_some() {
        len += 1;
        curr = curr.next.as_ref().unwrap();
    }

    // Rotating by len gives same list, so only the remainder matters
    // e.g., k=4, len=3 → 4 % 3 = 1 (4 rotations = 1 full cycle + 1 extra)
    let k = k % len;
    if k == 0 {
        return head;
    }

    // Find new tail at position (len - k - 1)
    let mut new_tail = head.as_mut().unwrap();
    for _ in 0..(len - k - 1) {
        new_tail = new_tail.next.as_mut().unwrap();
    }

    // Split: new_tail.next becomes new head
    let mut new_head = new_tail.next.take();

    // Find tail of new_head portion and connect to old head
    let mut tail = &mut new_head;
    while tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    tail.as_mut().unwrap().next = head;

    new_head
}

/// Partition List
///
/// Rearrange so all nodes < x come before nodes >= x.
/// Preserve original relative order within each group.
///
/// # Examples
///
/// ```text
/// Input:  head = [1, 4, 3, 2, 5, 2], x = 3
/// Output: [1, 2, 2, 4, 3, 5]
/// ```
///
/// Build two separate chains: "less" and "greater or equal".
/// Then connect less_tail -> greater_head.
///
/// Example walkthrough for [1, 4, 3, 2, 5, 2], x=3:
///
/// ```text
/// 1 < 3  -> less:    [1]
/// 4 >= 3 -> greater: [4]
/// 3 >= 3 -> greater: [4, 3]
/// 2 < 3  -> less:    [1, 2]
/// 5 >= 3 -> greater: [4, 3, 5]
/// 2 < 3  -> less:    [1, 2, 2]
///
/// Connect: [1, 2, 2] -> [4, 3, 5]
/// Result: [1, 2, 2, 4, 3, 5]
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the list
/// - Space: O(1) — reuse existing nodes
pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut less_dummy = Box::new(ListNode::new(0));
    let mut greater_dummy = Box::new(ListNode::new(0));

    let mut less = &mut less_dummy;
    let mut greater = &mut greater_dummy;

    while let Some(mut node) = head {
        head = node.next.take();

        if node.val < x {
            less.next = Some(node);
            less = less.next.as_mut().unwrap();
        } else {
            greater.next = Some(node);
            greater = greater.next.as_mut().unwrap();
        }
    }

    // Connect less list to greater list
    less.next = greater_dummy.next;

    less_dummy.next
}

/// Merge Two Sorted Lists
///
/// Merge two sorted linked lists into one sorted list.
///
/// # Examples
///
/// ```text
/// Input:  l1 = 1 -> 2 -> 4, l2 = 1 -> 3 -> 4
/// Output: 1 -> 1 -> 2 -> 3 -> 4 -> 4
/// ```
///
/// Use a dummy node and compare heads of both lists.
/// Always pick the smaller value and advance that pointer.
///
/// Example walkthrough for l1=[1, 2, 4], l2=[1, 3, 4]:
///
/// ```text
/// Compare 1 vs 1 -> take l1's 1
/// Compare 2 vs 1 -> take l2's 1
/// Compare 2 vs 3 -> take l1's 2
/// Compare 4 vs 3 -> take l2's 3
/// Compare 4 vs 4 -> take l1's 4
/// l1 exhausted   -> append l2's [4]
/// Result: [1, 1, 2, 3, 4, 4]
/// ```
///
/// # Complexity
///
/// - Time: O(n + m) — single pass through both lists
/// - Space: O(1) — reuse existing nodes
pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut curr = &mut dummy;

    while list1.is_some() && list2.is_some() {
        let val1 = list1.as_ref().unwrap().val;
        let val2 = list2.as_ref().unwrap().val;

        if val1 <= val2 {
            curr.next = list1.take();
            curr = curr.next.as_mut().unwrap();
            list1 = curr.next.take();
        } else {
            curr.next = list2.take();
            curr = curr.next.as_mut().unwrap();
            list2 = curr.next.take();
        }
    }

    // Append the remaining
    curr.next = if list1.is_some() { list1 } else { list2 };

    dummy.next
}

/// LRU Cache
///
/// Design a Least Recently Used cache with get and put operations.
/// When capacity is exceeded, evict the least recently used key.
///
/// # Examples
///
/// ```text
/// Input:  cache = LRUCache(3); cache.put(1, 100); cache.put(2, 250)
///         cache.get(2)
/// Output: Some(250)
///
/// Input:  cache.put(4, 300); cache.put(3, 200)  // evicts key 1
///         cache.get(1)
/// Output: None
/// ```
///
/// Implementation: doubly linked list + hash map. The list tracks recency
/// (head = LRU, tail = most recent), and the map gives O(1) node lookup.
///
/// How it works:
/// - get(key): if exists, move node to tail (most recent), return value
/// - put(key, val): if exists, update + move to tail
///                  if new + full, remove head node (least recent)
///                  add at tail (most recent)
///
/// Example walkthrough for cap=3:
///
/// ```text
/// put(1,100): list = [1]
/// put(2,250): list = [1, 2]
/// get(2):     list = [1, 2] (2 already at tail), returns 250
/// put(4,300): list = [1, 2, 4]
/// put(3,200): full! evict head (key 1), list = [2, 4, 3]
/// get(1):     not in map, returns None
/// ```
///
/// # Complexity
///
/// - Time: O(1) for both get and put
/// - Space: O(capacity) — one node and one map entry per key
struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>, // dummy head
    tail: Option<Rc<RefCell<Node>>>, // dummy tail
}

struct Node {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        let head = Rc::new(RefCell::new(Node {
            key: -1,
            val: -1,
            prev: None,
            next: None,
        }));
        let tail = Rc::new(RefCell::new(Node {
            key: -1,
            val: -1,
            prev: None,
            next: None,
        }));

        // Link head <-> tail
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));

        LRUCache {
            capacity,
            map: HashMap::new(),
            head: Some(head),
            tail: Some(tail),
        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(node) = self.map.get(&key).cloned() {
            let val = node.borrow().val;

            self.remove_node(&node);
            self.add_node(&node);

            return Some(val);
        }

        None
    }

    pub fn put(&mut self, key: i32, val: i32) {
        let node = Rc::new(RefCell::new(Node {
            key,
            val,
            prev: None,
            next: None,
        }));

        if !self.map.contains_key(&key) && self.map.len() >= self.capacity {
            let least_node = self
                .head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .map(|n| Rc::clone(n))
                .unwrap();

            self.map.remove(&least_node.borrow().key);
            self.remove_node(&least_node);
        }

        self.map.insert(key, Rc::clone(&node));
        self.add_node(&node);
    }

    fn add_node(&mut self, node: &Rc<RefCell<Node>>) {
        let prev_node = self
            .tail
            .as_ref()
            .unwrap()
            .borrow()
            .prev
            .as_ref()
            .map(|n| Rc::clone(n));

        // Link prev -> node
        if let Some(prev) = &prev_node {
            prev.borrow_mut().next = Some(Rc::clone(node));
            node.borrow_mut().prev = Some(Rc::clone(&prev));
        }

        // Link tail -> node
        if let Some(tail) = &self.tail {
            node.borrow_mut().next = Some(Rc::clone(&tail));
            tail.borrow_mut().prev = Some(Rc::clone(&node));
        }
    }

    fn remove_node(&mut self, node: &Rc<RefCell<Node>>) {
        // Get prev and next of the node to remove
        let prev_node = node.borrow().prev.as_ref().map(|n| Rc::clone(n));
        let next_node = node.borrow().next.as_ref().map(|n| Rc::clone(n));

        // Link prev -> next
        if let Some(prev) = &prev_node {
            prev.borrow_mut().next = next_node.clone();
        }

        // Link next -> prev
        if let Some(next) = &next_node {
            next.borrow_mut().prev = prev_node;
        }
    }
}

/// Add Two Numbers
///
/// Two linked lists store digits in reverse order.
/// Add the numbers and return the sum as a linked list.
///
/// # Examples
///
/// ```text
/// Input:  l1 = [2, 4, 3], l2 = [5, 6, 4]
/// Output: [7, 0, 8]
/// (342 + 465 = 807, stored as 7 -> 0 -> 8)
/// ```
///
/// Walk both lists simultaneously, adding digits + carry.
/// Like grade school addition, right to left (but lists are already reversed).
///
/// Example walkthrough for [2,4,3] + [5,6,4]:
///
/// ```text
/// 2+5+0 = 7,  carry=0 -> 7
/// 4+6+0 = 10, carry=1 -> 0
/// 3+4+1 = 8,  carry=0 -> 8
/// Result: [7, 0, 8]
/// ```
///
/// Example walkthrough for [9,9] + [1]:
///
/// ```text
/// 9+1+0 = 10, carry=1 -> 0
/// 9+0+1 = 10, carry=1 -> 0
/// 0+0+1 = 1,  carry=0 -> 1
/// Result: [0, 0, 1]  (99 + 1 = 100)
/// ```
///
/// # Complexity
///
/// - Time: O(max(m, n)) — single pass through the longer list
/// - Space: O(max(m, n)) — new list of digits
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut curr = &mut dummy;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let val1 = l1.as_ref().map_or(0, |n| n.val);
        let val2 = l2.as_ref().map_or(0, |n| n.val);

        let sum = val1 + val2 + carry;
        carry = sum / 10;

        curr.next = Some(Box::new(ListNode::new(sum % 10)));
        curr = curr.next.as_mut().unwrap();

        l1 = l1.and_then(|n| n.next);
        l2 = l2.and_then(|n| n.next);
    }

    dummy.next
}

/// Node with random pointer for Copy List problem
#[derive(Debug)]
pub struct RandomNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<RandomNode>>>,
    pub random: Option<Rc<RefCell<RandomNode>>>,
}

impl RandomNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(RandomNode {
            val,
            next: None,
            random: None,
        }))
    }
}

/// Copy List with Random Pointer
///
/// Each node has val, next, and a random pointer (can point to any node or None).
/// Create a deep copy of the list.
///
/// # Examples
///
/// ```text
/// Input:  A(random->C) -> B(random->A) -> C(random->B)
/// Output: deep copy with same structure, no shared nodes
/// ```
///
/// Two-pass approach: first create all new nodes and build a map from old to
/// new, then wire up next and random pointers using the map. This avoids the
/// chicken-and-egg problem of needing nodes to exist before you can point to
/// them.
///
/// Example walkthrough for A(random->C) -> B(random->A) -> C(random->B):
///
/// ```text
/// Pass 1: create A', B', C'
///         map = {A: A', B: B', C: C'}
/// Pass 2: A'.next = map[A.next] = B'
///         A'.random = map[A.random] = C'
///         B'.next = C', B'.random = A'
///         C'.next = None, C'.random = B'
/// ```
///
/// # Complexity
///
/// - Time: O(n) — two passes through the list
/// - Space: O(n) — hash map of old->new nodes
pub fn copy_random_list(head: Option<Rc<RefCell<RandomNode>>>) -> Option<Rc<RefCell<RandomNode>>> {
    if head.is_none() {
        return None;
    }

    // Map from original node address to cloned node
    let mut map: HashMap<*const RefCell<RandomNode>, Rc<RefCell<RandomNode>>> = HashMap::new();

    // First pass: create all nodes and populate map
    let mut curr = head.clone();
    while let Some(node) = curr {
        let clone = RandomNode::new(node.borrow().val);
        map.insert(Rc::as_ptr(&node), clone);
        curr = node.borrow().next.clone();
    }

    // Second pass: wire up next and random pointers
    curr = head.clone();
    while let Some(node) = curr {
        let node_ptr = Rc::as_ptr(&node);
        let clone = map.get(&node_ptr).unwrap();

        // Set next
        if let Some(next) = node.borrow().next.as_ref() {
            let next_clone = map.get(&Rc::as_ptr(next)).unwrap();
            clone.borrow_mut().next = Some(Rc::clone(next_clone));
        }

        // Set random
        if let Some(random) = node.borrow().random.as_ref() {
            let random_clone = map.get(&Rc::as_ptr(random)).unwrap();
            clone.borrow_mut().random = Some(Rc::clone(random_clone));
        }

        curr = node.borrow().next.clone();
    }

    map.get(&Rc::as_ptr(&head.unwrap())).cloned()
}

// =============================================================================
// Fast and Slow Pointers
// =============================================================================

/// Node for linked list with potential cycles.
/// Using Rc<RefCell<>> to allow multiple references (needed for cycles).
#[derive(Debug)]
pub struct CycleNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<CycleNode>>>,
}

impl CycleNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CycleNode { val, next: None }))
    }
}

/// Linked List Loop
///
/// Determine if a linked list contains a cycle.
///
/// # Examples
///
/// ```text
/// Input:  1 -> 2 -> 3 -> 4
///              ^         |
///              |_________|
/// Output: true (node 4 points back to node 2)
/// ```
///
/// Floyd's Cycle Detection:
/// - slow moves 1 step, fast moves 2 steps
/// - If no cycle: fast reaches None -> return False
/// - If cycle: fast gains 1 step per iteration on slow
///   gap closes: k -> k-1 -> k-2 -> ... -> 0 (they meet!)
///
/// Analogy: two runners on a circular track. The faster one always laps the
/// slower one.
///
/// Example walkthrough for 1 -> 2 -> 3 -> 4 -> (back to 2):
///
/// ```text
/// slow=1, fast=1
/// Step 1: slow=2, fast=3
/// Step 2: slow=3, fast=3 (4->2, then 2->3)...
/// Actually: fast moves 2 steps: 3->4, 4->2 -> fast=2
/// Step 2: slow=3, fast=2
/// Step 3: slow=4, fast=4 -> MATCH! cycle detected
/// ```
///
/// # Complexity
///
/// - Time: O(n) — at most 2 full traversals
/// - Space: O(1) — just two pointers
pub fn has_cycle(head: Option<Rc<RefCell<CycleNode>>>) -> bool {
    if head.is_none() {
        return false;
    }

    let mut slow = head.clone();
    let mut fast = head.clone();

    while let Some(fast_node) = fast {
        // Move fast one step
        let next = fast_node.borrow().next.clone();

        // Check if fast can move another step
        if let Some(fast_node_next) = next {
            fast = fast_node_next.borrow().next.clone();
        } else {
            // End of linked list, there is no cycle
            return false;
        }

        // Move slow one step
        slow = slow.unwrap().borrow().next.clone();

        // Check if they meet
        if let (Some(s), Some(f)) = (&slow, &fast) {
            if s.borrow().val == f.borrow().val {
                return true;
            }
        }
    }

    false
}

/// Linked List Midpoint
///
/// Find the middle node. If two middles, return the second one.
///
/// # Examples
///
/// ```text
/// Input:  1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7
/// Output: 4
///
/// Input:  1 -> 2 -> 3 -> 4 -> 5 -> 6
/// Output: 4 (second of the two middles)
/// ```
///
/// When fast reaches the end, slow is at the middle. Fast moves 2x speed,
/// so when fast travels n steps, slow travels n/2.
///
/// Example walkthrough for [1, 2, 3, 4, 5]:
///
/// ```text
/// slow=1, fast=1
/// Step 1: slow=2, fast=3
/// Step 2: slow=3, fast=5
/// fast.next is None -> stop. slow=3 ✓
/// ```
///
/// Example walkthrough for [1, 2, 3, 4, 5, 6]:
///
/// ```text
/// slow=1, fast=1
/// Step 1: slow=2, fast=3
/// Step 2: slow=3, fast=5
/// Step 3: slow=4, fast=None (5.next=6, 6.next=None)
/// fast is None -> stop. slow=4 ✓ (second middle)
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the list
/// - Space: O(1) — two pointers
pub fn find_middle(head: Option<Rc<RefCell<CycleNode>>>) -> Option<Rc<RefCell<CycleNode>>> {
    let mut slow = head.clone();
    let mut fast = head.clone();

    while let Some(fast_node) = fast {
        if let Some(fast_node_next) = fast_node.borrow().next.clone() {
            fast = fast_node_next.borrow().next.clone();
        } else {
            break;
        }
        slow = slow.unwrap().borrow().next.clone();
    }

    slow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list_basic() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let reversed = reverse_list(head);
        assert_eq!(ListNode::to_vec(&reversed), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_list_two_elements() {
        let head = ListNode::from_vec(vec![1, 2]);
        let reversed = reverse_list(head);
        assert_eq!(ListNode::to_vec(&reversed), vec![2, 1]);
    }

    #[test]
    fn test_reverse_list_single_element() {
        let head = ListNode::from_vec(vec![42]);
        let reversed = reverse_list(head);
        assert_eq!(ListNode::to_vec(&reversed), vec![42]);
    }

    #[test]
    fn test_reverse_list_empty() {
        let head: Option<Box<ListNode>> = None;
        let reversed = reverse_list(head);
        assert_eq!(ListNode::to_vec(&reversed), vec![]);
    }

    #[test]
    fn test_reverse_list_negative_numbers() {
        let head = ListNode::from_vec(vec![-1, -2, -3]);
        let reversed = reverse_list(head);
        assert_eq!(ListNode::to_vec(&reversed), vec![-3, -2, -1]);
    }

    #[test]
    fn test_reverse_list_duplicates() {
        let head = ListNode::from_vec(vec![1, 1, 2, 2, 3]);
        let reversed = reverse_list(head);
        assert_eq!(ListNode::to_vec(&reversed), vec![3, 2, 2, 1, 1]);
    }

    #[test]
    fn test_reverse_list_large() {
        let values: Vec<i32> = (1..=100).collect();
        let expected: Vec<i32> = (1..=100).rev().collect();
        let head = ListNode::from_vec(values);
        let reversed = reverse_list(head);
        assert_eq!(ListNode::to_vec(&reversed), expected);
    }

    #[test]
    fn test_remove_kth_from_end_middle() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = remove_kth_from_end(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_remove_kth_from_end_last() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = remove_kth_from_end(head, 1);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_kth_from_end_first() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = remove_kth_from_end(head, 5);
        assert_eq!(ListNode::to_vec(&result), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_kth_from_end_single_element() {
        let head = ListNode::from_vec(vec![1]);
        let result = remove_kth_from_end(head, 1);
        assert_eq!(ListNode::to_vec(&result), vec![]);
    }

    #[test]
    fn test_remove_kth_from_end_two_elements_remove_first() {
        let head = ListNode::from_vec(vec![1, 2]);
        let result = remove_kth_from_end(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![2]);
    }

    #[test]
    fn test_remove_kth_from_end_two_elements_remove_last() {
        let head = ListNode::from_vec(vec![1, 2]);
        let result = remove_kth_from_end(head, 1);
        assert_eq!(ListNode::to_vec(&result), vec![1]);
    }

    #[test]
    fn test_merge_two_lists_example() {
        let list1 = ListNode::from_vec(vec![1, 2, 4]);
        let list2 = ListNode::from_vec(vec![1, 3, 4]);
        let result = merge_two_lists(list1, list2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_two_lists_both_empty() {
        let result = merge_two_lists(None, None);
        assert_eq!(ListNode::to_vec(&result), vec![]);
    }

    #[test]
    fn test_merge_two_lists_one_empty() {
        let list1 = ListNode::from_vec(vec![1, 2, 3]);
        let result = merge_two_lists(list1, None);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_two_lists_other_empty() {
        let list2 = ListNode::from_vec(vec![4, 5, 6]);
        let result = merge_two_lists(None, list2);
        assert_eq!(ListNode::to_vec(&result), vec![4, 5, 6]);
    }

    #[test]
    fn test_merge_two_lists_different_lengths() {
        let list1 = ListNode::from_vec(vec![1, 2]);
        let list2 = ListNode::from_vec(vec![3, 4, 5, 6]);
        let result = merge_two_lists(list1, list2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_lru_cache_example() {
        let mut cache = LRUCache::new(3);
        cache.put(1, 100);
        cache.put(2, 250);
        assert_eq!(cache.get(2), Some(250));
        cache.put(4, 300);
        cache.put(3, 200); // evicts key 1
        assert_eq!(cache.get(4), Some(300));
        assert_eq!(cache.get(1), None); // was evicted
    }

    #[test]
    fn test_lru_cache_update_existing() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 100);
        cache.put(1, 200); // update value
        assert_eq!(cache.get(1), Some(200));
    }

    #[test]
    fn test_lru_cache_get_updates_recency() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 100);
        cache.put(2, 200);
        cache.get(1); // makes key 1 most recently used
        cache.put(3, 300); // should evict key 2, not key 1
        assert_eq!(cache.get(1), Some(100));
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(300));
    }

    #[test]
    fn test_lru_cache_capacity_one() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 100);
        cache.put(2, 200); // evicts key 1
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(200));
    }

    #[test]
    fn test_lru_cache_get_nonexistent() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(1), None);
        cache.put(1, 100);
        assert_eq!(cache.get(2), None);
    }

    #[test]
    fn test_add_two_numbers_example1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_example2() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![0]);
    }

    #[test]
    fn test_add_two_numbers_example3() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn test_add_two_numbers_different_lengths() {
        let l1 = ListNode::from_vec(vec![1, 2, 3]);
        let l2 = ListNode::from_vec(vec![4, 5]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![5, 7, 3]); // 321 + 54 = 375
    }

    #[test]
    fn test_add_two_numbers_carry_propagates() {
        let l1 = ListNode::from_vec(vec![9, 9]);
        let l2 = ListNode::from_vec(vec![1]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![0, 0, 1]); // 99 + 1 = 100
    }

    #[test]
    fn test_copy_random_list_empty() {
        let result = copy_random_list(None);
        assert!(result.is_none());
    }

    #[test]
    fn test_copy_random_list_single() {
        let node = RandomNode::new(7);
        let result = copy_random_list(Some(node.clone()));

        assert!(result.is_some());
        let copied = result.unwrap();
        assert_eq!(copied.borrow().val, 7);
        // Ensure it's a different node (deep copy)
        assert!(!Rc::ptr_eq(&node, &copied));
    }

    #[test]
    fn test_copy_random_list_with_random() {
        // Create: 1 -> 2, where 1.random = 2 and 2.random = 2 (self)
        let node1 = RandomNode::new(1);
        let node2 = RandomNode::new(2);

        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node1.borrow_mut().random = Some(Rc::clone(&node2));
        node2.borrow_mut().random = Some(Rc::clone(&node2));

        let result = copy_random_list(Some(Rc::clone(&node1)));

        let copied1 = result.unwrap();
        assert_eq!(copied1.borrow().val, 1);

        let copied2 = copied1.borrow().next.clone().unwrap();
        assert_eq!(copied2.borrow().val, 2);

        // Check random pointers point to copied nodes, not originals
        let copied1_random = copied1.borrow().random.clone().unwrap();
        assert!(Rc::ptr_eq(&copied1_random, &copied2));

        let copied2_random = copied2.borrow().random.clone().unwrap();
        assert!(Rc::ptr_eq(&copied2_random, &copied2));
    }

    #[test]
    fn test_reverse_between_example1() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = reverse_between(head, 2, 4);
        assert_eq!(ListNode::to_vec(&result), vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn test_reverse_between_example2() {
        let head = ListNode::from_vec(vec![5]);
        let result = reverse_between(head, 1, 1);
        assert_eq!(ListNode::to_vec(&result), vec![5]);
    }

    #[test]
    fn test_reverse_between_full_list() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = reverse_between(head, 1, 5);
        assert_eq!(ListNode::to_vec(&result), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_between_first_two() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = reverse_between(head, 1, 2);
        assert_eq!(ListNode::to_vec(&result), vec![2, 1, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_between_last_two() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = reverse_between(head, 4, 5);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3, 5, 4]);
    }

    #[test]
    fn test_reverse_between_two_elements() {
        let head = ListNode::from_vec(vec![3, 5]);
        let result = reverse_between(head, 1, 2);
        assert_eq!(ListNode::to_vec(&result), vec![5, 3]);
    }

    #[test]
    fn test_delete_duplicates_example1() {
        let head = ListNode::from_vec(vec![1, 2, 3, 3, 4, 4, 5]);
        let result = delete_duplicates(head);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 5]);
    }

    #[test]
    fn test_delete_duplicates_example2() {
        let head = ListNode::from_vec(vec![1, 1, 1, 2, 3]);
        let result = delete_duplicates(head);
        assert_eq!(ListNode::to_vec(&result), vec![2, 3]);
    }

    #[test]
    fn test_delete_duplicates_all_same() {
        let head = ListNode::from_vec(vec![1, 1, 1]);
        let result = delete_duplicates(head);
        assert_eq!(ListNode::to_vec(&result), vec![]);
    }

    #[test]
    fn test_delete_duplicates_no_duplicates() {
        let head = ListNode::from_vec(vec![1, 2, 3]);
        let result = delete_duplicates(head);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn test_delete_duplicates_empty() {
        let result = delete_duplicates(None);
        assert_eq!(ListNode::to_vec(&result), vec![]);
    }

    #[test]
    fn test_delete_duplicates_single() {
        let head = ListNode::from_vec(vec![1]);
        let result = delete_duplicates(head);
        assert_eq!(ListNode::to_vec(&result), vec![1]);
    }

    #[test]
    fn test_rotate_list_example1() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = rotate_list(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_rotate_list_example2() {
        let head = ListNode::from_vec(vec![0, 1, 2]);
        let result = rotate_list(head, 4);
        assert_eq!(ListNode::to_vec(&result), vec![2, 0, 1]);
    }

    #[test]
    fn test_rotate_list_empty() {
        let result = rotate_list(None, 5);
        assert_eq!(ListNode::to_vec(&result), vec![]);
    }

    #[test]
    fn test_rotate_list_single() {
        let head = ListNode::from_vec(vec![1]);
        let result = rotate_list(head, 3);
        assert_eq!(ListNode::to_vec(&result), vec![1]);
    }

    #[test]
    fn test_rotate_list_k_zero() {
        let head = ListNode::from_vec(vec![1, 2, 3]);
        let result = rotate_list(head, 0);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_list_k_equals_len() {
        let head = ListNode::from_vec(vec![1, 2, 3]);
        let result = rotate_list(head, 3);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn test_partition_example1() {
        let head = ListNode::from_vec(vec![1, 4, 3, 2, 5, 2]);
        let result = partition(head, 3);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 2, 4, 3, 5]);
    }

    #[test]
    fn test_partition_example2() {
        let head = ListNode::from_vec(vec![2, 1]);
        let result = partition(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2]);
    }

    #[test]
    fn test_partition_empty() {
        let result = partition(None, 5);
        assert_eq!(ListNode::to_vec(&result), vec![]);
    }

    #[test]
    fn test_partition_all_less() {
        let head = ListNode::from_vec(vec![1, 2, 3]);
        let result = partition(head, 5);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn test_partition_all_greater() {
        let head = ListNode::from_vec(vec![5, 6, 7]);
        let result = partition(head, 3);
        assert_eq!(ListNode::to_vec(&result), vec![5, 6, 7]);
    }

    #[test]
    fn test_partition_single() {
        let head = ListNode::from_vec(vec![1]);
        let result = partition(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![1]);
    }

    // -------------------------------------------------------------------------
    // Fast and Slow Pointers tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_has_cycle_with_cycle() {
        let node1 = CycleNode::new(1);
        let node2 = CycleNode::new(2);
        let node3 = CycleNode::new(3);
        let node4 = CycleNode::new(4);

        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node3));
        node3.borrow_mut().next = Some(Rc::clone(&node4));
        node4.borrow_mut().next = Some(Rc::clone(&node2)); // cycle back to node2

        assert!(has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_no_cycle() {
        let node1 = CycleNode::new(1);
        let node2 = CycleNode::new(2);
        let node3 = CycleNode::new(3);

        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node3));

        assert!(!has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_single_node_no_cycle() {
        let node1 = CycleNode::new(1);
        assert!(!has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_single_node_self_loop() {
        let node1 = CycleNode::new(1);
        node1.borrow_mut().next = Some(Rc::clone(&node1)); // self loop

        assert!(has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_empty_list() {
        assert!(!has_cycle(None));
    }

    #[test]
    fn test_has_cycle_two_nodes_with_cycle() {
        let node1 = CycleNode::new(1);
        let node2 = CycleNode::new(2);

        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node1)); // cycle back to node1

        assert!(has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_two_nodes_no_cycle() {
        let node1 = CycleNode::new(1);
        let node2 = CycleNode::new(2);

        node1.borrow_mut().next = Some(Rc::clone(&node2));

        assert!(!has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_long_list_cycle_at_end() {
        let nodes: Vec<Rc<RefCell<CycleNode>>> = (1..=5).map(|i| CycleNode::new(i)).collect();

        for i in 0..4 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        nodes[4].borrow_mut().next = Some(Rc::clone(&nodes[2])); // cycle to node 3

        assert!(has_cycle(Some(Rc::clone(&nodes[0]))));
    }

    #[test]
    fn test_find_middle_odd_length() {
        let nodes: Vec<Rc<RefCell<CycleNode>>> = (1..=7).map(|i| CycleNode::new(i)).collect();
        for i in 0..6 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        let middle = find_middle(Some(Rc::clone(&nodes[0])));
        assert_eq!(middle.unwrap().borrow().val, 4);
    }

    #[test]
    fn test_find_middle_even_length() {
        let nodes: Vec<Rc<RefCell<CycleNode>>> = (1..=6).map(|i| CycleNode::new(i)).collect();
        for i in 0..5 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        let middle = find_middle(Some(Rc::clone(&nodes[0])));
        assert_eq!(middle.unwrap().borrow().val, 4);
    }

    #[test]
    fn test_find_middle_single_node() {
        let node = CycleNode::new(42);
        let middle = find_middle(Some(node));
        assert_eq!(middle.unwrap().borrow().val, 42);
    }

    #[test]
    fn test_find_middle_two_nodes() {
        let node1 = CycleNode::new(1);
        let node2 = CycleNode::new(2);
        node1.borrow_mut().next = Some(Rc::clone(&node2));
        let middle = find_middle(Some(node1));
        assert_eq!(middle.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_find_middle_three_nodes() {
        let node1 = CycleNode::new(1);
        let node2 = CycleNode::new(2);
        let node3 = CycleNode::new(3);
        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node3));
        let middle = find_middle(Some(node1));
        assert_eq!(middle.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_find_middle_four_nodes() {
        let nodes: Vec<Rc<RefCell<CycleNode>>> = (1..=4).map(|i| CycleNode::new(i)).collect();
        for i in 0..3 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        let middle = find_middle(Some(Rc::clone(&nodes[0])));
        assert_eq!(middle.unwrap().borrow().val, 3);
    }
}
