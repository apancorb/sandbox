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
pub fn remove_kth_from_end(mut head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut len = 0;
    let mut curr = &head;
    while let Some(node) = curr {
        len += 1;
        curr = &node.next;
    }

    // Special case: removing the first node
    if k == len {
        return head.unwrap().next;
    }

    let target = len - k - 1;
    let mut curr = &mut head;
    for _ in 0..target {
        curr = &mut curr.as_mut().unwrap().next;
    }

    let target_node = curr.as_mut().unwrap().next.take();
    curr.as_mut().unwrap().next = target_node.unwrap().next;

    head
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
/// ```
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
            prev.as_ref().borrow_mut().next = Some(Rc::clone(node));
            node.borrow_mut().prev = Some(Rc::clone(&prev));
        }

        // Link tail -> node
        if let Some(tail) = &self.tail {
            node.as_ref().borrow_mut().next = Some(Rc::clone(&tail));
            tail.as_ref().borrow_mut().prev = Some(Rc::clone(&node));
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
}
