use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = Option<Rc<RefCell<Node>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub left: TreeNode,
    pub right: TreeNode,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

/// Invert Binary Tree
///
/// Invert a binary tree and return its root. When a binary tree is inverted, it becomes the
/// mirror image of itself.
///
/// # Example
///
/// ```text
/// Input:
///        1
///       / \
///      2   3
///     / \
///    4   5
///
/// Output:
///        1
///       / \
///      3   2
///         / \
///        5   4
/// ```
pub fn invert_tree(root: TreeNode) -> TreeNode {
    // 1st solution:
    //
    // if let Some(node) = &root {
    //     let left_node = node.borrow().left.clone();
    //     let right_node = node.borrow().right.clone();
    //
    //     node.borrow_mut().left = right_node.clone();
    //     node.borrow_mut().right = left_node.clone();
    //
    //     invert_tree(left_node);
    //     invert_tree(right_node);
    // }
    //
    // root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree_node(val: i32, left: TreeNode, right: TreeNode) -> TreeNode {
        Some(Rc::new(RefCell::new(Node { val, left, right })))
    }

    fn leaf(val: i32) -> TreeNode {
        tree_node(val, None, None)
    }

    #[test]
    fn test_invert_tree_example() {
        // Input:    1
        //          / \
        //         2   3
        //        / \
        //       4   5
        let root = tree_node(1, tree_node(2, leaf(4), leaf(5)), leaf(3));

        // Expected: 1
        //          / \
        //         3   2
        //            / \
        //           5   4
        let expected = tree_node(1, leaf(3), tree_node(2, leaf(5), leaf(4)));

        assert_eq!(invert_tree(root), expected);
    }

    #[test]
    fn test_invert_tree_empty() {
        assert_eq!(invert_tree(None), None);
    }

    #[test]
    fn test_invert_tree_single() {
        assert_eq!(invert_tree(leaf(1)), leaf(1));
    }

    #[test]
    fn test_invert_tree_two_levels() {
        let root = tree_node(1, leaf(2), leaf(3));
        let expected = tree_node(1, leaf(3), leaf(2));
        assert_eq!(invert_tree(root), expected);
    }

    #[test]
    fn test_invert_tree_left_only() {
        let root = tree_node(1, leaf(2), None);
        let expected = tree_node(1, None, leaf(2));
        assert_eq!(invert_tree(root), expected);
    }

    #[test]
    fn test_invert_tree_right_only() {
        let root = tree_node(1, None, leaf(2));
        let expected = tree_node(1, leaf(2), None);
        assert_eq!(invert_tree(root), expected);
    }

    #[test]
    fn test_invert_tree_twice() {
        let root = tree_node(1, tree_node(2, leaf(4), leaf(5)), leaf(3));
        let original = tree_node(1, tree_node(2, leaf(4), leaf(5)), leaf(3));

        // Inverting twice should give back the original
        assert_eq!(invert_tree(invert_tree(root)), original);
    }
}
