use std::collections::HashSet;

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
/// ```
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
/// ```
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
/// ```
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
}
