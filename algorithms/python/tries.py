"""
Tries Pattern

A collection of algorithm problems using tries (prefix trees).

A trie stores strings character by character in a tree structure.
Each node has up to 26 children (for lowercase English letters).

Structure:
    root
    ├── t
    │   └── o
    │       └── p (is_word=True)
    └── b
        └── y
            └── e (is_word=True)

    Words stored: "top", "bye"

Key operations:
    insert(word):     O(m) - walk/create nodes for each char, mark end
    search(word):     O(m) - walk nodes, check is_word at end
    has_prefix(pre):  O(m) - walk nodes, return True if path exists

Why not just use a set?
    - Trie supports prefix queries efficiently
    - Trie enables wildcard matching (. = any char)
    - Trie + DFS = word search on boards
"""


class Trie:
    """
    Design a Trie

    Support insert, search, and has_prefix.

    Example:
        >>> t = Trie()
        >>> t.insert("top"); t.insert("bye")
        >>> t.has_prefix("to")
        True
        >>> t.search("to")
        False
        >>> t.insert("to"); t.search("to")
        True

    Each node is a dict of children + is_word flag.
    In Python, we represent a node as: {children: {char: node}, is_word: bool}
    Simplification: use a dict for children, a separate flag for is_word.
    """

    def __init__(self):
        self.children = {}
        self.is_word = False

    def insert(self, word: str) -> None:
        node = self
        for c in word:
            if c not in node.children:
                node.children[c] = Trie()
            node = node.children[c]
        node.is_word = True

    def search(self, word: str) -> bool:
        node = self
        for c in word:
            if c not in node.children:
                return False
            node = node.children[c]
        return node.is_word

    def has_prefix(self, prefix: str) -> bool:
        node = self
        for c in prefix:
            if c not in node.children:
                return False
            node = node.children[c]
        return True


# -----------------------------------------------------------------------------
# Tests for Trie
# -----------------------------------------------------------------------------

def test_trie_example():
    t = Trie()
    t.insert("top")
    t.insert("bye")
    assert t.has_prefix("to") == True
    assert t.search("to") == False
    t.insert("to")
    assert t.search("to") == True


def test_trie_search_empty():
    assert Trie().search("hello") == False


def test_trie_has_prefix_empty():
    assert Trie().has_prefix("he") == False


def test_trie_insert_and_search():
    t = Trie()
    t.insert("hello")
    assert t.search("hello") == True
    assert t.search("hell") == False
    assert t.search("helloo") == False


def test_trie_has_prefix():
    t = Trie()
    t.insert("hello")
    assert t.has_prefix("h") == True
    assert t.has_prefix("he") == True
    assert t.has_prefix("hel") == True
    assert t.has_prefix("hell") == True
    assert t.has_prefix("hello") == True
    assert t.has_prefix("helloa") == False


def test_trie_multiple_words_same_prefix():
    t = Trie()
    for w in ["car", "card", "care", "careful"]:
        t.insert(w)
    assert t.search("car") == True
    assert t.search("card") == True
    assert t.search("care") == True
    assert t.search("careful") == True
    assert t.search("ca") == False
    assert t.has_prefix("ca") == True
    assert t.has_prefix("car") == True


def test_trie_single_char():
    t = Trie()
    t.insert("a")
    assert t.search("a") == True
    assert t.has_prefix("a") == True
    assert t.search("b") == False


def test_trie_overlapping_words():
    t = Trie()
    for w in ["abc", "ab", "a"]:
        t.insert(w)
    assert t.search("a") == True
    assert t.search("ab") == True
    assert t.search("abc") == True


class WildcardTrie:
    """
    Insert and Search Words with Wildcards

    Like Trie but search supports '.' as wildcard (matches any single char).

    Example:
        >>> t = WildcardTrie()
        >>> t.insert("band"); t.insert("rat")
        >>> t.search("ra.")   # matches "rat"
        True
        >>> t.search("b..")   # no 3-letter word starting with 'b'
        False
        >>> t.insert("ran"); t.search(".an")  # matches "ran"
        True

    Time Complexity:
        insert: O(m)
        search: O(m) without wildcards, O(26^w * m) worst case with w wildcards

    When we hit '.', we must try ALL children (branching search).
    Normal chars follow one path. This is why wildcards are expensive.
    """

    def __init__(self):
        self.children = {}
        self.is_word = False

    def insert(self, word: str) -> None:
        node = self
        for c in word:
            if c not in node.children:
                node.children[c] = WildcardTrie()
            node = node.children[c]
        node.is_word = True

    def search(self, word: str) -> bool:
        return self._search(word, 0)

    def _search(self, word: str, i: int) -> bool:
        # Base case: consumed all characters
        if i == len(word):
            return self.is_word

        c = word[i]
        if c == '.':
            # Wildcard: try every child
            for child in self.children.values():
                if child._search(word, i + 1):
                    return True
            return False
        else:
            if c not in self.children:
                return False
            return self.children[c]._search(word, i + 1)


# -----------------------------------------------------------------------------
# Tests for WildcardTrie
# -----------------------------------------------------------------------------

def test_wildcard_trie_example():
    t = WildcardTrie()
    t.insert("band")
    t.insert("rat")
    assert t.search("ra.") == True
    assert t.search("b..") == False
    t.insert("ran")
    assert t.search(".an") == True


def test_wildcard_trie_no_wildcard():
    t = WildcardTrie()
    t.insert("hello")
    assert t.search("hello") == True
    assert t.search("hell") == False
    assert t.search("helloo") == False


def test_wildcard_trie_all_wildcards():
    t = WildcardTrie()
    t.insert("cat")
    t.insert("car")
    assert t.search("...") == True
    assert t.search("..") == False


def test_wildcard_trie_wildcard_at_start():
    t = WildcardTrie()
    for w in ["cat", "bat", "rat"]:
        t.insert(w)
    assert t.search(".at") == True
    assert t.search(".it") == False


def test_wildcard_trie_wildcard_in_middle():
    t = WildcardTrie()
    for w in ["bad", "bed", "bid"]:
        t.insert(w)
    assert t.search("b.d") == True
    assert t.search("b.t") == False


def test_wildcard_trie_empty():
    t = WildcardTrie()
    assert t.search("...") == False
    assert t.search("a") == False


def test_wildcard_trie_multiple_matches():
    t = WildcardTrie()
    for w in ["dad", "mad", "pad"]:
        t.insert(w)
    assert t.search(".ad") == True
    assert t.search("..d") == True
    assert t.search("...") == True


def find_words(board: list[list[str]], words: list[str]) -> list[str]:
    """
    Find All Words on a Board

    Given a 2D board and a list of words, find all words that can be formed
    by tracing a path through adjacent cells (up/down/left/right).
    Each cell can only be used once per word.

    Example:
        board = [['b','y','s'],
                 ['r','t','e'],
                 ['a','i','n']]
        words = ["byte","bytes","rat","rain","trait","train"]
        → ["byte", "bytes", "rain", "train"]

    Time Complexity: O(w*m + rows*cols*4^max_word_len)
    Space Complexity: O(w*m) for trie + O(max_word_len) recursion

    Approach: Build a trie from all words, then DFS from each cell.
    The trie lets us prune early: if no word starts with the current
    path, stop exploring.

    Without trie: search each word separately → O(words * cells * 4^len)
    With trie: search all words simultaneously, sharing common prefixes.

    Steps:
        1. Build trie from words, store full word at leaf nodes
        2. For each cell, if it matches a trie child, start DFS
        3. DFS: mark cell visited, explore 4 directions, unmark
        4. When we reach a leaf with a word, add it to results
    """
    # Build trie: each node is {children, word}
    # Store the full word at leaf instead of is_word flag
    trie = {}
    for word in words:
        node = trie
        for c in word:
            if c not in node:
                node[c] = {}
            node = node[c]
        node['#'] = word  # '#' marks end, stores the full word

    rows, cols = len(board), len(board[0])
    result = []

    def dfs(r, c, node):
        # If this node has a complete word, collect it
        if '#' in node:
            result.append(node.pop('#'))  # pop to avoid duplicates

        # Mark visited
        tmp = board[r][c]
        board[r][c] = '.'

        # Explore 4 directions
        for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nr, nc = r + dr, c + dc
            if 0 <= nr < rows and 0 <= nc < cols and board[nr][nc] in node:
                dfs(nr, nc, node[board[nr][nc]])

        # Unmark
        board[r][c] = tmp

    # Start DFS from every cell that matches a trie root child
    for r in range(rows):
        for c in range(cols):
            if board[r][c] in trie:
                dfs(r, c, trie[board[r][c]])

    return result


# -----------------------------------------------------------------------------
# Tests for find_words
# -----------------------------------------------------------------------------

def test_find_words_example():
    board = [['b', 'y', 's'],
             ['r', 't', 'e'],
             ['a', 'i', 'n']]
    words = ["byte", "bytes", "rat", "rain", "trait", "train"]
    result = sorted(find_words(board, words))
    assert result == ["byte", "bytes", "rain", "train"]


def test_find_words_no_matches():
    board = [['a', 'b'],
             ['c', 'd']]
    assert find_words(board, ["xyz", "hello"]) == []


def test_find_words_single_cell():
    board = [['a']]
    assert find_words(board, ["a", "b", "ab"]) == ["a"]


def test_find_words_all_match():
    board = [['a', 'b'],
             ['c', 'd']]
    result = sorted(find_words(board, ["ab", "abc", "abdc", "acd"]))
    assert result == ["ab", "abdc", "acd"]


def test_find_words_no_reuse():
    board = [['a', 'a']]
    result = find_words(board, ["aa", "aaa"])
    assert result == ["aa"]


def test_find_words_empty_words():
    board = [['a', 'b'],
             ['c', 'd']]
    assert find_words(board, []) == []


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
