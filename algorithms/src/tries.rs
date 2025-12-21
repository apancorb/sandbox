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

#[cfg(test)]
mod tests {
    use super::*;

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
