use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
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
    //     let left_node = node.borrow_mut().left.take();
    //     let right_node = node.borrow_mut().right.take();
    //
    //     node.borrow_mut().left = right_node.clone();
    //     node.borrow_mut().right = left_node.clone();
    //
    //     invert_tree(left_node);
    //     invert_tree(right_node);
    // }
    //
    // root
    //
    // 2nd solution:
    //
    let mut stack = vec![root.clone()];

    while let Some(Some(node)) = stack.pop() {
        let left = node.borrow_mut().left.take();
        let right = node.borrow_mut().right.take();

        node.borrow_mut().left = right.clone();
        node.borrow_mut().right = left.clone();

        if !left.is_none() {
            stack.push(left);
        }

        if !right.is_none() {
            stack.push(right);
        }
    }

    root
}

/// Balanced Binary Tree Validation
///
/// Determine if a binary tree is height-balanced, meaning no node's left subtree and right
/// subtree have a height difference greater than 1.
///
/// # Example
///
/// ```text
/// Balanced:
///        1
///       / \
///      2   3
///     / \
///    4   5
///
/// Not Balanced:
///        1
///       /
///      2
///     /
///    3
/// ```
pub fn is_balanced(root: TreeNode) -> bool {
    fn helper(node: &TreeNode) -> i32 {
        let Some(node) = node else {
            return 0;
        };

        let left = helper(&node.borrow().left);
        let right = helper(&node.borrow().right);

        if left == -1 || right == -1 {
            return -1;
        }

        if (left - right).abs() > 1 {
            return -1;
        }

        1 + left.max(right)
    }

    helper(&root) != -1
}

/// Rightmost Nodes of a Binary Tree
///
/// Return an array containing the values of the rightmost nodes at each level of a binary tree.
///
/// # Example
///
/// ```text
/// Input:
///        1
///       / \
///      2   3
///     / \   \
///    4   5   6
///
/// Output: [1, 3, 6]
/// Explanation: The rightmost nodes at each level are 1 (level 0), 3 (level 1), and 6 (level 2).
/// ```
pub fn rightmost_nodes(root: TreeNode) -> Vec<i32> {
    let Some(root) = root else {
        return Vec::new();
    };

    let mut ans = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let level_size = queue.len();

        for i in 0..level_size {
            let node = queue.pop_front().unwrap();

            if i == level_size - 1 {
                ans.push(node.borrow().val);
            }

            if let Some(left) = &node.borrow().left {
                queue.push_back(left.clone());
            }

            if let Some(right) = &node.borrow().right {
                queue.push_back(right.clone());
            }
        }
    }

    ans
}

/// Binary Search Tree Validation
///
/// Verify whether a binary tree is a valid binary search tree (BST). A BST is a binary tree where
/// each node meets the following criteria:
/// - A node's left subtree contains only nodes of lower values than the node's value.
/// - A node's right subtree contains only nodes of greater values than the node's value.
///
/// # Example
///
/// ```text
/// Input:
///        5
///       / \
///      3   7
///     / \ / \
///    2  6 7  8
///
/// Output: false
/// Explanation: This tree has two violations of the BST criteria:
/// - Node 5's left subtree contains node 6, and node 6's value is greater than 5.
/// - Node 7 has a left child with the same value of 7.
/// ```
pub fn is_valid_bst(root: TreeNode) -> bool {
    fn helper(node: &TreeNode, lower_bound: i32, upper_bound: i32) -> bool {
        let Some(node) = node else {
            return true;
        };

        if lower_bound >= node.borrow().val || node.borrow().val >= upper_bound {
            return false;
        }

        helper(&node.borrow().left, lower_bound, node.borrow().val)
            && helper(&node.borrow().right, node.borrow().val, upper_bound)
    }

    helper(&root, i32::MIN, i32::MAX)
}

/// Lowest Common Ancestor
///
/// Return the lowest common ancestor (LCA) of two nodes, p and q, in a binary tree. The LCA is
/// defined as the lowest node that has both p and q as descendants. A node can be considered an
/// ancestor of itself.
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
/// p = 4, q = 5
/// Output: 2
/// Explanation: The LCA of nodes 4 and 5 is node 2.
///
/// p = 4, q = 3
/// Output: 1
/// Explanation: The LCA of nodes 4 and 3 is node 1.
/// ```
///
/// # Constraints
///
/// - The tree contains at least two nodes.
/// - All node values are unique.
/// - p and q represent different nodes in the tree.
pub fn lowest_common_ancestor(root: TreeNode, p: i32, q: i32) -> TreeNode {
    fn helper(node: &TreeNode, p: i32, q: i32) -> TreeNode {
        let Some(n) = node else {
            return None;
        };

        if n.borrow().val == p || n.borrow().val == q {
            return node.clone();
        }

        let left = helper(&n.borrow().left, p, q);
        let right = helper(&n.borrow().right, p, q);

        if left.is_some() && right.is_some() {
            return node.clone();
        }

        if left.is_some() { left } else { right }
    }
    helper(&root, p, q)
}

/// Build a Binary Tree From Preorder and Inorder Traversals
///
/// Construct a binary tree using arrays of values obtained after a preorder traversal and an
/// inorder traversal of the tree.
///
/// # Example
///
/// ```text
/// Input: preorder = [5, 9, 2, 3, 4, 7], inorder = [2, 9, 5, 4, 3, 7]
///
/// Output:
///        5
///       / \
///      9   3
///     /   / \
///    2   4   7
/// ```
///
/// # Constraints
///
/// - The tree consists of unique values.
pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> TreeNode {
    if preorder.is_empty() {
        return None;
    }

    let mut inorder_indexes = HashMap::new();
    for (i, &val) in inorder.iter().enumerate() {
        inorder_indexes.insert(val, i);
    }

    fn build_tree_helper(
        preorder: &[i32],
        inorder: &[i32],
        left: usize,
        right: usize,
        inorder_indexes: &HashMap<i32, usize>,
        preorder_index: &mut usize,
    ) -> TreeNode {
        if left > right {
            return None;
        }

        let val = preorder[*preorder_index];

        let inorder_index = *inorder_indexes.get(&val).unwrap();
        *preorder_index += 1;

        let mut node = Node::new(val);

        if inorder_index > 0 {
            node.left = build_tree_helper(
                preorder,
                inorder,
                left,
                inorder_index - 1,
                inorder_indexes,
                preorder_index,
            );
        }

        if inorder_index < inorder.len() - 1 {
            node.right = build_tree_helper(
                preorder,
                inorder,
                inorder_index + 1,
                right,
                inorder_indexes,
                preorder_index,
            );
        }

        Some(Rc::new(RefCell::new(node)))
    }

    build_tree_helper(
        preorder,
        inorder,
        0,
        inorder.len() - 1,
        &inorder_indexes,
        &mut 0,
    )
}

/// Widest Binary Tree Level
///
/// Return the width of the widest level in a binary tree, where the width of a level is defined
/// as the distance between its leftmost and rightmost non-null nodes.
///
/// The width includes any null nodes that would be between the leftmost and rightmost nodes.
///
/// # Example
///
/// ```text
/// Input:
///        1
///       / \
///      2   3
///     /     \
///    4       5
///
/// Output: 4
/// Explanation: Level 2 has nodes 4 and 5. The width is 4 (positions: 4, null, null, 5).
/// ```
pub fn widest_level(root: TreeNode) -> usize {
    let Some(root) = root else {
        return 0;
    };

    let mut max_width = 0;
    let mut queue = VecDeque::new();
    queue.push_back((root, 0));

    while !queue.is_empty() {
        let size = queue.len();

        let left_index = queue[0].1;
        let right_index = queue[size - 1].1;

        max_width = max_width.max(right_index - left_index + 1);

        for _ in 0..size {
            let (node, i) = queue.pop_front().unwrap();

            if let Some(left_node) = &node.borrow().left {
                queue.push_back((left_node.clone(), 2 * i + 1));
            }

            if let Some(right_node) = &node.borrow().right {
                queue.push_back((right_node.clone(), 2 * i + 2));
            }
        }
    }

    max_width
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
    fn test_build_tree_example() {
        //        5
        //       / \
        //      9   3
        //     /   / \
        //    2   4   7
        let expected = tree_node(
            5,
            tree_node(9, leaf(2), None),
            tree_node(3, leaf(4), leaf(7)),
        );
        assert_eq!(
            build_tree(&[5, 9, 2, 3, 4, 7], &[2, 9, 5, 4, 3, 7]),
            expected
        );
    }

    #[test]
    fn test_build_tree_empty() {
        assert_eq!(build_tree(&[], &[]), None);
    }

    #[test]
    fn test_build_tree_single() {
        assert_eq!(build_tree(&[1], &[1]), leaf(1));
    }

    #[test]
    fn test_build_tree_left_only() {
        //      1
        //     /
        //    2
        //   /
        //  3
        let expected = tree_node(1, tree_node(2, leaf(3), None), None);
        assert_eq!(build_tree(&[1, 2, 3], &[3, 2, 1]), expected);
    }

    #[test]
    fn test_build_tree_right_only() {
        //  1
        //   \
        //    2
        //     \
        //      3
        let expected = tree_node(1, None, tree_node(2, None, leaf(3)));
        assert_eq!(build_tree(&[1, 2, 3], &[1, 2, 3]), expected);
    }

    #[test]
    fn test_build_tree_balanced() {
        //        1
        //       / \
        //      2   3
        //     / \ / \
        //    4  5 6  7
        let expected = tree_node(
            1,
            tree_node(2, leaf(4), leaf(5)),
            tree_node(3, leaf(6), leaf(7)),
        );
        assert_eq!(
            build_tree(&[1, 2, 4, 5, 3, 6, 7], &[4, 2, 5, 1, 6, 3, 7]),
            expected
        );
    }

    #[test]
    fn test_lca_example_siblings() {
        //        1
        //       / \
        //      2   3
        //     / \
        //    4   5
        let root = tree_node(1, tree_node(2, leaf(4), leaf(5)), leaf(3));
        let result = lowest_common_ancestor(root, 4, 5);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_lca_example_different_subtrees() {
        //        1
        //       / \
        //      2   3
        //     / \
        //    4   5
        let root = tree_node(1, tree_node(2, leaf(4), leaf(5)), leaf(3));
        let result = lowest_common_ancestor(root, 4, 3);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_lca_ancestor_is_node() {
        //        1
        //       / \
        //      2   3
        //     / \
        //    4   5
        let root = tree_node(1, tree_node(2, leaf(4), leaf(5)), leaf(3));
        let result = lowest_common_ancestor(root, 2, 4);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_lca_root_is_answer() {
        //      1
        //     / \
        //    2   3
        let root = tree_node(1, leaf(2), leaf(3));
        let result = lowest_common_ancestor(root, 2, 3);
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    #[test]
    fn test_lca_deep_tree() {
        //        1
        //       /
        //      2
        //     /
        //    3
        //   /
        //  4
        let root = tree_node(1, tree_node(2, tree_node(3, leaf(4), None), None), None);
        let result = lowest_common_ancestor(root, 3, 4);
        assert_eq!(result.unwrap().borrow().val, 3);
    }

    #[test]
    fn test_lca_right_subtree() {
        //    1
        //     \
        //      2
        //     / \
        //    3   4
        let root = tree_node(1, None, tree_node(2, leaf(3), leaf(4)));
        let result = lowest_common_ancestor(root, 3, 4);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_is_valid_bst_example() {
        //        5
        //       / \
        //      3   7
        //     / \ / \
        //    2  6 7  8
        let root = tree_node(
            5,
            tree_node(3, leaf(2), leaf(6)),
            tree_node(7, leaf(7), leaf(8)),
        );
        assert!(!is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_valid() {
        //        5
        //       / \
        //      3   7
        //     / \ / \
        //    2  4 6  8
        let root = tree_node(
            5,
            tree_node(3, leaf(2), leaf(4)),
            tree_node(7, leaf(6), leaf(8)),
        );
        assert!(is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_empty() {
        assert!(is_valid_bst(None));
    }

    #[test]
    fn test_is_valid_bst_single() {
        assert!(is_valid_bst(leaf(1)));
    }

    #[test]
    fn test_is_valid_bst_left_only_valid() {
        //      5
        //     /
        //    3
        let root = tree_node(5, leaf(3), None);
        assert!(is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_left_only_invalid() {
        //      5
        //     /
        //    7
        let root = tree_node(5, leaf(7), None);
        assert!(!is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_right_only_valid() {
        //    5
        //     \
        //      7
        let root = tree_node(5, None, leaf(7));
        assert!(is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_right_only_invalid() {
        //    5
        //     \
        //      3
        let root = tree_node(5, None, leaf(3));
        assert!(!is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_subtree_violation() {
        // The tricky case: 6 is valid as right child of 3,
        // but violates 5's left subtree constraint
        //        5
        //       /
        //      3
        //       \
        //        6
        let root = tree_node(5, tree_node(3, None, leaf(6)), None);
        assert!(!is_valid_bst(root));
    }

    #[test]
    fn test_is_valid_bst_equal_values() {
        //      5
        //     /
        //    5
        let root = tree_node(5, leaf(5), None);
        assert!(!is_valid_bst(root));
    }

    #[test]
    fn test_widest_level_example() {
        //        1
        //       / \
        //      2   3
        //     /     \
        //    4       5
        let root = tree_node(1, tree_node(2, leaf(4), None), tree_node(3, None, leaf(5)));
        assert_eq!(widest_level(root), 4);
    }

    #[test]
    fn test_widest_level_empty() {
        assert_eq!(widest_level(None), 0);
    }

    #[test]
    fn test_widest_level_single() {
        assert_eq!(widest_level(leaf(1)), 1);
    }

    #[test]
    fn test_widest_level_full() {
        //        1
        //       / \
        //      2   3
        //     / \ / \
        //    4  5 6  7
        let root = tree_node(
            1,
            tree_node(2, leaf(4), leaf(5)),
            tree_node(3, leaf(6), leaf(7)),
        );
        assert_eq!(widest_level(root), 4);
    }

    #[test]
    fn test_widest_level_left_heavy() {
        //        1
        //       /
        //      2
        //     /
        //    3
        let root = tree_node(1, tree_node(2, leaf(3), None), None);
        assert_eq!(widest_level(root), 1);
    }

    #[test]
    fn test_widest_level_two_levels() {
        //      1
        //     / \
        //    2   3
        let root = tree_node(1, leaf(2), leaf(3));
        assert_eq!(widest_level(root), 2);
    }

    #[test]
    fn test_rightmost_nodes_example() {
        //        1
        //       / \
        //      2   3
        //     / \   \
        //    4   5   6
        let root = tree_node(
            1,
            tree_node(2, leaf(4), leaf(5)),
            tree_node(3, None, leaf(6)),
        );
        assert_eq!(rightmost_nodes(root), vec![1, 3, 6]);
    }

    #[test]
    fn test_rightmost_nodes_empty() {
        assert_eq!(rightmost_nodes(None), Vec::<i32>::new());
    }

    #[test]
    fn test_rightmost_nodes_single() {
        assert_eq!(rightmost_nodes(leaf(1)), vec![1]);
    }

    #[test]
    fn test_rightmost_nodes_left_only() {
        //      1
        //     /
        //    2
        //   /
        //  3
        let root = tree_node(1, tree_node(2, leaf(3), None), None);
        assert_eq!(rightmost_nodes(root), vec![1, 2, 3]);
    }

    #[test]
    fn test_rightmost_nodes_right_only() {
        //  1
        //   \
        //    2
        //     \
        //      3
        let root = tree_node(1, None, tree_node(2, None, leaf(3)));
        assert_eq!(rightmost_nodes(root), vec![1, 2, 3]);
    }

    #[test]
    fn test_rightmost_nodes_zigzag() {
        //      1
        //     /
        //    2
        //     \
        //      3
        let root = tree_node(1, tree_node(2, None, leaf(3)), None);
        assert_eq!(rightmost_nodes(root), vec![1, 2, 3]);
    }

    #[test]
    fn test_is_balanced_empty() {
        assert!(is_balanced(None));
    }

    #[test]
    fn test_is_balanced_single() {
        assert!(is_balanced(leaf(1)));
    }

    #[test]
    fn test_is_balanced_true() {
        //        1
        //       / \
        //      2   3
        //     / \
        //    4   5
        let root = tree_node(1, tree_node(2, leaf(4), leaf(5)), leaf(3));
        assert!(is_balanced(root));
    }

    #[test]
    fn test_is_balanced_false() {
        //        1
        //       /
        //      2
        //     /
        //    3
        let root = tree_node(1, tree_node(2, leaf(3), None), None);
        assert!(!is_balanced(root));
    }

    #[test]
    fn test_is_balanced_perfect() {
        //        1
        //       / \
        //      2   3
        //     / \ / \
        //    4  5 6  7
        let root = tree_node(
            1,
            tree_node(2, leaf(4), leaf(5)),
            tree_node(3, leaf(6), leaf(7)),
        );
        assert!(is_balanced(root));
    }

    #[test]
    fn test_is_balanced_unbalanced_deep() {
        //        1
        //       / \
        //      2   3
        //     /
        //    4
        //   /
        //  5
        let root = tree_node(1, tree_node(2, tree_node(4, leaf(5), None), None), leaf(3));
        assert!(!is_balanced(root));
    }

    #[test]
    fn test_is_balanced_one_child_each() {
        //      1
        //     / \
        //    2   3
        let root = tree_node(1, leaf(2), leaf(3));
        assert!(is_balanced(root));
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
