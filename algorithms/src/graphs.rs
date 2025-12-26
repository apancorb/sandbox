use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
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

/// Matrix Infection
///
/// You are given a matrix where each cell is either:
/// - 0: Empty
/// - 1: Uninfected
/// - 2: Infected
///
/// With each passing second, every infected cell (2) infects its uninfected neighboring cells
/// (1) that are 4-directionally adjacent. Determine the number of seconds required for all
/// uninfected cells to become infected. If this is impossible, return -1.
///
/// # Example
///
/// ```text
/// Input: matrix = [[1, 1, 1, 0],
///                  [0, 0, 2, 1],
///                  [0, 1, 1, 0]]
///
/// Second 0:    Second 1:    Second 2:    Second 3:
/// 1 1 1 0      1 1 2 0      1 2 2 0      2 2 2 0
/// 0 0 2 1  ->  0 0 2 2  ->  0 0 2 2  ->  0 0 2 2
/// 0 1 1 0      0 1 2 0      0 2 2 0      0 2 2 0
///
/// Output: 3
/// ```
pub fn matrix_infection(matrix: &mut [&mut [i32]]) -> i32 {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ones = 0;
    let mut deque = VecDeque::new();

    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if matrix[r][c] == 1 {
                ones += 1;
            } else if matrix[r][c] == 2 {
                deque.push_back((r, c));
            }
        }
    }

    let mut seconds = 0;
    while ones != 0 && !deque.is_empty() {
        let level_size = deque.len();
        for _ in 0..level_size {
            let (r, c) = deque.pop_front().unwrap();
            for (dr, dc) in DIRS {
                let Some(next_r) = r.checked_add_signed(dr) else {
                    continue;
                };
                let Some(next_c) = c.checked_add_signed(dc) else {
                    continue;
                };

                if next_r < matrix.len() && next_c < matrix[0].len() && matrix[next_r][next_c] == 1
                {
                    matrix[next_r][next_c] = 2;
                    deque.push_back((next_r, next_c));
                    ones -= 1;
                }
            }
        }

        seconds += 1;
    }

    if ones != 0 { -1 } else { seconds }
}

/// Bipartite Graph Validation
///
/// Given an undirected graph, determine if it's bipartite. A graph is bipartite if the nodes can
/// be colored in one of two colors, so that no two adjacent nodes are the same color.
///
/// The input is presented as an adjacency list, where graph[i] is a list of all nodes adjacent to
/// node i.
///
/// # Example
///
/// ```text
/// Input: graph = [[1, 4], [0, 2], [1], [4], [0, 3]]
///
///     0 (blue) --- 1 (orange) --- 2 (blue)
///     |
///     4 (orange) --- 3 (blue)
///
/// Output: true
/// Explanation: Nodes can be colored with two colors such that no adjacent nodes share a color.
/// ```
pub fn is_bipartite(graph: &[Vec<usize>]) -> bool {
    fn is_bipartite_helper(
        graph: &[Vec<usize>],
        colors: &mut Vec<i32>,
        node: usize,
        color: i32,
    ) -> bool {
        colors[node] = color;

        for &neighbor in &graph[node] {
            if colors[neighbor] == color {
                return false;
            }

            if colors[neighbor] == 0 && !is_bipartite_helper(graph, colors, neighbor, -color) {
                return false;
            }
        }

        true
    }

    let mut colors = vec![0; graph.len()];
    for i in 0..colors.len() {
        if colors[i] == 0 && !is_bipartite_helper(graph, &mut colors, i, 1) {
            return false;
        }
    }

    true
}

/// Longest Increasing Path
///
/// Find the longest strictly increasing path in a matrix of positive integers. A path is a
/// sequence of cells where each one is 4-directionally adjacent (up, down, left, or right) to
/// the previous one.
///
/// # Example
///
/// ```text
/// Input: matrix = [[2, 7, 9],
///                  [5, 4, 3],
///                  [6, 1, 8]]
///
/// Output: 4
/// Explanation:
///   Positions:
///     2(0,0) 7(0,1) 9(0,2)
///     5(1,0) 4(1,1) 3(1,2)
///     6(2,0) 1(2,1) 8(2,2)
///
///   Longest paths of length 4:
///   - 1 -> 4 -> 5 -> 6
///   - 1 -> 4 -> 7 -> 9
/// ```
pub fn longest_increasing_path(matrix: &[&[i32]]) -> i32 {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn longest_increasing_path_helper(
        matrix: &[&[i32]],
        memo: &mut Vec<Vec<i32>>,
        r: usize,
        c: usize,
    ) -> i32 {
        if memo[r][c] != 0 {
            return memo[r][c];
        }

        let mut max = 1;
        for (dr, dc) in DIRS {
            let Some(next_r) = r.checked_add_signed(dr) else {
                continue;
            };
            let Some(next_c) = c.checked_add_signed(dc) else {
                continue;
            };

            if next_r < matrix.len()
                && next_c < matrix[0].len()
                && matrix[next_r][next_c] > matrix[r][c]
            {
                max = max.max(1 + longest_increasing_path_helper(matrix, memo, next_r, next_c));
            }
        }

        memo[r][c] = max;
        max
    }

    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let mut res = 0;
    let mut memo = vec![vec![0; matrix[0].len()]; matrix.len()];
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            res = res.max(longest_increasing_path_helper(matrix, &mut memo, r, c));
        }
    }

    res
}

/// Shortest Transformation Sequence
///
/// Given two words, start and end, and a dictionary containing an array of words, return the
/// length of the shortest transformation sequence to transform start to end. A transformation
/// sequence is a series of words in which:
/// - Each word differs from the preceding word by exactly one letter.
/// - Each word in the sequence exists in the dictionary.
///
/// If no such transformation sequence exists, return 0.
///
/// # Example
///
/// ```text
/// Input: start = "red", end = "hit", dictionary = ["red", "rod", "rad", "rat", "hat", "bad", "bat", "hit"]
///
/// Transformation: red -> rad -> rat -> hat -> hit
///
/// Output: 5
/// ```
///
/// # Constraints
///
/// - All words are the same length.
/// - All words contain only lowercase English letters.
/// - The dictionary contains no duplicate words.
pub fn shortest_transformation(start: &str, end: &str, dictionary: &[&str]) -> i32 {
    if start == end {
        return 1;
    }

    let dictionary: HashSet<String> = dictionary.iter().map(|s| s.to_string()).collect();

    if !dictionary.contains(end) {
        return 0;
    }

    let mut dist = 1;
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.to_string());
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(start.to_string());

    while !queue.is_empty() {
        let level_size = queue.len();
        for _ in 0..level_size {
            let word = queue.pop_front().unwrap();

            let chars: Vec<char> = word.chars().collect();
            for i in 0..chars.len() {
                for c in 'a'..='z' {
                    if chars[i] != c {
                        let mut next_chars = chars.clone();
                        next_chars[i] = c;
                        let next_word: String = next_chars.into_iter().collect();

                        if next_word == end {
                            return dist + 1;
                        }

                        if !visited.contains(&next_word) && dictionary.contains(&next_word) {
                            visited.insert(next_word.clone());
                            queue.push_back(next_word);
                        }
                    }
                }
            }
        }
        dist += 1;
    }

    0
}

/// Prerequisites
///
/// Given an integer n representing the number of courses labeled from 0 to n - 1, and an
/// array of prerequisite pairs, determine if it's possible to enroll in all courses.
///
/// Each prerequisite is represented as a pair [a, b], indicating that course a must be taken
/// before course b.
///
/// # Example
///
/// ```text
/// Input: n = 3, prerequisites = [[0, 1], [1, 2], [2, 1]]
///
/// Output: false
/// Explanation: Course 1 cannot be taken without first completing course 2, and vice versa.
/// ```
///
/// # Constraints
///
/// - For any prerequisite [a, b], a will not equal b.
pub fn can_finish(n: usize, prerequisites: &[[usize; 2]]) -> bool {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degrees = vec![0; n];

    for prerequisite in prerequisites {
        let prereq = prerequisite[0];
        let course = prerequisite[1];

        graph.entry(prereq).or_default().push(course);
        in_degrees[course] += 1;
    }

    let mut queue = VecDeque::new();
    for (course, &in_degree) in in_degrees.iter().enumerate() {
        if in_degree == 0 {
            queue.push_back(course);
        }
    }

    let mut enrolled_courses = 0;
    // Perform topological sort
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        enrolled_courses += 1;
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                in_degrees[neighbor] -= 1;
                if in_degrees[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    enrolled_courses == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_finish_example() {
        // 0 -> 1 -> 2
        //      ^----+  (cycle between 1 and 2)
        assert!(!can_finish(3, &[[0, 1], [1, 2], [2, 1]]));
    }

    #[test]
    fn test_can_finish_no_cycle() {
        // 0 -> 1 -> 2
        assert!(can_finish(3, &[[0, 1], [1, 2]]));
    }

    #[test]
    fn test_can_finish_no_prereqs() {
        assert!(can_finish(3, &[]));
    }

    #[test]
    fn test_can_finish_single_course() {
        assert!(can_finish(1, &[]));
    }

    #[test]
    fn test_can_finish_diamond() {
        //   0
        //  / \
        // 1   2
        //  \ /
        //   3
        assert!(can_finish(4, &[[0, 1], [0, 2], [1, 3], [2, 3]]));
    }

    #[test]
    fn test_can_finish_self_loop_impossible() {
        // Constraint says a != b, but testing cycle detection
        // 0 -> 1 -> 0
        assert!(!can_finish(2, &[[0, 1], [1, 0]]));
    }

    #[test]
    fn test_can_finish_disconnected() {
        // 0 -> 1    2 -> 3
        assert!(can_finish(4, &[[0, 1], [2, 3]]));
    }

    #[test]
    fn test_shortest_transformation_example() {
        let dictionary = &["red", "rod", "rad", "rat", "hat", "bad", "bat", "hit"];
        assert_eq!(shortest_transformation("red", "hit", dictionary), 5);
    }

    #[test]
    fn test_shortest_transformation_no_path() {
        let dictionary = &["red", "rod", "rad"];
        assert_eq!(shortest_transformation("red", "hit", dictionary), 0);
    }

    #[test]
    fn test_shortest_transformation_one_step() {
        let dictionary = &["hit", "hot"];
        assert_eq!(shortest_transformation("hit", "hot", dictionary), 2);
    }

    #[test]
    fn test_shortest_transformation_same_word() {
        let dictionary = &["hit"];
        assert_eq!(shortest_transformation("hit", "hit", dictionary), 1);
    }

    #[test]
    fn test_shortest_transformation_direct() {
        // red -> rad is 1 step, sequence length is 2
        let dictionary = &["red", "rod", "rad"];
        assert_eq!(shortest_transformation("red", "rad", dictionary), 2);
    }

    #[test]
    fn test_shortest_transformation_multiple_paths() {
        // hit -> hot -> dot -> dog
        // hit -> hot -> lot -> log -> dog (longer)
        let dictionary = &["hit", "hot", "dot", "dog", "lot", "log"];
        assert_eq!(shortest_transformation("hit", "dog", dictionary), 4);
    }

    #[test]
    fn test_longest_increasing_path_example() {
        let matrix: &[&[i32]] = &[&[2, 7, 9], &[5, 4, 3], &[6, 1, 8]];
        assert_eq!(longest_increasing_path(matrix), 4);
    }

    #[test]
    fn test_longest_increasing_path_single() {
        let matrix: &[&[i32]] = &[&[1]];
        assert_eq!(longest_increasing_path(matrix), 1);
    }

    #[test]
    fn test_longest_increasing_path_row() {
        let matrix: &[&[i32]] = &[&[1, 2, 3, 4, 5]];
        assert_eq!(longest_increasing_path(matrix), 5);
    }

    #[test]
    fn test_longest_increasing_path_decreasing() {
        let matrix: &[&[i32]] = &[&[5, 4, 3, 2, 1]];
        assert_eq!(longest_increasing_path(matrix), 5);
    }

    #[test]
    fn test_longest_increasing_path_column() {
        let matrix: &[&[i32]] = &[&[1], &[2], &[3], &[4]];
        assert_eq!(longest_increasing_path(matrix), 4);
    }

    #[test]
    fn test_longest_increasing_path_all_same() {
        let matrix: &[&[i32]] = &[&[1, 1], &[1, 1]];
        assert_eq!(longest_increasing_path(matrix), 1);
    }

    #[test]
    fn test_longest_increasing_path_spiral() {
        // 1 2 3
        // 8 9 4
        // 7 6 5
        let matrix: &[&[i32]] = &[&[1, 2, 3], &[8, 9, 4], &[7, 6, 5]];
        assert_eq!(longest_increasing_path(matrix), 9);
    }

    #[test]
    fn test_longest_increasing_path_empty() {
        let matrix: &[&[i32]] = &[];
        assert_eq!(longest_increasing_path(matrix), 0);
    }

    #[test]
    fn test_is_bipartite_example() {
        // 0 -- 1 -- 2
        // |
        // 4 -- 3
        let graph = vec![vec![1, 4], vec![0, 2], vec![1], vec![4], vec![0, 3]];
        assert!(is_bipartite(&graph));
    }

    #[test]
    fn test_is_bipartite_triangle() {
        // 0 -- 1
        //  \  /
        //   2
        // Odd cycle - not bipartite
        let graph = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert!(!is_bipartite(&graph));
    }

    #[test]
    fn test_is_bipartite_square() {
        // 0 -- 1
        // |    |
        // 3 -- 2
        // Even cycle - bipartite
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert!(is_bipartite(&graph));
    }

    #[test]
    fn test_is_bipartite_disconnected() {
        // 0 -- 1    2 -- 3
        // Two disconnected edges - bipartite
        let graph = vec![vec![1], vec![0], vec![3], vec![2]];
        assert!(is_bipartite(&graph));
    }

    #[test]
    fn test_is_bipartite_single_node() {
        let graph = vec![vec![]];
        assert!(is_bipartite(&graph));
    }

    #[test]
    fn test_is_bipartite_empty() {
        let graph: Vec<Vec<usize>> = vec![];
        assert!(is_bipartite(&graph));
    }

    #[test]
    fn test_is_bipartite_disconnected_with_odd_cycle() {
        // 0 -- 1    2 -- 3
        //           |  /
        //           4
        // Second component has odd cycle
        let graph = vec![vec![1], vec![0], vec![3, 4], vec![2, 4], vec![2, 3]];
        assert!(!is_bipartite(&graph));
    }

    #[test]
    fn test_matrix_infection_example() {
        let mut row0 = [1, 1, 1, 0];
        let mut row1 = [0, 0, 2, 1];
        let mut row2 = [0, 1, 1, 0];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0, &mut row1, &mut row2];
        assert_eq!(matrix_infection(matrix), 3);
    }

    #[test]
    fn test_matrix_infection_impossible() {
        // Uninfected cell isolated by empty cells
        let mut row0 = [1, 0, 2];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0];
        assert_eq!(matrix_infection(matrix), -1);
    }

    #[test]
    fn test_matrix_infection_already_done() {
        // No uninfected cells
        let mut row0 = [2, 0, 2];
        let mut row1 = [0, 0, 0];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0, &mut row1];
        assert_eq!(matrix_infection(matrix), 0);
    }

    #[test]
    fn test_matrix_infection_one_second() {
        let mut row0 = [2, 1];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0];
        assert_eq!(matrix_infection(matrix), 1);
    }

    #[test]
    fn test_matrix_infection_multiple_sources() {
        // Two infected cells spread simultaneously
        let mut row0 = [2, 1, 1, 1, 2];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0];
        assert_eq!(matrix_infection(matrix), 2);
    }

    #[test]
    fn test_matrix_infection_empty_matrix() {
        let matrix: &mut [&mut [i32]] = &mut [];
        assert_eq!(matrix_infection(matrix), 0);
    }

    #[test]
    fn test_matrix_infection_no_infected() {
        // Has uninfected but no infected cells
        let mut row0 = [1, 1];
        let matrix: &mut [&mut [i32]] = &mut [&mut row0];
        assert_eq!(matrix_infection(matrix), -1);
    }

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
