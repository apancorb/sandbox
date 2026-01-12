use std::cell::RefCell;
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
}
