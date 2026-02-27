"""
Trees Pattern

A collection of algorithm problems using binary trees.

Binary tree node:
    class TreeNode:
        def __init__(self, val=0, left=None, right=None)

Common techniques:
- DFS (recursive): inorder, preorder, postorder traversals
- BFS (level order): use a queue (collections.deque)
- Pass bounds down (BST validation)
- Return info up (height, sum, found/not-found)

Helper:
    tree(val, left, right) builds a node quickly for tests
"""

from collections import deque


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __eq__(self, other):
        if not other:
            return False
        return (self.val == other.val
                and self.left == other.left
                and self.right == other.right)


def tree(val, left=None, right=None):
    """Shortcut to build a tree node for tests."""
    return TreeNode(val, left, right)


def invert_tree(root: TreeNode | None) -> TreeNode | None:
    """
    Invert Binary Tree

    Mirror a binary tree (swap every left and right child).

    Example:
            1              1
           / \\    →      / \\
          2   3          3   2
         / \\                / \\
        4   5              5   4

    At each node, swap left and right, then recurse. The key insight is
    that inverting is just swapping children at every level. You don't
    need to think about it globally -- just swap locally and let
    recursion handle the rest.

    Example tree (1,2,3,4,5):
        at node 1: swap children → left=3, right=2
        recurse left (node 3): no children to swap
        recurse right (node 2): swap → left=5, right=4
        Result: 1(3, 2(5, 4))

    Time Complexity: O(n) - visit every node
    Space Complexity: O(h) - recursion stack (h = height)
        Why h? Each recursive call stays on the stack until it returns.
        The deepest the stack gets = longest root-to-leaf path = height.
        Balanced tree: h = log n. Skewed tree: h = n.
    """
    if not root:
        return None

    root.left, root.right = root.right, root.left
    invert_tree(root.left)
    invert_tree(root.right)

    return root


# -----------------------------------------------------------------------------
# Tests for invert_tree
# -----------------------------------------------------------------------------

def test_invert_tree_example():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3))
    expected = tree(1, tree(3), tree(2, tree(5), tree(4)))
    assert invert_tree(root) == expected


def test_invert_tree_empty():
    assert invert_tree(None) is None


def test_invert_tree_single():
    assert invert_tree(tree(1)) == tree(1)


def test_invert_tree_two_levels():
    assert invert_tree(tree(1, tree(2), tree(3))) == tree(1, tree(3), tree(2))


def test_invert_tree_left_only():
    assert invert_tree(tree(1, tree(2))) == tree(1, None, tree(2))


def test_invert_tree_right_only():
    assert invert_tree(tree(1, None, tree(2))) == tree(1, tree(2))


def test_invert_tree_twice():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3))
    original = tree(1, tree(2, tree(4), tree(5)), tree(3))
    assert invert_tree(invert_tree(root)) == original


def is_balanced(root: TreeNode | None) -> bool:
    """
    Balanced Binary Tree Validation

    A tree is balanced if no node's subtrees differ in height by more than 1.

    Example:
        Balanced:       Not balanced:
            1               1
           / \\             /
          2   3            2
         / \\             /
        4   5            3

    Return height from each subtree. If any subtree is unbalanced,
    propagate -1 up as a "poisoned" signal.

    helper(node) returns:
        -1    → unbalanced somewhere below
        >= 0  → height of this subtree

    Example balanced tree (1,2,3,4,5):
        helper(4)=0, helper(5)=0
        helper(2): |0-0|=0 ≤1 ✓ → return 1
        helper(3)=0
        helper(1): |1-0|=1 ≤1 ✓ → return 2 → balanced!

    Time Complexity: O(n)
    Space Complexity: O(h)
    """
    def height(node):
        if not node:
            return 0

        left = height(node.left)
        right = height(node.right)

        # Propagate -1 (unbalanced) upward
        if left == -1 or right == -1:
            return -1

        if abs(left - right) > 1:
            return -1

        return 1 + max(left, right)

    return height(root) != -1


# -----------------------------------------------------------------------------
# Tests for is_balanced
# -----------------------------------------------------------------------------

def test_is_balanced_empty():
    assert is_balanced(None) == True


def test_is_balanced_single():
    assert is_balanced(tree(1)) == True


def test_is_balanced_true():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3))
    assert is_balanced(root) == True


def test_is_balanced_false():
    root = tree(1, tree(2, tree(3)))
    assert is_balanced(root) == False


def test_is_balanced_perfect():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3, tree(6), tree(7)))
    assert is_balanced(root) == True


def test_is_balanced_unbalanced_deep():
    root = tree(1, tree(2, tree(4, tree(5))), tree(3))
    assert is_balanced(root) == False


def test_is_balanced_one_child_each():
    assert is_balanced(tree(1, tree(2), tree(3))) == True


def rightmost_nodes(root: TreeNode | None) -> list[int]:
    """
    Rightmost Nodes of a Binary Tree (Right Side View)

    Return the last node value at each level.

    Example:
            1
           / \\
          2   3
         / \\   \\
        4   5   6

        → [1, 3, 6]

    BFS level by level. The last node in each level is the rightmost.

    Use a queue. Process one level at a time:
        level_size = len(queue)
        for i in range(level_size):
            node = queue.popleft()
            if i == level_size - 1: → this is the rightmost!
            add children to queue

    Time Complexity: O(n) - visit every node
    Space Complexity: O(w) - max width of tree (queue)
    """
    if not root:
        return []

    result = []
    queue = deque([root])

    while queue:
        level_size = len(queue)
        for i in range(level_size):
            node = queue.popleft()
            if i == level_size - 1:
                result.append(node.val)
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)

    return result


# -----------------------------------------------------------------------------
# Tests for rightmost_nodes
# -----------------------------------------------------------------------------

def test_rightmost_nodes_example():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3, None, tree(6)))
    assert rightmost_nodes(root) == [1, 3, 6]


def test_rightmost_nodes_empty():
    assert rightmost_nodes(None) == []


def test_rightmost_nodes_single():
    assert rightmost_nodes(tree(1)) == [1]


def test_rightmost_nodes_left_only():
    root = tree(1, tree(2, tree(3)))
    assert rightmost_nodes(root) == [1, 2, 3]


def test_rightmost_nodes_right_only():
    root = tree(1, None, tree(2, None, tree(3)))
    assert rightmost_nodes(root) == [1, 2, 3]


def test_rightmost_nodes_zigzag():
    root = tree(1, tree(2, None, tree(3)))
    assert rightmost_nodes(root) == [1, 2, 3]


def is_valid_bst(root: TreeNode | None) -> bool:
    """
    Binary Search Tree Validation

    Check if a binary tree is a valid BST:
        - Left subtree values < node value
        - Right subtree values > node value
        - This must hold for ALL ancestors, not just the parent

    Example:
            5
           / \\
          3   7       → True (all constraints satisfied)
         / \\ / \\
        2  4 6  8

            5
           / \\
          3   7       → False (6 in left subtree of 5!)
         / \\ / \\
        2  6 7  8

    Pass bounds down: each node must be in range (lower, upper).
    Go left  → update upper bound to current val
    Go right → update lower bound to current val

    Example valid tree:
        check(5, -inf, inf): 5 in range ✓
          check(3, -inf, 5): 3 in range ✓
            check(2, -inf, 3): 2 in range ✓
            check(4, 3, 5):    4 in range ✓
          check(7, 5, inf): 7 in range ✓

    Time Complexity: O(n)
    Space Complexity: O(h)
    """
    def check(node, lower, upper):
        if not node:
            return True

        if node.val <= lower or node.val >= upper:
            return False

        return (check(node.left, lower, node.val)
                and check(node.right, node.val, upper))

    return check(root, float('-inf'), float('inf'))


# -----------------------------------------------------------------------------
# Tests for is_valid_bst
# -----------------------------------------------------------------------------

def test_is_valid_bst_example_invalid():
    root = tree(5, tree(3, tree(2), tree(6)), tree(7, tree(7), tree(8)))
    assert is_valid_bst(root) == False


def test_is_valid_bst_valid():
    root = tree(5, tree(3, tree(2), tree(4)), tree(7, tree(6), tree(8)))
    assert is_valid_bst(root) == True


def test_is_valid_bst_empty():
    assert is_valid_bst(None) == True


def test_is_valid_bst_single():
    assert is_valid_bst(tree(1)) == True


def test_is_valid_bst_left_valid():
    assert is_valid_bst(tree(5, tree(3))) == True


def test_is_valid_bst_left_invalid():
    assert is_valid_bst(tree(5, tree(7))) == False


def test_is_valid_bst_right_valid():
    assert is_valid_bst(tree(5, None, tree(7))) == True


def test_is_valid_bst_right_invalid():
    assert is_valid_bst(tree(5, None, tree(3))) == False


def test_is_valid_bst_subtree_violation():
    # 6 is valid child of 3, but violates 5's left subtree constraint
    root = tree(5, tree(3, None, tree(6)))
    assert is_valid_bst(root) == False


def test_is_valid_bst_equal_values():
    assert is_valid_bst(tree(5, tree(5))) == False


def lowest_common_ancestor(root: TreeNode | None, p: int, q: int) -> TreeNode | None:
    """
    Lowest Common Ancestor (LCA)

    Find the lowest node that has both p and q as descendants.
    A node can be its own ancestor.

    Example:
            1
           / \\
          2   3
         / \\
        4   5

        LCA(4, 5) = 2    (both under 2)
        LCA(4, 3) = 1    (split across root)
        LCA(2, 4) = 2    (2 is ancestor of 4)

    Post-order DFS: search left and right subtrees.
        - If current node is p or q → return it
        - If left AND right both found something → current node is the LCA
        - If only one side found → propagate that result up

    Example LCA(4, 5) on tree above:
        at node 4: val==4 → return 4
        at node 5: val==5 → return 5
        at node 2: left=4, right=5 → BOTH found → return 2 (LCA!)
        at node 3: left=None, right=None → return None
        at node 1: left=2, right=None → return 2

    Time Complexity: O(n)
    Space Complexity: O(h)
    """
    if not root:
        return None

    if root.val == p or root.val == q:
        return root

    left = lowest_common_ancestor(root.left, p, q)
    right = lowest_common_ancestor(root.right, p, q)

    if left and right:
        return root  # this node is the LCA

    return left if left else right


# -----------------------------------------------------------------------------
# Tests for lowest_common_ancestor
# -----------------------------------------------------------------------------

def test_lca_siblings():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3))
    assert lowest_common_ancestor(root, 4, 5).val == 2


def test_lca_different_subtrees():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3))
    assert lowest_common_ancestor(root, 4, 3).val == 1


def test_lca_ancestor_is_node():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3))
    assert lowest_common_ancestor(root, 2, 4).val == 2


def test_lca_root_is_answer():
    root = tree(1, tree(2), tree(3))
    assert lowest_common_ancestor(root, 2, 3).val == 1


def test_lca_deep_tree():
    root = tree(1, tree(2, tree(3, tree(4))))
    assert lowest_common_ancestor(root, 3, 4).val == 3


def test_lca_right_subtree():
    root = tree(1, None, tree(2, tree(3), tree(4)))
    assert lowest_common_ancestor(root, 3, 4).val == 2


def build_tree(preorder: list[int], inorder: list[int]) -> TreeNode | None:
    """
    Build Binary Tree from Preorder and Inorder Traversals

    Given preorder and inorder traversal arrays, reconstruct the original
    binary tree. Each value is unique.

    Example:
        preorder = [5, 9, 2, 3, 4, 7]
        inorder  = [2, 9, 5, 4, 3, 7]

        Result:
              5
             / \\
            9   3
           /   / \\
          2   4   7

    Key insight: preorder[0] is always the root. Find that root in the
    inorder array -- everything to its left is the left subtree, everything
    to its right is the right subtree. Then recurse on each side. Use a
    hash map for O(1) lookup of inorder indices.

    Example: preorder=[5,9,2,3,4,7], inorder=[2,9,5,4,3,7]
        root = preorder[0] = 5
        find 5 in inorder at index 2
        left subtree inorder:  [2, 9]     (indices 0..1)
        right subtree inorder: [4, 3, 7]  (indices 3..5)
        next preorder element for left subtree: 9
        next preorder element for right subtree: 3
        Recurse on each side...

    Time Complexity: O(n)
    Space Complexity: O(n) - hash map + recursion

    Use a shared preorder index that increments as we consume elements.
    """
    if not preorder:
        return None

    # Map inorder values to indices for O(1) lookup
    inorder_idx = {val: i for i, val in enumerate(inorder)}
    pre_i = [0]  # shared mutable index into preorder (list so inner fn can modify)

    def helper(left, right):
        if left > right:
            return None

        val = preorder[pre_i[0]]
        pre_i[0] += 1

        mid = inorder_idx[val]
        node = TreeNode(val)
        node.left = helper(left, mid - 1)    # build left subtree first
        node.right = helper(mid + 1, right)   # then right subtree

        return node

    return helper(0, len(inorder) - 1)


# -----------------------------------------------------------------------------
# Tests for build_tree
# -----------------------------------------------------------------------------

def test_build_tree_example():
    expected = tree(5, tree(9, tree(2)), tree(3, tree(4), tree(7)))
    assert build_tree([5, 9, 2, 3, 4, 7], [2, 9, 5, 4, 3, 7]) == expected


def test_build_tree_empty():
    assert build_tree([], []) is None


def test_build_tree_single():
    assert build_tree([1], [1]) == tree(1)


def test_build_tree_left_only():
    expected = tree(1, tree(2, tree(3)))
    assert build_tree([1, 2, 3], [3, 2, 1]) == expected


def test_build_tree_right_only():
    expected = tree(1, None, tree(2, None, tree(3)))
    assert build_tree([1, 2, 3], [1, 2, 3]) == expected


def test_build_tree_balanced():
    expected = tree(1, tree(2, tree(4), tree(5)), tree(3, tree(6), tree(7)))
    assert build_tree([1, 2, 4, 5, 3, 6, 7], [4, 2, 5, 1, 6, 3, 7]) == expected


def max_path_sum(root: TreeNode | None) -> int:
    """
    Maximum Path Sum in a Binary Tree

    Find the maximum sum along any path (any node to any node).
    Path = connected sequence of nodes, no splits.

    Example:
            5
           / \\
          3   8
         / \\   \\
        4  -2   6

        Best path: 4 → 3 → 5 → 8 → 6 = 26

    At each node, we decide:
        - left_gain = max(0, best path going left)   ← 0 if negative (skip)
        - right_gain = max(0, best path going right)
        - path_through = val + left_gain + right_gain ← path using this node as "turn"
        - Update global max with path_through

    Return to parent: val + max(left_gain, right_gain)
        (can only extend in ONE direction, not both, when going up)

    Example:
        at node 4:  return 4,  path_through=4
        at node -2: return 0 (capped), path_through=-2
        at node 3:  left=4, right=0 → path_through=3+4+0=7, return 3+4=7
        at node 6:  return 6,  path_through=6
        at node 8:  left=0, right=6 → path_through=8+0+6=14, return 8+6=14
        at node 5:  left=7, right=14 → path_through=5+7+14=26 ★, return 5+14=19
        Answer: 26

    Time Complexity: O(n)
    Space Complexity: O(h)
    """
    best = [float('-inf')]  # list so inner fn can modify

    def dfs(node):
        if not node:
            return 0

        left_gain = max(0, dfs(node.left))    # 0 if negative (don't take it)
        right_gain = max(0, dfs(node.right))

        # Path through this node as the "turning point"
        best[0] = max(best[0], node.val + left_gain + right_gain)

        # Return best single-direction path to parent
        return node.val + max(left_gain, right_gain)

    dfs(root)
    return best[0]


# -----------------------------------------------------------------------------
# Tests for max_path_sum
# -----------------------------------------------------------------------------

def test_max_path_sum_example():
    root = tree(5, tree(3, tree(4), tree(-2)), tree(8, None, tree(6)))
    assert max_path_sum(root) == 26


def test_max_path_sum_single():
    assert max_path_sum(tree(5)) == 5


def test_max_path_sum_negative_single():
    assert max_path_sum(tree(-3)) == -3


def test_max_path_sum_all_negative():
    assert max_path_sum(tree(-3, tree(-2), tree(-1))) == -1


def test_max_path_sum_skip_subtree():
    root = tree(10, tree(2), tree(-25, tree(3), tree(4)))
    assert max_path_sum(root) == 12


def test_max_path_sum_through_root():
    assert max_path_sum(tree(1, tree(2), tree(3))) == 6


def test_max_path_sum_not_through_root():
    root = tree(-10, tree(9), tree(20, tree(15), tree(7)))
    assert max_path_sum(root) == 42


def widest_level(root: TreeNode | None) -> int:
    """
    Widest Binary Tree Level

    Return the width of the widest level, including null gaps.

    Example:
            1
           / \\
          2   3
         /     \\
        4       5

        Level 2 has nodes at positions 0 and 3 → width = 4
        (positions: 4, null, null, 5)

    Assign position indices like a heap array:
        - root = 0
        - left child = 2*i + 1
        - right child = 2*i + 2

    Width of a level = rightmost_pos - leftmost_pos + 1

    Example:
        Level 0: node 1 at pos 0           → width = 1
        Level 1: node 2 at pos 1, 3 at pos 2  → width = 2
        Level 2: node 4 at pos 3, 5 at pos 6  → width = 6-3+1 = 4 ★

    Time Complexity: O(n)
    Space Complexity: O(w) - max width (queue)
    """
    if not root:
        return 0

    max_width = 0
    queue = deque([(root, 0)])  # (node, position index)

    while queue:
        level_size = len(queue)
        left_idx = queue[0][1]
        right_idx = queue[-1][1]
        max_width = max(max_width, right_idx - left_idx + 1)

        for _ in range(level_size):
            node, i = queue.popleft()
            if node.left:
                queue.append((node.left, 2 * i + 1))
            if node.right:
                queue.append((node.right, 2 * i + 2))

    return max_width


# -----------------------------------------------------------------------------
# Tests for widest_level
# -----------------------------------------------------------------------------

def test_widest_level_example():
    root = tree(1, tree(2, tree(4)), tree(3, None, tree(5)))
    assert widest_level(root) == 4


def test_widest_level_empty():
    assert widest_level(None) == 0


def test_widest_level_single():
    assert widest_level(tree(1)) == 1


def test_widest_level_full():
    root = tree(1, tree(2, tree(4), tree(5)), tree(3, tree(6), tree(7)))
    assert widest_level(root) == 4


def test_widest_level_left_heavy():
    root = tree(1, tree(2, tree(3)))
    assert widest_level(root) == 1


def test_widest_level_two_levels():
    assert widest_level(tree(1, tree(2), tree(3))) == 2


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
