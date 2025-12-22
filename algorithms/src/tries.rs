use std::collections::HashMap;

/// Design a Trie
///
/// Design and implement a trie data structure that supports the following operations:
/// - `insert(word)`: Inserts a word into the trie.
/// - `search(word)`: Returns true if a word exists in the trie, and false if not.
/// - `has_prefix(prefix)`: Returns true if the trie contains a word with the given prefix,
///   and false if not.
///
/// # Example
///
/// ```text
/// Input: [insert("top"), insert("bye"), has_prefix("to"), search("to"), insert("to"), search("to")]
/// Output: [true, false, true]
///
/// Explanation:
/// insert("top")      // trie has: "top"
/// insert("bye")      // trie has: "top" and "bye"
/// has_prefix("to")   // prefix "to" exists in the string "top": return true
/// search("to")       // trie does not contain the word "to": return false
/// insert("to")       // trie has: "top", "bye", and "to"
/// search("to")       // trie contains the word "to": return true
/// ```
///
/// # Constraints
///
/// - The words and prefixes consist only of lowercase English letters.
/// - The length of each word and prefix is at least one character.

#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for c in word.chars() {
            if !node.children.contains_key(&c) {
                node.children.insert(c, TrieNode::default());
            }
            node = node.children.get_mut(&c).unwrap();
        }

        node.is_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;

        for c in word.chars() {
            if !node.children.contains_key(&c) {
                return false;
            }
            node = node.children.get(&c).unwrap();
        }

        node.is_word
    }

    pub fn has_prefix(&self, prefix: &str) -> bool {
        let mut node = &self.root;

        for c in prefix.chars() {
            if !node.children.contains_key(&c) {
                return false;
            }
            node = node.children.get(&c).unwrap();
        }

        true
    }
}

/// Insert and Search Words with Wildcards
///
/// Design and implement a data structure that supports the following operations:
/// - `insert(word)`: Inserts a word into the data structure.
/// - `search(word)`: Returns true if a word exists in the data structure and false if not.
///   The word may contain wildcards (`.`) that can represent any letter.
///
/// # Example
///
/// ```text
/// Input: [insert("band"), insert("rat"), search("ra."), search("b.."), insert("ran"), search(".an")]
/// Output: [true, false, true]
///
/// Explanation:
/// insert("band")   // data structure has: "band"
/// insert("rat")    // data structure has: "band" and "rat"
/// search("ra.")    // "ra." matches "rat": return true
/// search("b..")    // no three-letter word starting with 'b': return false
/// insert("ran")    // data structure has: "band", "rat", and "ran"
/// search(".an")    // ".an" matches "ran": return true
/// ```
///
/// # Constraints
///
/// - Words will only contain lowercase English letters and (`.`) characters.

pub struct WildcardTrie {
    root: TrieNode,
}

impl WildcardTrie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for c in word.chars() {
            if !node.children.contains_key(&c) {
                node.children.insert(c, TrieNode::default());
            }
            node = node.children.get_mut(&c).unwrap();
        }

        node.is_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let node = &self.root;
        self.search_helper(node, 0, word)
    }

    fn search_helper(&self, mut node: &TrieNode, start_index: usize, word: &str) -> bool {
        let words: Vec<char> = word.chars().collect();
        for i in start_index..words.len() {
            let c = words[i];
            if c == '.' {
                for next_node in node.children.values() {
                    if self.search_helper(next_node, i + 1, word) {
                        return true;
                    }
                }
                return false;
            } else if !node.children.contains_key(&c) {
                return false;
            } else {
                node = node.children.get(&c).unwrap();
            }
        }
        node.is_word
    }
}

/// Find All Words on a Board
///
/// Given a 2D board of characters and an array of words, find all the words in the array that
/// can be formed by tracing a path through adjacent cells in the board. Adjacent cells are those
/// which horizontally or vertically neighbor each other. We can't use the same cell more than
/// once for a single word.
///
/// # Example
///
/// ```text
/// Input: board = [['b', 'y', 's'],
///                 ['r', 't', 'e'],
///                 ['a', 'i', 'n']],
///        words = ["byte", "bytes", "rat", "rain", "trait", "train"]
///
/// Output: ["byte", "bytes", "rain", "train"]
///
/// Explanation:
/// - "byte": b(0,0) -> y(0,1) -> t(1,1) -> e(1,2) ✓
/// - "bytes": b(0,0) -> y(0,1) -> t(1,1) -> e(1,2) -> s(0,2) ✓
/// - "rat": no valid path
/// - "rain": r(1,0) -> a(2,0) -> i(2,1) -> n(2,2) ✓
/// - "trait": no valid path
/// - "train": t(1,1) -> r(1,0) -> a(2,0) -> i(2,1) -> n(2,2) ✓
/// ```
///
/// # Constraints
///
/// - The board contains only lowercase English letters.
/// - Words contain only lowercase English letters.

#[derive(Default)]
pub struct TrieNodeWithWord {
    children: HashMap<char, TrieNodeWithWord>,
    word: Option<String>,
}

impl TrieNodeWithWord {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn find_words(board: &mut [&mut [char]], words: &[&str]) -> Vec<String> {
    fn find_words_helper(
        board: &mut [&mut [char]],
        node: &mut TrieNodeWithWord,
        r: usize,
        c: usize,
        res: &mut Vec<String>,
    ) {
        if let Some(word) = node.word.take() {
            res.push(word);
        }

        let tmp = board[r][c];
        board[r][c] = '#';

        const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dr, dc) in DIRS {
            let Some(next_r) = r.checked_add_signed(dr) else { continue };
            let Some(next_c) = c.checked_add_signed(dc) else { continue };

            if next_r < board.len()
                && next_c < board[0].len()
                && node.children.contains_key(&board[next_r][next_c])
            {
                find_words_helper(
                    board,
                    node.children.get_mut(&board[next_r][next_c]).unwrap(),
                    next_r,
                    next_c,
                    res,
                );
            }
        }

        board[r][c] = tmp;
    }

    let mut root = TrieNodeWithWord::new();
    for word in words {
        let mut node = &mut root;
        for c in word.chars() {
            if !node.children.contains_key(&c) {
                node.children.insert(c, TrieNodeWithWord::new());
            }
            node = node.children.get_mut(&c).unwrap();
        }
        node.word = Some(word.to_string());
    }

    let mut res = Vec::new();
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            if let Some(child) = root.children.get_mut(&board[r][c]) {
                find_words_helper(board, child, r, c, &mut res);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words_example() {
        let mut row0 = ['b', 'y', 's'];
        let mut row1 = ['r', 't', 'e'];
        let mut row2 = ['a', 'i', 'n'];
        let board: &mut [&mut [char]] = &mut [&mut row0, &mut row1, &mut row2];
        let words = &["byte", "bytes", "rat", "rain", "trait", "train"];
        let mut result = find_words(board, words);
        result.sort();
        assert_eq!(result, vec!["byte", "bytes", "rain", "train"]);
    }

    #[test]
    fn test_find_words_no_matches() {
        let mut row0 = ['a', 'b'];
        let mut row1 = ['c', 'd'];
        let board: &mut [&mut [char]] = &mut [&mut row0, &mut row1];
        let words = &["xyz", "hello"];
        let result = find_words(board, words);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_words_single_cell() {
        let mut row0 = ['a'];
        let board: &mut [&mut [char]] = &mut [&mut row0];
        let words = &["a", "b", "ab"];
        let result = find_words(board, words);
        assert_eq!(result, vec!["a"]);
    }

    #[test]
    fn test_find_words_all_match() {
        // Board:
        //   a  b
        //   c  d
        // "ab": a(0,0) -> b(0,1) ✓
        // "abc": a -> b -> c? b(0,1) not adjacent to c(1,0) - diagonal ✗
        // "abdc": a(0,0) -> b(0,1) -> d(1,1) -> c(1,0) ✓
        // "acd": a(0,0) -> c(1,0) -> d(1,1) ✓
        let mut row0 = ['a', 'b'];
        let mut row1 = ['c', 'd'];
        let board: &mut [&mut [char]] = &mut [&mut row0, &mut row1];
        let words = &["ab", "abc", "abdc", "acd"];
        let mut result = find_words(board, words);
        result.sort();
        assert_eq!(result, vec!["ab", "abdc", "acd"]);
    }

    #[test]
    fn test_find_words_no_reuse() {
        // Can't use same cell twice
        let mut row0 = ['a', 'a'];
        let board: &mut [&mut [char]] = &mut [&mut row0];
        let words = &["aa", "aaa"];
        let result = find_words(board, words);
        assert_eq!(result, vec!["aa"]);
    }

    #[test]
    fn test_find_words_empty_words() {
        let mut row0 = ['a', 'b'];
        let mut row1 = ['c', 'd'];
        let board: &mut [&mut [char]] = &mut [&mut row0, &mut row1];
        let words: &[&str] = &[];
        let result = find_words(board, words);
        assert!(result.is_empty());
    }

    #[test]
    fn test_wildcard_trie_example() {
        let mut trie = WildcardTrie::new();
        trie.insert("band");
        trie.insert("rat");
        assert!(trie.search("ra.")); // matches "rat"
        assert!(!trie.search("b..")); // no 3-letter word starting with 'b'
        trie.insert("ran");
        assert!(trie.search(".an")); // matches "ran"
    }

    #[test]
    fn test_wildcard_trie_no_wildcard() {
        let mut trie = WildcardTrie::new();
        trie.insert("hello");
        assert!(trie.search("hello"));
        assert!(!trie.search("hell"));
        assert!(!trie.search("helloo"));
    }

    #[test]
    fn test_wildcard_trie_all_wildcards() {
        let mut trie = WildcardTrie::new();
        trie.insert("cat");
        trie.insert("car");
        assert!(trie.search("...")); // matches any 3-letter word
        assert!(!trie.search("..")); // no 2-letter words
    }

    #[test]
    fn test_wildcard_trie_wildcard_at_start() {
        let mut trie = WildcardTrie::new();
        trie.insert("cat");
        trie.insert("bat");
        trie.insert("rat");
        assert!(trie.search(".at"));
        assert!(!trie.search(".it"));
    }

    #[test]
    fn test_wildcard_trie_wildcard_in_middle() {
        let mut trie = WildcardTrie::new();
        trie.insert("bad");
        trie.insert("bed");
        trie.insert("bid");
        assert!(trie.search("b.d"));
        assert!(!trie.search("b.t"));
    }

    #[test]
    fn test_wildcard_trie_empty() {
        let trie = WildcardTrie::new();
        assert!(!trie.search("..."));
        assert!(!trie.search("a"));
    }

    #[test]
    fn test_wildcard_trie_multiple_matches() {
        let mut trie = WildcardTrie::new();
        trie.insert("dad");
        trie.insert("mad");
        trie.insert("pad");
        assert!(trie.search(".ad"));
        assert!(trie.search("..d"));
        assert!(trie.search("..."));
    }

    #[test]
    fn test_trie_example() {
        let mut trie = Trie::new();
        trie.insert("top");
        trie.insert("bye");
        assert!(trie.has_prefix("to"));
        assert!(!trie.search("to"));
        trie.insert("to");
        assert!(trie.search("to"));
    }

    #[test]
    fn test_trie_search_empty() {
        let trie = Trie::new();
        assert!(!trie.search("hello"));
    }

    #[test]
    fn test_trie_has_prefix_empty() {
        let trie = Trie::new();
        assert!(!trie.has_prefix("he"));
    }

    #[test]
    fn test_trie_insert_and_search() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(trie.search("hello"));
        assert!(!trie.search("hell"));
        assert!(!trie.search("helloo"));
    }

    #[test]
    fn test_trie_has_prefix() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(trie.has_prefix("h"));
        assert!(trie.has_prefix("he"));
        assert!(trie.has_prefix("hel"));
        assert!(trie.has_prefix("hell"));
        assert!(trie.has_prefix("hello"));
        assert!(!trie.has_prefix("helloa"));
    }

    #[test]
    fn test_trie_multiple_words_same_prefix() {
        let mut trie = Trie::new();
        trie.insert("car");
        trie.insert("card");
        trie.insert("care");
        trie.insert("careful");

        assert!(trie.search("car"));
        assert!(trie.search("card"));
        assert!(trie.search("care"));
        assert!(trie.search("careful"));
        assert!(!trie.search("ca"));
        assert!(trie.has_prefix("ca"));
        assert!(trie.has_prefix("car"));
        assert!(trie.has_prefix("care"));
    }

    #[test]
    fn test_trie_single_char() {
        let mut trie = Trie::new();
        trie.insert("a");
        assert!(trie.search("a"));
        assert!(trie.has_prefix("a"));
        assert!(!trie.search("b"));
    }

    #[test]
    fn test_trie_overlapping_words() {
        let mut trie = Trie::new();
        trie.insert("abc");
        trie.insert("ab");
        trie.insert("a");

        assert!(trie.search("a"));
        assert!(trie.search("ab"));
        assert!(trie.search("abc"));
        assert!(trie.has_prefix("a"));
        assert!(trie.has_prefix("ab"));
        assert!(trie.has_prefix("abc"));
    }
}
