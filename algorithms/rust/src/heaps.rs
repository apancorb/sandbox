use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

pub type ListNode = Option<Box<Node>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: i32,
    pub next: ListNode,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Kth Largest Integer
///
/// Return the kth largest integer in an array.
///
/// # Examples
///
/// ```text
/// Input: nums = [5, 2, 4, 3, 1, 6], k = 3
/// Output: 4
/// // Sorted desc: [6, 5, 4, 3, 2, 1] -> 3rd largest is 4
/// ```
///
/// Keep a min heap of size k (the "top k" bucket).
/// The smallest in the bucket = kth largest overall.
///
/// Example walkthrough for k=3 on [5, 2, 4, 3, 1, 6]:
///
/// ```text
/// 5 -> bucket: [5]           not full yet
/// 2 -> bucket: [2, 5]        not full yet
/// 4 -> bucket: [2, 4, 5]     full! (size k=3)
/// 3 -> 3 > min(2)? yes -> kick 2, add 3 -> bucket: [3, 4, 5]
/// 1 -> 1 > min(3)? no -> skip
/// 6 -> 6 > min(3)? yes -> kick 3, add 6 -> bucket: [4, 5, 6]
/// Answer: min of bucket = 4
/// ```
///
/// # Complexity
///
/// - Time: O(n log k) - heap operations
/// - Space: O(k) - heap size
pub fn kth_largest(nums: &[i32], k: usize) -> i32 {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for &num in nums {
        if min_heap.len() < k {
            min_heap.push(Reverse(num));
        } else if num > min_heap.peek().unwrap().0 {
            min_heap.pop();
            min_heap.push(Reverse(num));
        }
    }

    min_heap.peek().unwrap().0
}

/// Combine K Sorted Lists
///
/// Given k sorted lists, combine them into one sorted list.
///
/// # Examples
///
/// ```text
/// Input: lists = [[1, 3, 5], [2, 4, 6], [0, 7, 8]]
/// Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]
/// ```
///
/// Push first element of each list into min heap.
/// Pop smallest, push the next element from that same list.
/// Heap entries are (value, list_index, element_index) to break ties.
///
/// Example walkthrough for [[1,3,5], [2,4,6], [0,7,8]]:
///
/// ```text
/// Init heap: [(1,0,0), (2,1,0), (0,2,0)]
/// Pop (0,2,0) -> result=[0], push (7,2,1)
/// Pop (1,0,0) -> result=[0,1], push (3,0,1)
/// Pop (2,1,0) -> result=[0,1,2], push (4,1,1)
/// Pop (3,0,1) -> result=[0,1,2,3], push (5,0,2)
/// Pop (4,1,1) -> result=[0,1,2,3,4], push (6,1,2)
/// Pop (5,0,2) -> result=[0,1,2,3,4,5], list 0 exhausted
/// Pop (6,1,2) -> result=[0,1,2,3,4,5,6], list 1 exhausted
/// Pop (7,2,1) -> result=[0,1,2,3,4,5,6,7], push (8,2,2)
/// Pop (8,2,2) -> result=[0,1,2,3,4,5,6,7,8], list 2 exhausted
/// ```
///
/// # Complexity
///
/// - Time: O(n log k) - n total elements, k lists
/// - Space: O(k) - heap holds one element per list
pub fn combine_sorted_lists(lists: Vec<ListNode>) -> ListNode {
    if lists.is_empty() {
        return None;
    }

    let mut heap = BinaryHeap::new();

    for list in lists {
        if let Some(node) = list {
            heap.push(node);
        }
    }

    let mut dummy = Node::new(-1);
    let mut curr = &mut dummy;

    while let Some(mut node) = heap.pop() {
        if let Some(next) = node.next.take() {
            heap.push(next);
        }

        curr.next = Some(node);
        curr = curr.next.as_mut().unwrap();
    }

    dummy.next
}

#[derive(Eq, PartialEq)]
struct Pair(String, usize);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).then_with(|| other.0.cmp(&self.0))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Median of an Integer Stream
///
/// Design a data structure that supports:
///     - add(num): adds an integer
///     - get_median(): returns the median of all integers so far
///
/// # Examples
///
/// ```text
/// Input: [add(3), add(6), get_median(), add(1), get_median()]
/// Output: [4.5, 3.0]
/// ```
///
/// Two heaps split the data into smaller and larger halves:
///
/// ```text
/// left (max heap): smaller half    right (min heap): larger half
///
/// Example after adding [1, 3, 6]:
///     left: [1, 3]   right: [6]
///     max=3           min=6
///
/// The median is always at the boundary between the two heaps!
///
/// Rules:
///     - left.size == right.size  OR  left.size == right.size + 1
///     - Everything in left <= everything in right
///     - Median = top of left (odd count), avg of both tops (even)
/// ```
///
/// Example walkthrough: add(3), add(6), add(1)
///
/// ```text
/// add(3): left=[-3], right=[]          -> left has 3
/// add(6): 6 > 3, goes right            -> left=[-3], right=[6]
/// add(1): 1 <= max(left)=3 -> push to left
///         left=[-3,-1] (max heap: 3,1), right=[6]
///         left.size(2) - right.size(1) = 1, ok no rebalance
///         median = top of left = 3 ✓
/// ```
///
/// Python only has min heap, so left uses NEGATED values for max heap.
/// push -5 into min heap -> acts like pushing 5 into max heap.
///
/// # Complexity
///
/// - Time: O(log n) per add, O(1) per get_median
/// - Space: O(n)
struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, num: i32) {
        if self.left.is_empty() || *self.left.peek().unwrap() >= num {
            self.left.push(num);
            if self.left.len() - self.right.len() > 1 {
                self.right.push(Reverse(self.left.pop().unwrap()));
            }
        } else {
            self.right.push(Reverse(num));
            if self.right.len() - self.left.len() > 0 {
                self.left.push(self.right.pop().unwrap().0);
            }
        }
    }

    pub fn get_median(&self) -> f64 {
        if self.left.len() == self.right.len() {
            (self.left.peek().unwrap() + self.right.peek().unwrap().0) as f64 / 2.0
        } else {
            *self.left.peek().unwrap() as f64
        }
    }
}

/// K Most Frequent Strings
///
/// Find the k most frequent strings. Sort by frequency (desc),
/// then lexicographically for ties.
///
/// # Examples
///
/// ```text
/// Input: strs = ["go","coding","byte","byte","go","interview","go"], k = 2
/// Output: ["go", "byte"]
/// // go: 3 times, byte: 2 times
/// ```
///
/// Count frequencies with a Counter, then use a min heap with negated
/// counts so the highest frequency pops first. For ties, the tuple
/// comparison naturally falls through to lexicographic order on the name.
///
/// Example walkthrough for ["go","coding","byte","byte","go","interview","go"], k=2:
///
/// ```text
/// Counter: {"go": 3, "byte": 2, "coding": 1, "interview": 1}
/// Heap entries: [(-3,"go"), (-2,"byte"), (-1,"coding"), (-1,"interview")]
/// Pop 1: (-3,"go")    -> "go"
/// Pop 2: (-2,"byte")  -> "byte"
/// Result: ["go", "byte"]
/// ```
///
/// # Complexity
///
/// - Time: O(n log n) - heap/sort
/// - Space: O(n) - counter
pub fn k_most_frequent_strings(strs: &[&str], k: usize) -> Vec<String> {
    let mut counter = HashMap::new();
    for &s in strs {
        *counter.entry(s.to_string()).or_insert(0) += 1;
    }

    let mut heap: BinaryHeap<Pair> = counter
        .into_iter()
        .map(|(s, count)| Pair(s, count))
        .collect();

    (0..k).filter_map(|_| heap.pop()).map(|p| p.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build a linked list from a vec
    fn build_list(vals: Vec<i32>) -> ListNode {
        let mut head: ListNode = None;
        for &val in vals.iter().rev() {
            let mut node = Node::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    // Helper to convert linked list to vec
    fn list_to_vec(mut head: ListNode) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        result
    }

    #[test]
    fn test_kth_largest_example() {
        assert_eq!(kth_largest(&[5, 2, 4, 3, 1, 6], 3), 4);
    }

    #[test]
    fn test_kth_largest_first() {
        assert_eq!(kth_largest(&[5, 2, 4, 3, 1, 6], 1), 6);
    }

    #[test]
    fn test_kth_largest_last() {
        assert_eq!(kth_largest(&[5, 2, 4, 3, 1, 6], 6), 1);
    }

    #[test]
    fn test_kth_largest_single() {
        assert_eq!(kth_largest(&[42], 1), 42);
    }

    #[test]
    fn test_kth_largest_duplicates() {
        assert_eq!(kth_largest(&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }

    #[test]
    fn test_kth_largest_negative() {
        assert_eq!(kth_largest(&[-1, -5, 0, 3, -2], 2), 0);
    }

    #[test]
    fn test_combine_sorted_lists_example() {
        let lists = vec![
            build_list(vec![1, 3, 5]),
            build_list(vec![2, 4, 6]),
            build_list(vec![0, 7, 8]),
        ];
        assert_eq!(
            list_to_vec(combine_sorted_lists(lists)),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn test_combine_sorted_lists_empty() {
        let lists: Vec<ListNode> = vec![];
        assert_eq!(list_to_vec(combine_sorted_lists(lists)), vec![]);
    }

    #[test]
    fn test_combine_sorted_lists_all_empty() {
        let lists = vec![None, None, None];
        assert_eq!(list_to_vec(combine_sorted_lists(lists)), vec![]);
    }

    #[test]
    fn test_combine_sorted_lists_single_list() {
        let lists = vec![build_list(vec![1, 2, 3])];
        assert_eq!(list_to_vec(combine_sorted_lists(lists)), vec![1, 2, 3]);
    }

    #[test]
    fn test_combine_sorted_lists_two_lists() {
        let lists = vec![build_list(vec![1, 3, 5]), build_list(vec![2, 4, 6])];
        assert_eq!(
            list_to_vec(combine_sorted_lists(lists)),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_combine_sorted_lists_with_duplicates() {
        let lists = vec![build_list(vec![1, 2, 2]), build_list(vec![1, 1, 2])];
        assert_eq!(
            list_to_vec(combine_sorted_lists(lists)),
            vec![1, 1, 1, 2, 2, 2]
        );
    }

    #[test]
    fn test_combine_sorted_lists_negative_numbers() {
        let lists = vec![build_list(vec![-3, -1, 2]), build_list(vec![-2, 0, 3])];
        assert_eq!(
            list_to_vec(combine_sorted_lists(lists)),
            vec![-3, -2, -1, 0, 2, 3]
        );
    }

    #[test]
    fn test_median_finder_example() {
        let mut mf = MedianFinder::new();
        mf.add(3);
        mf.add(6);
        assert_eq!(mf.get_median(), 4.5);
        mf.add(1);
        assert_eq!(mf.get_median(), 3.0);
    }

    #[test]
    fn test_median_finder_single() {
        let mut mf = MedianFinder::new();
        mf.add(5);
        assert_eq!(mf.get_median(), 5.0);
    }

    #[test]
    fn test_median_finder_two_elements() {
        let mut mf = MedianFinder::new();
        mf.add(1);
        mf.add(2);
        assert_eq!(mf.get_median(), 1.5);
    }

    #[test]
    fn test_median_finder_odd_count() {
        let mut mf = MedianFinder::new();
        mf.add(1);
        mf.add(2);
        mf.add(3);
        mf.add(4);
        mf.add(5);
        assert_eq!(mf.get_median(), 3.0);
    }

    #[test]
    fn test_median_finder_even_count() {
        let mut mf = MedianFinder::new();
        mf.add(1);
        mf.add(2);
        mf.add(3);
        mf.add(4);
        assert_eq!(mf.get_median(), 2.5);
    }

    #[test]
    fn test_median_finder_negative_numbers() {
        let mut mf = MedianFinder::new();
        mf.add(-5);
        mf.add(-3);
        mf.add(-1);
        assert_eq!(mf.get_median(), -3.0);
    }

    #[test]
    fn test_median_finder_duplicates() {
        let mut mf = MedianFinder::new();
        mf.add(5);
        mf.add(5);
        mf.add(5);
        assert_eq!(mf.get_median(), 5.0);
    }

    #[test]
    fn test_k_most_frequent_strings_example() {
        assert_eq!(
            k_most_frequent_strings(
                &["go", "coding", "byte", "byte", "go", "interview", "go"],
                2
            ),
            vec!["go", "byte"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_single() {
        assert_eq!(
            k_most_frequent_strings(&["a", "b", "c", "a", "a"], 1),
            vec!["a"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_all_same_freq() {
        // When frequencies are equal, sort lexicographically
        assert_eq!(k_most_frequent_strings(&["c", "b", "a"], 2), vec!["a", "b"]);
    }

    #[test]
    fn test_k_most_frequent_strings_k_equals_n() {
        assert_eq!(
            k_most_frequent_strings(&["x", "y", "z"], 3),
            vec!["x", "y", "z"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_tie_breaker() {
        // "apple" and "banana" both have freq 2, so lexicographical order
        assert_eq!(
            k_most_frequent_strings(&["apple", "banana", "apple", "banana", "cherry"], 2),
            vec!["apple", "banana"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_empty_k_zero() {
        assert_eq!(
            k_most_frequent_strings(&["a", "b", "c"], 0),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_k_most_frequent_strings_all_same() {
        assert_eq!(
            k_most_frequent_strings(&["same", "same", "same"], 1),
            vec!["same"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_mixed_frequencies() {
        // a: 4, b: 3, c: 2, d: 1
        assert_eq!(
            k_most_frequent_strings(&["a", "a", "a", "a", "b", "b", "b", "c", "c", "d"], 3),
            vec!["a", "b", "c"]
        );
    }
}
