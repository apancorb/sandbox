"""
Graphs Pattern

A collection of algorithm problems using graphs.

Common techniques:
- DFS: explore as deep as possible, backtrack (recursion or stack)
- BFS: explore level by level (queue), good for shortest paths
- Topological sort: ordering with dependencies (Kahn's algorithm)
- Union-Find: track connected components, merge groups

Representations:
- Adjacency list: graph[node] = [neighbors]  (most common)
- Grid/matrix: implicit graph, neighbors are 4-directional cells
"""

from collections import deque


def count_islands(grid: list[list[int]]) -> int:
    """
    Count Islands

    Count the number of islands in a binary grid (1=land, 0=water).
    Adjacent = up/down/left/right (not diagonal).

    Example:
        [[1,1,0,0],
         [1,1,0,0],
         [0,0,1,1],
         [0,0,1,1]]  → 2

    Time Complexity: O(rows * cols)
    Space Complexity: O(rows * cols) worst case recursion

    For each unvisited land cell, DFS to mark the whole island as visited.
    Each DFS = one island found.
    """
    if not grid:
        return 0

    rows, cols = len(grid), len(grid[0])
    count = 0

    def dfs(r, c):
        if r < 0 or r >= rows or c < 0 or c >= cols or grid[r][c] != 1:
            return
        grid[r][c] = -1  # mark visited
        dfs(r + 1, c)
        dfs(r - 1, c)
        dfs(r, c + 1)
        dfs(r, c - 1)

    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == 1:
                dfs(r, c)
                count += 1

    return count


# -----------------------------------------------------------------------------
# Tests for count_islands
# -----------------------------------------------------------------------------

def test_count_islands_example():
    grid = [[1,1,0,0],[1,1,0,0],[0,0,1,1],[0,0,1,1]]
    assert count_islands(grid) == 2


def test_count_islands_single():
    assert count_islands([[1,1,1],[1,1,1],[1,1,1]]) == 1


def test_count_islands_none():
    assert count_islands([[0,0,0],[0,0,0]]) == 0


def test_count_islands_diagonal():
    assert count_islands([[1,0],[0,1]]) == 2


def test_count_islands_many():
    assert count_islands([[1,0,1,0,1],[0,0,0,0,0],[1,0,1,0,1]]) == 6


def test_count_islands_single_cell():
    assert count_islands([[1]]) == 1


def test_count_islands_empty():
    assert count_islands([]) == 0


def matrix_infection(grid: list[list[int]]) -> int:
    """
    Matrix Infection (Rotting Oranges)

    0=empty, 1=uninfected, 2=infected.
    Each second, infected cells spread to adjacent uninfected cells.
    Return seconds to infect all, or -1 if impossible.

    Example:
        [[1,1,1,0],
         [0,0,2,1],
         [0,1,1,0]]  → 3

    Time Complexity: O(rows * cols)
    Space Complexity: O(rows * cols) - queue

    Multi-source BFS: start from ALL infected cells simultaneously.
    Each BFS level = one second. Count remaining uninfected at end.
    """
    if not grid:
        return 0

    rows, cols = len(grid), len(grid[0])
    queue = deque()
    ones = 0

    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == 2:
                queue.append((r, c))
            elif grid[r][c] == 1:
                ones += 1

    seconds = 0
    while ones > 0 and queue:
        for _ in range(len(queue)):
            r, c = queue.popleft()
            for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                nr, nc = r + dr, c + dc
                if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == 1:
                    grid[nr][nc] = 2
                    queue.append((nr, nc))
                    ones -= 1
        seconds += 1

    return -1 if ones > 0 else seconds


# -----------------------------------------------------------------------------
# Tests for matrix_infection
# -----------------------------------------------------------------------------

def test_matrix_infection_example():
    assert matrix_infection([[1,1,1,0],[0,0,2,1],[0,1,1,0]]) == 3


def test_matrix_infection_impossible():
    assert matrix_infection([[1,0,2]]) == -1


def test_matrix_infection_already_done():
    assert matrix_infection([[2,0,2],[0,0,0]]) == 0


def test_matrix_infection_one_second():
    assert matrix_infection([[2,1]]) == 1


def test_matrix_infection_multiple_sources():
    assert matrix_infection([[2,1,1,1,2]]) == 2


def test_matrix_infection_empty():
    assert matrix_infection([]) == 0


def test_matrix_infection_no_infected():
    assert matrix_infection([[1,1]]) == -1


def is_bipartite(graph: list[list[int]]) -> bool:
    """
    Bipartite Graph Validation

    Can nodes be colored with 2 colors so no adjacent nodes share a color?

    Example:
        0 -- 1 -- 2        graph = [[1,4],[0,2],[1],[4],[0,3]]
        |                  → True (two-colorable)
        4 -- 3

    Time Complexity: O(V + E)
    Space Complexity: O(V) - color array

    DFS coloring: assign color +1 to a node, -1 to its neighbors.
    If a neighbor already has the SAME color → not bipartite.
    A graph is bipartite iff it has no odd-length cycles.
    """
    n = len(graph)
    colors = [0] * n  # 0=unvisited, 1=color A, -1=color B

    def dfs(node, color):
        colors[node] = color
        for neighbor in graph[node]:
            if colors[neighbor] == color:
                return False  # same color as neighbor!
            if colors[neighbor] == 0 and not dfs(neighbor, -color):
                return False
        return True

    for i in range(n):
        if colors[i] == 0 and not dfs(i, 1):
            return False

    return True


# -----------------------------------------------------------------------------
# Tests for is_bipartite
# -----------------------------------------------------------------------------

def test_is_bipartite_example():
    assert is_bipartite([[1,4],[0,2],[1],[4],[0,3]]) == True


def test_is_bipartite_triangle():
    assert is_bipartite([[1,2],[0,2],[0,1]]) == False


def test_is_bipartite_square():
    assert is_bipartite([[1,3],[0,2],[1,3],[0,2]]) == True


def test_is_bipartite_disconnected():
    assert is_bipartite([[1],[0],[3],[2]]) == True


def test_is_bipartite_single():
    assert is_bipartite([[]]) == True


def test_is_bipartite_empty():
    assert is_bipartite([]) == True


def test_is_bipartite_disconnected_odd_cycle():
    assert is_bipartite([[1],[0],[3,4],[2,4],[2,3]]) == False


def longest_increasing_path(matrix: list[list[int]]) -> int:
    """
    Longest Increasing Path in a Matrix

    Find the longest strictly increasing path (4-directional moves).

    Example:
        [[2,7,9],
         [5,4,3],
         [6,1,8]]  → 4  (1→4→5→6 or 1→4→7→9)

    Time Complexity: O(rows * cols) - each cell computed once
    Space Complexity: O(rows * cols) - memo table

    DFS + memoization: from each cell, try all 4 directions where
    the neighbor is strictly larger. Cache results to avoid recomputation.

    Why memoization works: if we already know the longest path starting
    from cell (r,c), we don't need to recompute it.
    """
    if not matrix or not matrix[0]:
        return 0

    rows, cols = len(matrix), len(matrix[0])
    memo = [[0] * cols for _ in range(rows)]

    def dfs(r, c):
        if memo[r][c] != 0:
            return memo[r][c]

        best = 1
        for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and matrix[nr][nc] > matrix[r][c]:
                best = max(best, 1 + dfs(nr, nc))

        memo[r][c] = best
        return best

    result = 0
    for r in range(rows):
        for c in range(cols):
            result = max(result, dfs(r, c))

    return result


# -----------------------------------------------------------------------------
# Tests for longest_increasing_path
# -----------------------------------------------------------------------------

def test_longest_increasing_path_example():
    assert longest_increasing_path([[2,7,9],[5,4,3],[6,1,8]]) == 4


def test_longest_increasing_path_single():
    assert longest_increasing_path([[1]]) == 1


def test_longest_increasing_path_row():
    assert longest_increasing_path([[1,2,3,4,5]]) == 5


def test_longest_increasing_path_decreasing():
    assert longest_increasing_path([[5,4,3,2,1]]) == 5


def test_longest_increasing_path_column():
    assert longest_increasing_path([[1],[2],[3],[4]]) == 4


def test_longest_increasing_path_all_same():
    assert longest_increasing_path([[1,1],[1,1]]) == 1


def test_longest_increasing_path_spiral():
    assert longest_increasing_path([[1,2,3],[8,9,4],[7,6,5]]) == 9


def test_longest_increasing_path_empty():
    assert longest_increasing_path([]) == 0


def shortest_transformation(start: str, end: str, dictionary: list[str]) -> int:
    """
    Shortest Transformation Sequence (Word Ladder)

    Find length of shortest sequence from start to end where each step
    changes exactly one letter and the resulting word is in the dictionary.

    Example:
        start="red", end="hit"
        dictionary=["red","rod","rad","rat","hat","bad","bat","hit"]
        → 5  (red → rad → rat → hat → hit)

    Time Complexity: O(n * m * 26) - n words, m word length
    Space Complexity: O(n) - visited set + queue

    BFS level by level: try changing each character to a-z.
    If the new word is in the dictionary and unvisited, add to queue.
    BFS guarantees shortest path.
    """
    if start == end:
        return 1

    word_set = set(dictionary)
    if end not in word_set:
        return 0

    visited = {start}
    queue = deque([start])
    dist = 1

    while queue:
        for _ in range(len(queue)):
            word = queue.popleft()
            chars = list(word)

            for i in range(len(chars)):
                original = chars[i]
                for c in 'abcdefghijklmnopqrstuvwxyz':
                    if c != original:
                        chars[i] = c
                        new_word = ''.join(chars)

                        if new_word == end:
                            return dist + 1

                        if new_word in word_set and new_word not in visited:
                            visited.add(new_word)
                            queue.append(new_word)

                chars[i] = original  # restore
        dist += 1

    return 0


# -----------------------------------------------------------------------------
# Tests for shortest_transformation
# -----------------------------------------------------------------------------

def test_shortest_transformation_example():
    d = ["red","rod","rad","rat","hat","bad","bat","hit"]
    assert shortest_transformation("red", "hit", d) == 5


def test_shortest_transformation_no_path():
    assert shortest_transformation("red", "hit", ["red","rod","rad"]) == 0


def test_shortest_transformation_one_step():
    assert shortest_transformation("hit", "hot", ["hit","hot"]) == 2


def test_shortest_transformation_same_word():
    assert shortest_transformation("hit", "hit", ["hit"]) == 1


def test_shortest_transformation_direct():
    assert shortest_transformation("red", "rad", ["red","rod","rad"]) == 2


def test_shortest_transformation_multiple_paths():
    d = ["hit","hot","dot","dog","lot","log"]
    assert shortest_transformation("hit", "dog", d) == 4


def can_finish(n: int, prerequisites: list[list[int]]) -> bool:
    """
    Prerequisites (Course Schedule)

    Can you take all n courses given prerequisite pairs [a, b] meaning
    "a must be taken before b"? Basically: does the graph have a cycle?

    Example:
        n=3, prereqs=[[0,1],[1,2],[2,1]]  → False (cycle: 1↔2)

    Time Complexity: O(V + E)
    Space Complexity: O(V + E)

    Topological sort (Kahn's algorithm):
        1. Count in-degrees (how many prereqs each course has)
        2. Start with courses that have 0 in-degree (no prereqs)
        3. Process queue: for each course, reduce in-degree of dependents
        4. If we process all n courses → no cycle → True
           If some courses remain → cycle → False
    """
    graph = [[] for _ in range(n)]
    in_degree = [0] * n

    for prereq, course in prerequisites:
        graph[prereq].append(course)
        in_degree[course] += 1

    # Start with courses that have no prerequisites
    queue = deque(i for i in range(n) if in_degree[i] == 0)

    enrolled = 0
    while queue:
        node = queue.popleft()
        enrolled += 1
        for neighbor in graph[node]:
            in_degree[neighbor] -= 1
            if in_degree[neighbor] == 0:
                queue.append(neighbor)

    return enrolled == n


# -----------------------------------------------------------------------------
# Tests for can_finish
# -----------------------------------------------------------------------------

def test_can_finish_cycle():
    assert can_finish(3, [[0,1],[1,2],[2,1]]) == False


def test_can_finish_no_cycle():
    assert can_finish(3, [[0,1],[1,2]]) == True


def test_can_finish_no_prereqs():
    assert can_finish(3, []) == True


def test_can_finish_single():
    assert can_finish(1, []) == True


def test_can_finish_diamond():
    assert can_finish(4, [[0,1],[0,2],[1,3],[2,3]]) == True


def test_can_finish_two_node_cycle():
    assert can_finish(2, [[0,1],[1,0]]) == False


def test_can_finish_disconnected():
    assert can_finish(4, [[0,1],[2,3]]) == True


class UnionFind:
    """
    Union-Find (Disjoint Set)

    Track connected components. Support connect and get_community_size.

    Example:
        >>> uf = UnionFind(5)
        >>> uf.connect(0, 1); uf.connect(1, 2)
        >>> uf.get_community_size(0)
        3
        >>> uf.get_community_size(3)
        1

    Time Complexity: O(α(n)) ≈ O(1) amortized per operation
    Space Complexity: O(n)

    Two optimizations:
        - Path compression: in find(), point nodes directly to root
        - Union by size: attach smaller tree under larger tree

    Without these: O(n) per operation (degenerate chain).
    With both: O(α(n)) where α = inverse Ackermann ≈ constant.
    """

    def __init__(self, n: int):
        self.parent = list(range(n))  # each node is its own parent
        self.size = [1] * n           # each community starts at size 1

    def find(self, x: int) -> int:
        # Path compression: make every node point directly to root
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def connect(self, x: int, y: int) -> None:
        root_x = self.find(x)
        root_y = self.find(y)

        if root_x == root_y:
            return  # already connected

        # Union by size: attach smaller to larger
        if self.size[root_x] >= self.size[root_y]:
            self.parent[root_y] = root_x
            self.size[root_x] += self.size[root_y]
        else:
            self.parent[root_x] = root_y
            self.size[root_y] += self.size[root_x]

    def get_community_size(self, x: int) -> int:
        return self.size[self.find(x)]


# -----------------------------------------------------------------------------
# Tests for UnionFind
# -----------------------------------------------------------------------------

def test_union_find_example():
    uf = UnionFind(5)
    uf.connect(0, 1)
    uf.connect(1, 2)
    assert uf.get_community_size(3) == 1
    assert uf.get_community_size(0) == 3
    uf.connect(3, 4)
    assert uf.get_community_size(4) == 2


def test_union_find_all_separate():
    uf = UnionFind(4)
    for i in range(4):
        assert uf.get_community_size(i) == 1


def test_union_find_all_connected():
    uf = UnionFind(4)
    uf.connect(0, 1)
    uf.connect(1, 2)
    uf.connect(2, 3)
    for i in range(4):
        assert uf.get_community_size(i) == 4


def test_union_find_same_community():
    uf = UnionFind(3)
    uf.connect(0, 1)
    uf.connect(0, 1)
    assert uf.get_community_size(0) == 2


def test_union_find_merge_two():
    uf = UnionFind(6)
    uf.connect(0, 1)
    uf.connect(1, 2)
    uf.connect(3, 4)
    uf.connect(4, 5)
    assert uf.get_community_size(0) == 3
    assert uf.get_community_size(3) == 3
    uf.connect(2, 3)
    assert uf.get_community_size(0) == 6
    assert uf.get_community_size(5) == 6


def test_union_find_single():
    uf = UnionFind(1)
    assert uf.get_community_size(0) == 1


def clone_graph(node: dict | None) -> dict | None:
    """
    Graph Deep Copy

    Deep copy an undirected graph. Each node has val and neighbors list.
    Using dicts for simplicity: {"val": 1, "neighbors": [node2, node4]}

    Time Complexity: O(V + E)
    Space Complexity: O(V) - hash map of old→new

    DFS: clone current node, recursively clone neighbors.
    Use a map to avoid cloning the same node twice (handles cycles).
    """
    if not node:
        return None

    cloned = {}  # id(old_node) → new_node

    def dfs(n):
        if id(n) in cloned:
            return cloned[id(n)]

        copy = {"val": n["val"], "neighbors": []}
        cloned[id(n)] = copy

        for neighbor in n["neighbors"]:
            copy["neighbors"].append(dfs(neighbor))

        return copy

    return dfs(node)


# -----------------------------------------------------------------------------
# Tests for clone_graph
# -----------------------------------------------------------------------------

def test_clone_graph_example():
    n1 = {"val": 1, "neighbors": []}
    n2 = {"val": 2, "neighbors": []}
    n3 = {"val": 3, "neighbors": []}
    n4 = {"val": 4, "neighbors": []}
    n1["neighbors"] = [n2, n4]
    n2["neighbors"] = [n1, n3]
    n3["neighbors"] = [n2, n4]
    n4["neighbors"] = [n3, n1]

    c = clone_graph(n1)
    assert c["val"] == 1
    assert c is not n1
    assert len(c["neighbors"]) == 2
    vals = [nb["val"] for nb in c["neighbors"]]
    assert 2 in vals and 4 in vals


def test_clone_graph_empty():
    assert clone_graph(None) is None


def test_clone_graph_single():
    n1 = {"val": 1, "neighbors": []}
    c = clone_graph(n1)
    assert c["val"] == 1
    assert c is not n1
    assert c["neighbors"] == []


def test_clone_graph_two_nodes():
    n1 = {"val": 1, "neighbors": []}
    n2 = {"val": 2, "neighbors": []}
    n1["neighbors"] = [n2]
    n2["neighbors"] = [n1]
    c = clone_graph(n1)
    assert c["val"] == 1
    assert c["neighbors"][0]["val"] == 2
    assert c["neighbors"][0]["neighbors"][0] is c  # circular ref preserved


def test_clone_graph_independent():
    n1 = {"val": 1, "neighbors": []}
    n2 = {"val": 2, "neighbors": []}
    n1["neighbors"] = [n2]
    n2["neighbors"] = [n1]
    c = clone_graph(n1)
    n1["val"] = 100
    assert c["val"] == 1  # unaffected


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
