use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

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
/// # Example
///
/// ```text
/// Input: 1 -> 2 -> 3 -> 4 -> 5 -> None
/// Output: 5 -> 4 -> 3 -> 2 -> 1 -> None
/// ```
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

/// Remove the Kth Last Node From a Linked List
///
/// Return the head of a singly linked list after removing the kth node from the end of it.
///
/// # Example
///
/// ```text
/// Input: 1 -> 2 -> 3 -> 4 -> 5, k = 2
/// Output: 1 -> 2 -> 3 -> 5
/// Explanation: Removed the 2nd node from the end (which is 4)
/// ```
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

/// Merge Two Sorted Lists
///
/// Merge two sorted linked lists and return it as a new sorted list.
///
/// # Example
///
/// ```text
/// Input: 1 -> 2 -> 4, 1 -> 3 -> 4
/// Output: 1 -> 1 -> 2 -> 3 -> 4 -> 4
/// ```
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

/// Linked List Intersection
///
/// Return the value of the node where two singly linked lists intersect. If the linked lists
/// don't intersect, return None. Intersection is determined by pointer address (same memory location).
///
/// # Example
///
/// ```text
/// List A: 1 -> 2 -> 3 \
///                      -> 6 -> 7 -> None
/// List B:      4 -> 5 /
///
/// Output: 6
/// ```
pub fn linked_list_intersection(
    head_a: &Option<Box<ListNode>>,
    head_b: &Option<Box<ListNode>>,
) -> Option<i32> {
    let mut seen = HashSet::<i32>::new();
    let mut curr = head_a;

    while let Some(node) = curr {
        seen.insert(node.val);
        curr = &node.next;
    }

    curr = head_b;
    while let Some(node) = curr {
        if seen.contains(&node.val) {
            return Some(node.val);
        }
        curr = &node.next;
    }

    None
}

/// LRU Cache
///
/// Design and implement a data structure for the Least Recently Used (LRU) cache that
/// supports the following operations:
///
/// - `LRUCache::new(capacity)`: Initialize an LRU cache with the specified capacity.
/// - `get(key) -> Option<i32>`: Return the value associated with a key. Return None if the key doesn't exist.
/// - `put(key, value)`: Add a key and its value to the cache. If adding the key would result in
///   the cache exceeding its capacity, evict the least recently used element. If the key already
///   exists in the cache, update its value.
///
/// # Example
///
/// ```text
/// let mut cache = LRUCache::new(3);
/// cache.put(1, 100);  // cache is [1: 100]
/// cache.put(2, 250);  // cache is [1: 100, 2: 250]
/// cache.get(2);       // returns Some(250)
/// cache.put(4, 300);  // cache is [1: 100, 2: 250, 4: 300]
/// cache.put(3, 200);  // cache is [2: 250, 4: 300, 3: 200], evicts key 1
/// cache.get(4);       // returns Some(300)
/// cache.get(1);       // returns None (was evicted)
/// ```
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
/// Given two non-empty linked lists representing two non-negative integers,
/// where digits are stored in reverse order, add the two numbers and return
/// the sum as a linked list.
///
/// # Example 1
///
/// ```text
/// Input: l1 = [2, 4, 3], l2 = [5, 6, 4]
/// Output: [7, 0, 8]
/// Explanation: 342 + 465 = 807
/// ```
///
/// # Example 2
///
/// ```text
/// Input: l1 = [9, 9, 9, 9, 9, 9, 9], l2 = [9, 9, 9, 9]
/// Output: [8, 9, 9, 9, 0, 0, 0, 1]
/// ```
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
/// Given a linked list where each node has a `next` pointer and a `random` pointer
/// (which can point to any node or null), create a deep copy of the list.
///
/// # Example
///
/// ```text
/// Input: [[7,null],[13,0],[11,4],[10,2],[1,0]]
/// Output: [[7,null],[13,0],[11,4],[10,2],[1,0]]
/// ```
// Uses HashMap to map original nodes to cloned nodes.
// Two passes: 1) create all nodes, 2) wire up next and random pointers.
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
    fn test_linked_list_intersection_no_intersection() {
        let head_a = ListNode::from_vec(vec![1, 2, 3]);
        let head_b = ListNode::from_vec(vec![4, 5, 6]);
        let result = linked_list_intersection(&head_a, &head_b);
        assert!(result.is_none());
    }

    #[test]
    fn test_linked_list_intersection_both_empty() {
        let head_a: Option<Box<ListNode>> = None;
        let head_b: Option<Box<ListNode>> = None;
        let result = linked_list_intersection(&head_a, &head_b);
        assert!(result.is_none());
    }

    #[test]
    fn test_linked_list_intersection_one_empty() {
        let head_a = ListNode::from_vec(vec![1, 2, 3]);
        let head_b: Option<Box<ListNode>> = None;
        let result = linked_list_intersection(&head_a, &head_b);
        assert!(result.is_none());
    }

    #[test]
    fn test_linked_list_intersection_same_list() {
        // When both pointers point to the same list, intersection is at head
        let head = ListNode::from_vec(vec![1, 2, 3]);
        let result = linked_list_intersection(&head, &head);
        assert_eq!(result, Some(1));
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
}
