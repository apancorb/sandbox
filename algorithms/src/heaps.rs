use std::{
    cmp::Ordering,
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

/// Combine Sorted Linked Lists
///
/// Given k singly linked lists, each sorted in ascending order, combine them into one sorted
/// linked list.
///
/// # Example
///
/// ```text
/// Input: lists = [[1, 3, 5], [2, 4, 6], [0, 7, 8]]
/// Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]
/// ```
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

/// K Most Frequent Strings
///
/// Find the k most frequently occurring strings in an array, and return them sorted by
/// frequency in descending order. If two strings have the same frequency, sort them in
/// lexicographical order.
///
/// # Example
///
/// ```text
/// Input: strs = ["go", "coding", "byte", "byte", "go", "interview", "go"], k = 2
/// Output: ["go", "byte"]
/// Explanation: The strings "go" and "byte" appear the most frequently, with frequencies of
/// 3 and 2, respectively.
/// ```
///
/// # Constraints
///
/// - k <= n, where n denotes the length of the array.
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
