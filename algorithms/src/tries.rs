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

#[cfg(test)]
mod tests {
    use super::*;

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
