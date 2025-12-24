use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Graph Deep Copy
///
/// Given a reference to a node within an undirected graph, create a deep copy (clone) of the
/// graph. The copied graph must be completely independent of the original one. This means
/// you need to make new nodes for the copied graph instead of reusing any nodes from the
/// original graph.
///
/// # Example
///
/// ```text
/// Original Graph:
///     1 --- 2
///     |     |
///     4 --- 3 1
///
/// Cloned Graph:
///     1' --- 2'
///     |      |
///     4' --- 3'
///
/// All nodes are deep copies: no references to the original graph.
/// ```
///
/// # Constraints
///
/// - The value of each node is unique.
/// - Every node in the graph is reachable from the given node.

type GraphNode = Option<Rc<RefCell<Node>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}

pub fn clone_graph(node: GraphNode) -> GraphNode {
    fn clone_graph_helper(
        node: &Rc<RefCell<Node>>,
        map: &mut HashMap<i32, Rc<RefCell<Node>>>,
    ) -> Rc<RefCell<Node>> {
        let val = node.borrow().val;

        if let Some(cloned_node) = map.get(&val) {
            return Rc::clone(cloned_node);
        }

        let cloned_node = Rc::new(RefCell::new(Node::new(val)));
        map.insert(val, Rc::clone(&cloned_node));

        for neighbor in &node.borrow().neighbors {
            let cloned_neighbor = clone_graph_helper(neighbor, map);
            cloned_node.borrow_mut().neighbors.push(cloned_neighbor);
        }

        cloned_node
    }

    if let Some(node) = node {
        Some(clone_graph_helper(&node, &mut HashMap::new()))
    } else {
        None
    }
}

/// Count Islands
///
/// Given a binary matrix representing 1s as land and 0s as water, return the number of islands.
/// An island is formed by connecting adjacent lands 4-directionally (up, down, left, and right).
///
/// # Example
///
/// ```text
/// Input: matrix = [[1, 1, 0, 0],
///                  [1, 1, 0, 0],
///                  [0, 0, 1, 1],
///                  [0, 0, 1, 1]]
///
/// Output: 2
///
/// Explanation:
/// There are two islands:
/// - Top-left 2x2 block of 1s
/// - Bottom-right 2x2 block of 1s
/// ```
pub fn count_islands(matrix: &mut [&mut [i32]]) -> i32 {
    fn count_islands_helper(matrix: &mut [&mut [i32]], r: usize, c: usize) {
        const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        matrix[r][c] = -1;

        for (dr, dc) in DIRS {
            let Some(next_r) = r.checked_add_signed(dr) else {
                continue;
            };
            let Some(next_c) = c.checked_add_signed(dc) else {
                continue;
            };

            if next_r < matrix.len() && next_c < matrix[0].len() && matrix[next_r][next_c] == 1 {
                count_islands_helper(matrix, next_r, next_c);
            }
        }
    }

    let mut counter = 0;
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if matrix[r][c] == 1 {
                count_islands_helper(matrix, r, c);
                counter += 1;
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_node(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(val)))
    }

    fn connect(a: &Rc<RefCell<Node>>, b: &Rc<RefCell<Node>>) {
        a.borrow_mut().neighbors.push(Rc::clone(b));
        b.borrow_mut().neighbors.push(Rc::clone(a));
    }

    #[test]
    fn test_count_islands_example() {
        let mut row0 = [1, 1, 0, 0];
        let mut row1 = [1, 1, 0, 0];
        let mut row2 = [0, 0, 1, 1];
        let mut row3 = [0, 0, 1, 1];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0, &mut row1, &mut row2, &mut row3];
        assert_eq!(count_islands(matrix), 2);
    }

    #[test]
    fn test_count_islands_single_island() {
        let mut row0 = [1, 1, 1];
        let mut row1 = [1, 1, 1];
        let mut row2 = [1, 1, 1];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0, &mut row1, &mut row2];
        assert_eq!(count_islands(matrix), 1);
    }

    #[test]
    fn test_count_islands_no_islands() {
        let mut row0 = [0, 0, 0];
        let mut row1 = [0, 0, 0];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0, &mut row1];
        assert_eq!(count_islands(matrix), 0);
    }

    #[test]
    fn test_count_islands_diagonal_not_connected() {
        // Diagonal cells are NOT connected
        let mut row0 = [1, 0];
        let mut row1 = [0, 1];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0, &mut row1];
        assert_eq!(count_islands(matrix), 2);
    }

    #[test]
    fn test_count_islands_many_small() {
        let mut row0 = [1, 0, 1, 0, 1];
        let mut row1 = [0, 0, 0, 0, 0];
        let mut row2 = [1, 0, 1, 0, 1];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0, &mut row1, &mut row2];
        assert_eq!(count_islands(matrix), 6);
    }

    #[test]
    fn test_count_islands_single_cell() {
        let mut row0 = [1];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0];
        assert_eq!(count_islands(matrix), 1);
    }

    #[test]
    fn test_count_islands_empty() {
        let matrix: &mut [&mut [i32]] = &mut [];
        assert_eq!(count_islands(matrix), 0);
    }

    #[test]
    fn test_clone_graph_example() {
        // Create: 1 -- 2
        //         |    |
        //         4 -- 3
        let n1 = create_node(1);
        let n2 = create_node(2);
        let n3 = create_node(3);
        let n4 = create_node(4);

        connect(&n1, &n2);
        connect(&n2, &n3);
        connect(&n3, &n4);
        connect(&n4, &n1);

        let cloned = clone_graph(Some(Rc::clone(&n1)));

        // Verify clone exists
        assert!(cloned.is_some());
        let cloned = cloned.unwrap();

        // Verify it's a different node (not same reference)
        assert!(!Rc::ptr_eq(&n1, &cloned));

        // Verify value is same
        assert_eq!(cloned.borrow().val, 1);

        // Verify neighbors count
        assert_eq!(cloned.borrow().neighbors.len(), 2);

        // Verify neighbor values (should be 2 and 4)
        let neighbor_vals: Vec<i32> = cloned
            .borrow()
            .neighbors
            .iter()
            .map(|n| n.borrow().val)
            .collect();
        assert!(neighbor_vals.contains(&2));
        assert!(neighbor_vals.contains(&4));
    }

    #[test]
    fn test_clone_graph_empty() {
        let cloned = clone_graph(None);
        assert!(cloned.is_none());
    }

    #[test]
    fn test_clone_graph_single_node() {
        let n1 = create_node(1);
        let cloned = clone_graph(Some(Rc::clone(&n1)));

        assert!(cloned.is_some());
        let cloned = cloned.unwrap();
        assert!(!Rc::ptr_eq(&n1, &cloned));
        assert_eq!(cloned.borrow().val, 1);
        assert!(cloned.borrow().neighbors.is_empty());
    }

    #[test]
    fn test_clone_graph_two_nodes() {
        let n1 = create_node(1);
        let n2 = create_node(2);
        connect(&n1, &n2);

        let cloned = clone_graph(Some(Rc::clone(&n1)));

        assert!(cloned.is_some());
        let cloned = cloned.unwrap();
        assert_eq!(cloned.borrow().val, 1);
        assert_eq!(cloned.borrow().neighbors.len(), 1);
        assert_eq!(cloned.borrow().neighbors[0].borrow().val, 2);

        // Verify the neighbor's neighbor points back
        let cloned_n2 = Rc::clone(&cloned.borrow().neighbors[0]);
        assert_eq!(cloned_n2.borrow().neighbors.len(), 1);
        assert_eq!(cloned_n2.borrow().neighbors[0].borrow().val, 1);

        // Verify circular reference works (n2's neighbor is n1 clone)
        assert!(Rc::ptr_eq(&cloned, &cloned_n2.borrow().neighbors[0]));
    }

    #[test]
    fn test_clone_graph_triangle() {
        // 1 -- 2
        //  \  /
        //   3
        let n1 = create_node(1);
        let n2 = create_node(2);
        let n3 = create_node(3);

        connect(&n1, &n2);
        connect(&n2, &n3);
        connect(&n3, &n1);

        let cloned = clone_graph(Some(Rc::clone(&n1)));

        assert!(cloned.is_some());
        let cloned = cloned.unwrap();
        assert_eq!(cloned.borrow().val, 1);
        assert_eq!(cloned.borrow().neighbors.len(), 2);
    }

    #[test]
    fn test_clone_graph_no_shared_references() {
        let n1 = create_node(1);
        let n2 = create_node(2);
        connect(&n1, &n2);

        let cloned = clone_graph(Some(Rc::clone(&n1)));
        let cloned = cloned.unwrap();

        // Modify original
        n1.borrow_mut().val = 100;

        // Clone should be unaffected
        assert_eq!(cloned.borrow().val, 1);
    }
}
