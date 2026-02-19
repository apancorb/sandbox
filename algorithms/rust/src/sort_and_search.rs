use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

/// Linked list node using Rc<RefCell> for easier split/merge operations
#[derive(Debug, PartialEq)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

pub type NodeRef = Rc<RefCell<Node>>;

impl Node {
    pub fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(Node { val, next: None }))
    }

    pub fn from_vec(values: &[i32]) -> Option<NodeRef> {
        if values.is_empty() {
            return None;
        }
        let head = Node::new(values[0]);
        let mut curr = Rc::clone(&head);
        for &val in &values[1..] {
            let new_node = Node::new(val);
            curr.borrow_mut().next = Some(Rc::clone(&new_node));
            curr = new_node;
        }
        Some(head)
    }

    pub fn to_vec(head: &Option<NodeRef>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut curr = head.clone();
        while let Some(node) = curr {
            result.push(node.borrow().val);
            curr = node.borrow().next.clone();
        }
        result
    }
}

/// Sort Linked List
///
/// Given the head of a singly linked list, sort the linked list in ascending order.
/// Uses merge sort: split in half, sort each half, merge.
///
/// # Example
///
/// ```text
/// Input: 4 -> 2 -> 1 -> 3
///
/// Output: 1 -> 2 -> 3 -> 4
/// ```
pub fn sort_list(head: Option<NodeRef>) -> Option<NodeRef> {
    // Base case: empty or single node
    let head = head?;
    if head.borrow().next.is_none() {
        return Some(head);
    }

    // Split into two halves
    let right = split(Rc::clone(&head));
    let left = sort_list(Some(head));
    let right = sort_list(right);

    // Merge sorted halves
    merge(left, right)
}

fn split(head: NodeRef) -> Option<NodeRef> {
    let mut slow = Rc::clone(&head);
    let mut fast = Rc::clone(&head);

    loop {
        let Some(next) = fast.borrow().next.clone() else { break };
        let Some(next_next) = next.borrow().next.clone() else { break };

        let next_slow = slow.borrow().next.clone().unwrap();
        slow = next_slow;
        fast = next_next;
    }

    slow.borrow_mut().next.take()
}

fn merge(mut l1: Option<NodeRef>, mut l2: Option<NodeRef>) -> Option<NodeRef> {
    // Pick the smaller head as our result head
    let head = match (&l1, &l2) {
        (None, _) => return l2,
        (_, None) => return l1,
        (Some(a), Some(b)) => {
            if a.borrow().val <= b.borrow().val {
                let node = l1.take().unwrap();
                l1 = node.borrow().next.clone();
                node
            } else {
                let node = l2.take().unwrap();
                l2 = node.borrow().next.clone();
                node
            }
        }
    };

    let mut tail = Rc::clone(&head);

    while l1.is_some() || l2.is_some() {
        let take_l1 = match (&l1, &l2) {
            (None, _) => false,
            (_, None) => true,
            (Some(a), Some(b)) => a.borrow().val <= b.borrow().val,
        };

        let node = if take_l1 {
            let node = l1.take().unwrap();
            l1 = node.borrow().next.clone();
            node
        } else {
            let node = l2.take().unwrap();
            l2 = node.borrow().next.clone();
            node
        };

        tail.borrow_mut().next = Some(Rc::clone(&node));
        tail = node;
    }

    Some(head)
}

/// Kth Largest Integer
///
/// Return the kth largest integer in an array.
///
/// # Example
///
/// ```text
/// Input: nums = [5, 2, 4, 3, 1, 6], k = 3
///
/// Output: 4
/// ```
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

/// Sort Array
///
/// Given an integer array, sort it in ascending order.
///
/// # Example
///
/// ```text
/// Input: nums = [6, 8, 4, 2, 7, 3, 1, 5]
///
/// Output: [1, 2, 3, 4, 5, 6, 7, 8]
/// ```
// Quicksort Complexity:
//
// Time - Average O(n log n):
//   - Good pivots split array roughly in half → log(n) levels
//   - Each level does O(n) work (partition) → O(n log n) total
//
// Time - Worst O(n²):
//   - Bad pivots (always min/max) → one side has n-1 elements
//   - Creates n levels instead of log(n) → O(n²) total
//   - Happens with already sorted arrays when using last element as pivot
//
// Space - O(log n) average, O(n) worst:
//   - Recursion depth matches the number of levels
//
pub fn sort_array(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }
    quicksort(nums, 0, nums.len() - 1);
}

fn quicksort(nums: &mut [i32], left: usize, right: usize) {
    // Base case: if the subarray has 0 or 1 element, it's already sorted
    if left >= right {
        return;
    }

    // Partition the array and retrieve the pivot index
    let pivot_index = partition(nums, left, right);

    // Recursively sort left and right parts
    if pivot_index > 0 {
        quicksort(nums, left, pivot_index - 1);
    }
    quicksort(nums, pivot_index + 1, right);
}

fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = nums[right];
    let mut lo = left;

    // Move all numbers less than pivot to the left
    for i in left..right {
        if nums[i] < pivot {
            nums.swap(lo, i);
            lo += 1;
        }
    }

    // Swap pivot into its correct position
    nums.swap(lo, right);
    lo
}

/// Merge Sorted Array
///
/// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
/// The result is stored in nums1, which has length m + n (last n elements are 0s).
///
/// # Example
///
/// ```text
/// Input: nums1 = [1, 2, 3, 0, 0, 0], m = 3, nums2 = [2, 5, 6], n = 3
/// Output: nums1 = [1, 2, 2, 3, 5, 6]
/// ```
pub fn merge_sorted_array(nums1: &mut [i32], m: usize, nums2: &[i32], n: usize) {
    let copy_nums1 = nums1[0..m].to_vec();
    let mut p1 = 0;
    let mut p2 = 0;

    for i in 0..(m + n) {
        if p2 >= n || (p1 < m && copy_nums1[p1] <= nums2[p2]) {
            nums1[i] = copy_nums1[p1];
            p1 += 1;
        } else {
            nums1[i] = nums2[p2];
            p2 += 1;
        }
    }
}

/// H-Index
///
/// Given an array where citations[i] is the number of citations for the ith paper,
/// return the researcher's h-index: the maximum value h such that the researcher
/// has at least h papers with at least h citations each.
///
/// # Example 1
///
/// ```text
/// Input: citations = [3, 0, 6, 1, 5]
/// Output: 3
/// Explanation: 3 papers have at least 3 citations each
/// ```
///
/// # Example 2
///
/// ```text
/// Input: citations = [1, 3, 1]
/// Output: 1
/// ```
pub fn h_index(citations: &[usize]) -> usize {
    let mut sorted = citations.to_vec();
    sorted.sort();

    let n = sorted.len();
    let mut h = 0;

    for i in 0..n {
        let papers_with_at_least = n - i;
        let citation_count = sorted[i];

        if citation_count >= papers_with_at_least {
            h = h.max(papers_with_at_least);
        }
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_list_example() {
        let head = Node::from_vec(&[4, 2, 1, 3]);
        let sorted = sort_list(head);
        assert_eq!(Node::to_vec(&sorted), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sort_list_empty() {
        let head: Option<NodeRef> = None;
        let sorted = sort_list(head);
        assert_eq!(sorted, None);
    }

    #[test]
    fn test_sort_list_single() {
        let head = Node::from_vec(&[5]);
        let sorted = sort_list(head);
        assert_eq!(Node::to_vec(&sorted), vec![5]);
    }

    #[test]
    fn test_sort_list_already_sorted() {
        let head = Node::from_vec(&[1, 2, 3, 4, 5]);
        let sorted = sort_list(head);
        assert_eq!(Node::to_vec(&sorted), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_list_reverse() {
        let head = Node::from_vec(&[5, 4, 3, 2, 1]);
        let sorted = sort_list(head);
        assert_eq!(Node::to_vec(&sorted), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_list_duplicates() {
        let head = Node::from_vec(&[3, 1, 2, 1, 3]);
        let sorted = sort_list(head);
        assert_eq!(Node::to_vec(&sorted), vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_sort_array_example() {
        let mut nums = [6, 8, 4, 2, 7, 3, 1, 5];
        sort_array(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_sort_array_empty() {
        let mut nums: [i32; 0] = [];
        sort_array(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test_sort_array_single() {
        let mut nums = [5];
        sort_array(&mut nums);
        assert_eq!(nums, [5]);
    }

    #[test]
    fn test_sort_array_already_sorted() {
        let mut nums = [1, 2, 3, 4, 5];
        sort_array(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_array_reverse() {
        let mut nums = [5, 4, 3, 2, 1];
        sort_array(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_array_duplicates() {
        let mut nums = [3, 1, 2, 1, 3];
        sort_array(&mut nums);
        assert_eq!(nums, [1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_sort_array_negative() {
        let mut nums = [3, -1, 0, -5, 2];
        sort_array(&mut nums);
        assert_eq!(nums, [-5, -1, 0, 2, 3]);
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
    fn test_merge_sorted_array_example() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let nums2 = vec![2, 5, 6];
        merge_sorted_array(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_merge_sorted_array_nums2_empty() {
        let mut nums1 = vec![1, 2, 3];
        let nums2: Vec<i32> = vec![];
        merge_sorted_array(&mut nums1, 3, &nums2, 0);
        assert_eq!(nums1, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sorted_array_nums1_empty() {
        let mut nums1 = vec![0];
        let nums2 = vec![1];
        merge_sorted_array(&mut nums1, 0, &nums2, 1);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_merge_sorted_array_interleaved() {
        let mut nums1 = vec![1, 3, 5, 0, 0, 0];
        let nums2 = vec![2, 4, 6];
        merge_sorted_array(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sorted_array_nums2_all_smaller() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let nums2 = vec![1, 2, 3];
        merge_sorted_array(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_h_index_example1() {
        assert_eq!(h_index(&[3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn test_h_index_example2() {
        assert_eq!(h_index(&[1, 3, 1]), 1);
    }

    #[test]
    fn test_h_index_all_zeros() {
        assert_eq!(h_index(&[0, 0, 0]), 0);
    }

    #[test]
    fn test_h_index_single_high() {
        assert_eq!(h_index(&[100]), 1);
    }

    #[test]
    fn test_h_index_single_zero() {
        assert_eq!(h_index(&[0]), 0);
    }

    #[test]
    fn test_h_index_all_same() {
        assert_eq!(h_index(&[5, 5, 5, 5, 5]), 5);
    }
}
