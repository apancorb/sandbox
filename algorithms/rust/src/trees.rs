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
/// Mirror a binary tree (swap every left and right child).
///
/// # Examples
///
/// ```text
/// Input:        Output:
///     1             1
///    / \           / \
///   2   3         3   2
///  / \               / \
/// 4   5             5   4
/// ```
///
/// At each node, swap left and right, then recurse. The key insight is
/// that inverting is just swapping children at every level. You don't
/// need to think about it globally — just swap locally and let
/// recursion handle the rest.
///
/// Example walkthrough for tree (1, (2, 4, 5), 3):
///
/// ```text
/// at node 1: swap children → left=3, right=2
/// recurse left (node 3): no children to swap
/// recurse right (node 2): swap → left=5, right=4
/// Result: 1(3, 2(5, 4))
/// ```
///
/// # Complexity
///
/// - Time: O(n) — visit every node
/// - Space: O(h) — recursion stack (h = height). Each recursive call stays
///   on the stack until it returns. The deepest the stack gets = longest
///   root-to-leaf path = height. Balanced tree: h = log n. Skewed tree: h = n.
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
/// A tree is balanced if no node's subtrees differ in height by more than 1.
///
/// # Examples
///
/// ```text
/// Balanced:       Not balanced:
///     1               1
///    / \             /
///   2   3           2
///  / \             /
/// 4   5           3
/// ```
///
/// Return height from each subtree. If any subtree is unbalanced,
/// propagate -1 up as a "poisoned" signal.
///
/// helper(node) returns:
/// - -1   → unbalanced somewhere below
/// - >= 0 → height of this subtree
///
/// Example walkthrough for balanced tree (1, (2, 4, 5), 3):
///
/// ```text
/// helper(4)=0, helper(5)=0
/// helper(2): |0-0|=0 ≤1 ✓ → return 1
/// helper(3)=0
/// helper(1): |1-0|=1 ≤1 ✓ → return 2 → balanced!
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(h)
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

/// Rightmost Nodes of a Binary Tree (Right Side View)
///
/// Return the last node value at each level.
///
/// # Examples
///
/// ```text
/// Input:
///     1
///    / \
///   2   3
///  / \   \
/// 4   5   6
///
/// Output: [1, 3, 6]
/// ```
///
/// BFS level by level. The last node in each level is the rightmost.
///
/// Use a queue. Process one level at a time:
///
/// ```text
/// level_size = len(queue)
/// for i in range(level_size):
///     node = queue.popleft()
///     if i == level_size - 1: → this is the rightmost!
///     add children to queue
/// ```
///
/// # Complexity
///
/// - Time: O(n) — visit every node
/// - Space: O(w) — max width of tree (queue)
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
/// Check if a binary tree is a valid BST:
/// - Left subtree values < node value
/// - Right subtree values > node value
/// - This must hold for ALL ancestors, not just the parent
///
/// # Examples
///
/// ```text
///     5
///    / \
///   3   7       → True (all constraints satisfied)
///  / \ / \
/// 2  4 6  8
///
///     5
///    / \
///   3   7       → False (6 in left subtree of 5!)
///  / \ / \
/// 2  6 7  8
/// ```
///
/// Pass bounds down: each node must be in range (lower, upper).
/// Go left  → update upper bound to current val
/// Go right → update lower bound to current val
///
/// Example walkthrough for valid tree:
///
/// ```text
/// check(5, -inf, inf): 5 in range ✓
///   check(3, -inf, 5): 3 in range ✓
///     check(2, -inf, 3): 2 in range ✓
///     check(4, 3, 5):    4 in range ✓
///   check(7, 5, inf): 7 in range ✓
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(h)
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

/// Lowest Common Ancestor (LCA)
///
/// Find the lowest node that has both p and q as descendants.
/// A node can be its own ancestor.
///
/// # Examples
///
/// ```text
///     1
///    / \
///   2   3
///  / \
/// 4   5
///
/// LCA(4, 5) = 2    (both under 2)
/// LCA(4, 3) = 1    (split across root)
/// LCA(2, 4) = 2    (2 is ancestor of 4)
/// ```
///
/// Post-order DFS: search left and right subtrees.
/// - If current node is p or q → return it
/// - If left AND right both found something → current node is the LCA
/// - If only one side found → propagate that result up
///
/// Example walkthrough for LCA(4, 5) on tree above:
///
/// ```text
/// at node 4: val==4 → return 4
/// at node 5: val==5 → return 5
/// at node 2: left=4, right=5 → BOTH found → return 2 (LCA!)
/// at node 3: left=None, right=None → return None
/// at node 1: left=2, right=None → return 2
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(h)
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

/// Build Binary Tree from Preorder and Inorder Traversals
///
/// Given preorder and inorder traversal arrays, reconstruct the original
/// binary tree. Each value is unique.
///
/// # Examples
///
/// ```text
/// Input:
///   preorder = [5, 9, 2, 3, 4, 7]
///   inorder  = [2, 9, 5, 4, 3, 7]
///
/// Output:
///       5
///      / \
///     9   3
///    /   / \
///   2   4   7
/// ```
///
/// Key insight: preorder[0] is always the root. Find that root in the
/// inorder array — everything to its left is the left subtree, everything
/// to its right is the right subtree. Then recurse on each side. Use a
/// hash map for O(1) lookup of inorder indices. Use a shared preorder
/// index that increments as we consume elements.
///
/// Example walkthrough for preorder=[5,9,2,3,4,7], inorder=[2,9,5,4,3,7]:
///
/// ```text
/// root = preorder[0] = 5
/// find 5 in inorder at index 2
/// left subtree inorder:  [2, 9]     (indices 0..1)
/// right subtree inorder: [4, 3, 7]  (indices 3..5)
/// next preorder element for left subtree: 9
/// next preorder element for right subtree: 3
/// Recurse on each side...
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(n) — hash map + recursion
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

/// Maximum Path Sum in a Binary Tree
///
/// Find the maximum sum along any path (any node to any node).
/// Path = connected sequence of nodes, no splits.
///
/// # Examples
///
/// ```text
///     5
///    / \
///   3   8
///  / \   \
/// 4  -2   6
///
/// Best path: 4 → 3 → 5 → 8 → 6 = 26
/// ```
///
/// At each node, we decide:
/// - left_gain = max(0, best path going left)   ← 0 if negative (skip)
/// - right_gain = max(0, best path going right)
/// - path_through = val + left_gain + right_gain ← path using this node as "turn"
/// - Update global max with path_through
///
/// Return to parent: val + max(left_gain, right_gain)
/// (can only extend in ONE direction, not both, when going up)
///
/// Example walkthrough:
///
/// ```text
/// at node 4:  return 4,  path_through=4
/// at node -2: return 0 (capped), path_through=-2
/// at node 3:  left=4, right=0 → path_through=3+4+0=7, return 3+4=7
/// at node 6:  return 6,  path_through=6
/// at node 8:  left=0, right=6 → path_through=8+0+6=14, return 8+6=14
/// at node 5:  left=7, right=14 → path_through=5+7+14=26 ★, return 5+14=19
/// Answer: 26
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(h)
pub fn max_path_sum(root: TreeNode) -> i32 {
    fn max_path_sum_helper(node: &TreeNode, max_path_sum: &mut i32) -> i32 {
        let Some(node) = node else {
            return 0;
        };

        let left = max_path_sum_helper(&node.borrow().left, max_path_sum).max(0);
        let right = max_path_sum_helper(&node.borrow().right, max_path_sum).max(0);

        *max_path_sum = (*max_path_sum).max(node.borrow().val + left + right);

        node.borrow().val + left.max(right)
    }

    let mut max_path_sum = i32::MIN;
    max_path_sum_helper(&root, &mut max_path_sum);
    max_path_sum
}

/// Widest Binary Tree Level
///
/// Return the width of the widest level, including null gaps.
///
/// # Examples
///
/// ```text
///     1
///    / \
///   2   3
///  /     \
/// 4       5
///
/// Level 2 has nodes at positions 0 and 3 → width = 4
/// (positions: 4, null, null, 5)
/// ```
///
/// Assign position indices like a heap array:
/// - root = 0
/// - left child = 2*i + 1
/// - right child = 2*i + 2
///
/// Width of a level = rightmost_pos - leftmost_pos + 1
///
/// Example walkthrough:
///
/// ```text
/// Level 0: node 1 at pos 0           → width = 1
/// Level 1: node 2 at pos 1, 3 at pos 2  → width = 2
/// Level 2: node 4 at pos 3, 5 at pos 6  → width = 6-3+1 = 4 ★
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(w) — max width (queue)
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
    fn test_max_path_sum_example() {
        //        5
        //       / \
        //      3   8
        //     / \   \
        //    4  -2   6
        // Path: 4 -> 3 -> 5 -> 8 -> 6 = 26
        let root = tree_node(
            5,
            tree_node(3, leaf(4), leaf(-2)),
            tree_node(8, None, leaf(6)),
        );
        assert_eq!(max_path_sum(root), 26);
    }

    #[test]
    fn test_max_path_sum_single() {
        assert_eq!(max_path_sum(leaf(5)), 5);
    }

    #[test]
    fn test_max_path_sum_negative_single() {
        assert_eq!(max_path_sum(leaf(-3)), -3);
    }

    #[test]
    fn test_max_path_sum_all_negative() {
        //      -3
        //     /  \
        //   -2   -1
        let root = tree_node(-3, leaf(-2), leaf(-1));
        assert_eq!(max_path_sum(root), -1);
    }

    #[test]
    fn test_max_path_sum_skip_subtree() {
        //        10
        //       /  \
        //      2   -25
        //          /  \
        //         3    4
        // Best path is just 10 + 2 = 12, skip the right subtree
        let root = tree_node(10, leaf(2), tree_node(-25, leaf(3), leaf(4)));
        assert_eq!(max_path_sum(root), 12);
    }

    #[test]
    fn test_max_path_sum_through_root() {
        //      1
        //     / \
        //    2   3
        let root = tree_node(1, leaf(2), leaf(3));
        assert_eq!(max_path_sum(root), 6);
    }

    #[test]
    fn test_max_path_sum_not_through_root() {
        //        -10
        //       /   \
        //      9     20
        //           /  \
        //          15   7
        // Best path: 15 -> 20 -> 7 = 42
        let root = tree_node(-10, leaf(9), tree_node(20, leaf(15), leaf(7)));
        assert_eq!(max_path_sum(root), 42);
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
