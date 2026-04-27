use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

/// Graph Deep Copy
///
/// Deep copy an undirected graph. Each node has val and neighbors list.
///
/// # Examples
///
/// ```text
/// Input:  1 --- 2
///         |     |
///         4 --- 3
/// Output: deep copy with same structure, no shared nodes
/// ```
///
/// DFS: clone current node, recursively clone neighbors. Use a map to avoid
/// cloning the same node twice (handles cycles).
///
/// Example walkthrough on a graph 1--2--3:
///
/// ```text
/// dfs(1): create copy1, recurse on neighbor 2
/// dfs(2): create copy2, recurse on neighbor 1 -> already cloned, return copy1
///         recurse on neighbor 3 -> create copy3, done
/// copy2.neighbors = [copy1, copy3], copy1.neighbors = [copy2]
/// ```
///
/// # Complexity
///
/// - Time: O(V + E) — visit each node and edge once
/// - Space: O(V) — hash map of old->new
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
/// Count the number of islands in a binary grid (1=land, 0=water).
/// Adjacent = up/down/left/right (not diagonal).
///
/// # Examples
///
/// ```text
/// Input:  [[1,1,0,0],
///          [1,1,0,0],
///          [0,0,1,1],
///          [0,0,1,1]]
/// Output: 2
/// ```
///
/// For each unvisited land cell, DFS to mark the whole island as visited.
/// Each DFS = one island found.
///
/// Example walkthrough on the example grid:
///
/// ```text
/// Scan (0,0): land, DFS marks (0,0),(0,1),(1,0),(1,1) -> count=1
/// Scan (0,2): water, skip
/// Scan (2,2): land, DFS marks (2,2),(2,3),(3,2),(3,3) -> count=2
/// All other cells already visited or water -> return 2
/// ```
///
/// # Complexity
///
/// - Time: O(rows * cols) — each cell visited at most once
/// - Space: O(rows * cols) — worst case recursion depth
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

/// Matrix Infection (Rotting Oranges)
///
/// 0=empty, 1=uninfected, 2=infected. Each second, infected cells spread to
/// adjacent uninfected cells. Return seconds to infect all, or -1 if impossible.
///
/// # Examples
///
/// ```text
/// Input:  [[1,1,1,0],
///          [0,0,2,1],
///          [0,1,1,0]]
/// Output: 3
/// ```
///
/// Multi-source BFS: start from ALL infected cells simultaneously.
/// Each BFS level = one second. Count remaining uninfected at end.
///
/// Example walkthrough on the example grid:
///
/// ```text
/// Initial: queue=[(1,2)], uninfected=6: (0,0),(0,1),(0,2),(1,3),(2,1),(2,2)
/// t=1: (1,2) infects (0,2),(1,3),(2,2) -> uninfected=3
/// t=2: (0,2) infects (0,1), (2,2) infects (2,1) -> uninfected=1
/// t=3: (0,1) infects (0,0) -> uninfected=0 -> return 3
/// ```
///
/// # Complexity
///
/// - Time: O(rows * cols) — each cell processed at most once
/// - Space: O(rows * cols) — queue can hold all cells
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
/// Can nodes be colored with 2 colors so no adjacent nodes share a color?
///
/// # Examples
///
/// ```text
/// Input:  graph = [[1,4],[0,2],[1],[4],[0,3]]
///         0 -- 1 -- 2
///         |
///         4 -- 3
/// Output: true (two-colorable)
/// ```
///
/// DFS coloring: assign color +1 to a node, -1 to its neighbors. If a neighbor
/// already has the SAME color, it's not bipartite. A graph is bipartite iff it
/// has no odd-length cycles.
///
/// Example walkthrough on the example graph:
///
/// ```text
/// color[0]=+1 -> neighbor 1: color[1]=-1 -> neighbor 2: color[2]=+1
/// back to 0 -> neighbor 4: color[4]=-1 -> neighbor 3: color[3]=+1
/// No conflicts found -> return True
/// ```
///
/// # Complexity
///
/// - Time: O(V + E) — visit each node and edge once
/// - Space: O(V) — color array
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

/// Longest Increasing Path in a Matrix
///
/// Find the longest strictly increasing path (4-directional moves).
///
/// # Examples
///
/// ```text
/// Input:  [[2,7,9],
///          [5,4,3],
///          [6,1,8]]
/// Output: 4 (1 -> 4 -> 5 -> 6 or 1 -> 4 -> 7 -> 9)
/// ```
///
/// DFS + memoization: from each cell, try all 4 directions where the neighbor
/// is strictly larger. Cache results to avoid recomputation.
///
/// Why memoization works: if we already know the longest path starting from
/// cell (r,c), we don't need to recompute it.
///
/// Example walkthrough starting from cell (2,1)=1:
///
/// ```text
/// dfs(2,1)=1: neighbors 4>(1) yes -> dfs(1,1)
/// dfs(1,1)=4: neighbors 7>4 yes -> dfs(0,1), 5>4 yes -> dfs(1,0)
/// dfs(0,1)=7: neighbor 9>7 yes -> dfs(0,2)=1, so dfs(0,1)=2
/// dfs(1,0)=5: neighbor 6>5 yes -> dfs(2,0)=1, so dfs(1,0)=2
/// dfs(1,1)=max(1+2, 1+2)=3, dfs(2,1)=1+3=4
/// ```
///
/// # Complexity
///
/// - Time: O(rows * cols) — each cell computed once
/// - Space: O(rows * cols) — memo table
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

/// Shortest Transformation Sequence (Word Ladder)
///
/// Find length of shortest sequence from start to end where each step changes
/// exactly one letter and the resulting word is in the dictionary.
///
/// # Examples
///
/// ```text
/// Input:  start="red", end="hit",
///         dictionary=["red","rod","rad","rat","hat","bad","bat","hit"]
/// Output: 5  (red -> rad -> rat -> hat -> hit)
/// ```
///
/// BFS level by level: try changing each character to a-z. If the new word is
/// in the dictionary and unvisited, add to queue. BFS guarantees shortest path.
///
/// Example walkthrough on the example:
///
/// ```text
/// dist=1: queue=["red"]
/// dist=2: "red" -> try all 1-char changes -> "rad" in dict -> queue=["rad"]
/// dist=3: "rad" -> "rat" in dict -> queue=["rat"]
/// dist=4: "rat" -> "hat" in dict -> queue=["hat"]
/// dist=5: "hat" -> "hit" == end -> return 5
/// ```
///
/// # Complexity
///
/// - Time: O(n * m * 26) — n words, m word length
/// - Space: O(n) — visited set + queue
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

/// Prerequisites (Course Schedule)
///
/// Can you take all n courses given prerequisite pairs [a, b] meaning
/// "a must be taken before b"? Basically: does the graph have a cycle?
///
/// # Examples
///
/// ```text
/// Input:  n=3, prereqs=[[0,1],[1,2],[2,1]]
/// Output: false (cycle: 1 <-> 2)
/// ```
///
/// Topological sort using Kahn's algorithm: start with courses that have no
/// prerequisites (in-degree 0), process them, and reduce the in-degree of
/// their dependents. If we process all n courses there's no cycle.
///
/// Example walkthrough on the example:
///
/// ```text
/// graph: 0->[1], 1->[2], 2->[1]
/// in-degree: [0, 1+1, 1] = [0, 2, 1]
/// queue=[0] -> process 0, decrement in-degree[1] -> [0, 1, 1]
/// queue=[]: no more 0-degree nodes, enrolled=1 != 3 -> return False
/// ```
///
/// # Complexity
///
/// - Time: O(V + E) — visit each node and edge once
/// - Space: O(V + E) — adjacency list and in-degree array
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

/// Union-Find (Disjoint Set)
///
/// Track connected components. Support connect and get_community_size.
///
/// # Examples
///
/// ```text
/// Input:  uf = UnionFind(5); uf.connect(0,1); uf.connect(1,2)
///         uf.get_community_size(0)
/// Output: 3
///
/// Input:  uf.get_community_size(3)
/// Output: 1
/// ```
///
/// The idea is to maintain a forest of trees where each tree is a component.
/// To check connectivity, find the root of each node. To merge, attach one
/// root under the other.
///
/// Two optimizations:
/// - Path compression: in find(), point nodes directly to root
/// - Union by size: attach smaller tree under larger tree
///
/// Without these: O(n) per operation (degenerate chain).
/// With both: O(a(n)) where a = inverse Ackermann ~ constant.
///
/// Example walkthrough on the example:
///
/// ```text
/// init:       parent=[0,1,2,3,4], size=[1,1,1,1,1]
/// connect(0,1): root(0)=0, root(1)=1 -> parent=[0,0,2,3,4], size=[2,1,1,1,1]
/// connect(1,2): root(1)=0, root(2)=2 -> parent=[0,0,0,3,4], size=[3,1,1,1,1]
/// get_community_size(0): root=0, size[0]=3
/// get_community_size(3): root=3, size[3]=1
/// ```
///
/// # Complexity
///
/// - Time: O(a(n)) ~ O(1) amortized per operation
/// - Space: O(n) — parent and size arrays
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = Vec::new();
        for i in 0..n {
            parent.push(i);
        }
        Self {
            parent,
            size: vec![1; n],
        }
    }

    pub fn connect(&mut self, x: usize, y: usize) {
        self.union(x, y);
    }

    pub fn get_community_size(&mut self, x: usize) -> usize {
        let rep = self.find(x);
        self.size[rep]
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        // Path compression
        self.parent[x] = self.find(self.parent[x]);
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let rep_x = self.find(x);
        let rep_y = self.find(y);

        if rep_x == rep_y {
            return; // Already the same community
        }

        if self.size[rep_x] > self.size[rep_y] {
            self.parent[rep_y] = rep_x;
            self.size[rep_x] += self.size[rep_y];
        } else {
            self.parent[rep_x] = rep_y;
            self.size[rep_y] += self.size[rep_x];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find_example() {
        let mut uf = UnionFind::new(5);
        uf.connect(0, 1);
        uf.connect(1, 2);
        assert_eq!(uf.get_community_size(3), 1);
        assert_eq!(uf.get_community_size(0), 3);
        uf.connect(3, 4);
        assert_eq!(uf.get_community_size(4), 2);
    }

    #[test]
    fn test_union_find_all_separate() {
        let mut uf = UnionFind::new(4);
        assert_eq!(uf.get_community_size(0), 1);
        assert_eq!(uf.get_community_size(1), 1);
        assert_eq!(uf.get_community_size(2), 1);
        assert_eq!(uf.get_community_size(3), 1);
    }

    #[test]
    fn test_union_find_all_connected() {
        let mut uf = UnionFind::new(4);
        uf.connect(0, 1);
        uf.connect(1, 2);
        uf.connect(2, 3);
        assert_eq!(uf.get_community_size(0), 4);
        assert_eq!(uf.get_community_size(1), 4);
        assert_eq!(uf.get_community_size(2), 4);
        assert_eq!(uf.get_community_size(3), 4);
    }

    #[test]
    fn test_union_find_connect_same_community() {
        let mut uf = UnionFind::new(3);
        uf.connect(0, 1);
        uf.connect(0, 1); // connect again
        assert_eq!(uf.get_community_size(0), 2);
        assert_eq!(uf.get_community_size(1), 2);
    }

    #[test]
    fn test_union_find_merge_two_communities() {
        let mut uf = UnionFind::new(6);
        // Create two communities
        uf.connect(0, 1);
        uf.connect(1, 2);
        uf.connect(3, 4);
        uf.connect(4, 5);
        assert_eq!(uf.get_community_size(0), 3);
        assert_eq!(uf.get_community_size(3), 3);
        // Merge them
        uf.connect(2, 3);
        assert_eq!(uf.get_community_size(0), 6);
        assert_eq!(uf.get_community_size(5), 6);
    }

    #[test]
    fn test_union_find_single_person() {
        let mut uf = UnionFind::new(1);
        assert_eq!(uf.get_community_size(0), 1);
    }

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
