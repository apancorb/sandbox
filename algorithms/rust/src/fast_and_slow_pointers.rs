use std::{cell::RefCell, rc::Rc};

/// Node for linked list with potential cycles
/// Using Rc<RefCell<>> to allow multiple references (needed for cycles)
#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

/// Linked List Loop
///
/// Given a singly linked list, determine if it contains a cycle. A cycle occurs if a node's next
/// pointer references an earlier node in the list, causing a loop.
///
/// # Example
///
/// ```text
/// Input: 1 -> 2 -> 3 -> 4
///             ^         |
///             |_________|
///
/// Output: true
/// Explanation: Node 4's next points back to node 2, creating a cycle.
/// ```
// Floyd's Cycle Detection (Tortoise and Hare):
//
// Why it works:
// - Slow moves 1 step, fast moves 2 steps per iteration
// - If no cycle: fast reaches null → return false
// - If cycle exists: once both enter the cycle, fast gains 1 step per iteration
// - Gap closes by 1 each time: k → k-1 → k-2 → ... → 0 (they meet)
//
// Analogy: Two runners on a circular track - the faster one always laps the slower.
//
// Time: O(n) - at most 2 full traversals
// Space: O(1) - just two pointers
pub fn has_cycle(head: Option<Rc<RefCell<Node>>>) -> bool {
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
/// Given a singly linked list, find and return its middle node. If there are two middle nodes,
/// return the second one.
///
/// # Examples
///
/// Example 1:
/// ```text
/// Input: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7
/// Output: 4
/// ```
///
/// Example 2:
/// ```text
/// Input: 1 -> 2 -> 3 -> 4 -> 5 -> 6
/// Output: 4
/// ```
///
/// # Constraints
///
/// - The linked list contains at least one node.
/// - The linked list contains unique values.
pub fn find_middle(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
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

/// Happy Number
///
/// A happy number is defined as a number that, when repeatedly subjected to the process of
/// squaring its digits and summing those squares, eventually leads to 1.
/// An unhappy number will never reach 1 during this process, and will get stuck in an infinite loop.
///
/// Given an integer, determine if it's a happy number.
///
/// # Example
///
/// ```text
/// Input: n = 23
/// Output: true
/// Explanation: 2² + 3² = 13 → 1² + 3² = 10 → 1² + 0² = 1
/// ```
pub fn is_happy_number(num: u32) -> bool {
    let get_next_num = |mut curr_num: u32| -> u32 {
        let mut next_num = 0;
        while curr_num > 0 {
            let digit = curr_num % 10;
            curr_num = curr_num / 10;
            next_num += digit * digit;
        }
        next_num
    };

    let mut slow = num;
    let mut fast = num;

    loop {
        slow = get_next_num(slow);
        fast = get_next_num(get_next_num(fast));
        if fast == 1 {
            return true;
        } else if slow == fast {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cycle_with_cycle() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let node3 = Node::new(3);
        let node4 = Node::new(4);

        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node3));
        node3.borrow_mut().next = Some(Rc::clone(&node4));
        node4.borrow_mut().next = Some(Rc::clone(&node2)); // cycle back to node2

        assert!(has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_no_cycle() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let node3 = Node::new(3);

        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node3));

        assert!(!has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_single_node_no_cycle() {
        let node1 = Node::new(1);
        assert!(!has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_single_node_self_loop() {
        let node1 = Node::new(1);
        node1.borrow_mut().next = Some(Rc::clone(&node1)); // self loop

        assert!(has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_empty_list() {
        assert!(!has_cycle(None));
    }

    #[test]
    fn test_has_cycle_two_nodes_with_cycle() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);

        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node1)); // cycle back to node1

        assert!(has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_two_nodes_no_cycle() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);

        node1.borrow_mut().next = Some(Rc::clone(&node2));

        assert!(!has_cycle(Some(node1)));
    }

    #[test]
    fn test_has_cycle_long_list_cycle_at_end() {
        let nodes: Vec<Rc<RefCell<Node>>> = (1..=5).map(|i| Node::new(i)).collect();

        for i in 0..4 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        nodes[4].borrow_mut().next = Some(Rc::clone(&nodes[2])); // cycle to node 3

        assert!(has_cycle(Some(Rc::clone(&nodes[0]))));
    }

    // find_middle tests

    #[test]
    fn test_find_middle_odd_length() {
        let nodes: Vec<Rc<RefCell<Node>>> = (1..=7).map(|i| Node::new(i)).collect();
        for i in 0..6 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        let middle = find_middle(Some(Rc::clone(&nodes[0])));
        assert_eq!(middle.unwrap().borrow().val, 4);
    }

    #[test]
    fn test_find_middle_even_length() {
        let nodes: Vec<Rc<RefCell<Node>>> = (1..=6).map(|i| Node::new(i)).collect();
        for i in 0..5 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        let middle = find_middle(Some(Rc::clone(&nodes[0])));
        assert_eq!(middle.unwrap().borrow().val, 4);
    }

    #[test]
    fn test_find_middle_single_node() {
        let node = Node::new(42);
        let middle = find_middle(Some(node));
        assert_eq!(middle.unwrap().borrow().val, 42);
    }

    #[test]
    fn test_find_middle_two_nodes() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        node1.borrow_mut().next = Some(Rc::clone(&node2));
        let middle = find_middle(Some(node1));
        assert_eq!(middle.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_find_middle_three_nodes() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let node3 = Node::new(3);
        node1.borrow_mut().next = Some(Rc::clone(&node2));
        node2.borrow_mut().next = Some(Rc::clone(&node3));
        let middle = find_middle(Some(node1));
        assert_eq!(middle.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_find_middle_four_nodes() {
        let nodes: Vec<Rc<RefCell<Node>>> = (1..=4).map(|i| Node::new(i)).collect();
        for i in 0..3 {
            nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
        }
        let middle = find_middle(Some(Rc::clone(&nodes[0])));
        assert_eq!(middle.unwrap().borrow().val, 3);
    }

    // is_happy_number tests

    #[test]
    fn test_is_happy_number_23() {
        assert!(is_happy_number(23));
    }

    #[test]
    fn test_is_happy_number_1() {
        assert!(is_happy_number(1));
    }

    #[test]
    fn test_is_happy_number_7() {
        // 7 -> 49 -> 97 -> 130 -> 10 -> 1
        assert!(is_happy_number(7));
    }

    #[test]
    fn test_is_happy_number_19() {
        // 19 -> 82 -> 68 -> 100 -> 1
        assert!(is_happy_number(19));
    }

    #[test]
    fn test_is_unhappy_number_2() {
        assert!(!is_happy_number(2));
    }

    #[test]
    fn test_is_unhappy_number_4() {
        // 4 -> 16 -> 37 -> 58 -> 89 -> 145 -> 42 -> 20 -> 4 (cycle)
        assert!(!is_happy_number(4));
    }

    #[test]
    fn test_is_unhappy_number_20() {
        assert!(!is_happy_number(20));
    }

    #[test]
    fn test_is_happy_number_100() {
        // 100 -> 1
        assert!(is_happy_number(100));
    }
}
