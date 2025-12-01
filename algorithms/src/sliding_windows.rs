use std::collections::HashMap;

/// Substring Anagrams
///
/// Given two strings, s and t, both consisting of lowercase English letters, return the number
/// of substrings in s that are anagrams of t.
///
/// An anagram is a word or phrase formed by rearranging the letters of another word or
/// phrase, using all the original letters exactly once.
///
/// # Example
///
/// ```text
/// Input: s = "caabab", t = "aba"
/// Output: 2
/// Explanation: There is an anagram of t starting at index 1 ("caabab") and another starting at
/// index 2 ("caabab")
/// ```
pub fn count_anagrams(s: &str, t: &str) -> usize {
    if t.len() > s.len() {
        return 0;
    }

    let mut expected = [0; 26];
    let mut window = [0; 26];

    let mut left = 0;
    let mut right = 0;
    let mut res = 0;

    for c in t.chars() {
        expected[c as usize - 'a' as usize] += 1;
    }

    while right < s.len() {
        let right_char = s.chars().nth(right).unwrap();
        window[right_char as usize - 'a' as usize] += 1;

        if right - left + 1 == t.len() {
            if expected == window {
                res += 1;
            }
            let left_char = s.chars().nth(left).unwrap();
            window[left_char as usize - 'a' as usize] -= 1;
            left += 1;
        }

        right += 1;
    }

    res
}

/// Longest Substring With Unique Characters
///
/// Given a string, determine the length of its longest substring that consists only of unique
/// characters.
///
/// # Example
///
/// ```text
/// Input: s = "abcba"
/// Output: 3
/// Explanation: Substring "abc" is the longest substring of length 3 that contains unique
/// characters ("cba" also fits this description)
/// ```
pub fn longest_unique_substring(s: &str) -> usize {
    let mut map = HashMap::new();
    let mut max = 0;
    let mut left = 0;
    let mut right = 0;

    while right < s.len() {
        let char = s.chars().nth(right).unwrap();
        if let Some(&prev_index) = map.get(&char) {
            left = left.max(prev_index + 1);
        }

        max = max.max(right - left + 1);
        map.insert(char, right);
        right += 1;
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_anagrams_example() {
        assert_eq!(count_anagrams("caabab", "aba"), 2);
    }

    #[test]
    fn test_count_anagrams_no_match() {
        assert_eq!(count_anagrams("abcdef", "xyz"), 0);
    }

    #[test]
    fn test_count_anagrams_all_same_char() {
        assert_eq!(count_anagrams("aaaa", "aa"), 3);
    }

    #[test]
    fn test_count_anagrams_exact_match() {
        assert_eq!(count_anagrams("abc", "abc"), 1);
    }

    #[test]
    fn test_count_anagrams_single_char() {
        assert_eq!(count_anagrams("ababa", "a"), 3);
    }

    #[test]
    fn test_count_anagrams_t_longer_than_s() {
        assert_eq!(count_anagrams("ab", "abc"), 0);
    }

    #[test]
    fn test_count_anagrams_empty_s() {
        assert_eq!(count_anagrams("", "abc"), 0);
    }

    #[test]
    fn test_count_anagrams_empty_t() {
        assert_eq!(count_anagrams("abc", ""), 0);
    }

    #[test]
    fn test_count_anagrams_multiple_matches() {
        // "ab", "ba", "ab" are all anagrams of "ab"
        assert_eq!(count_anagrams("abab", "ab"), 3);
    }

    #[test]
    fn test_count_anagrams_overlapping() {
        assert_eq!(count_anagrams("cbaebabacd", "abc"), 2);
    }

    // longest_unique_substring tests

    #[test]
    fn test_longest_unique_substring_example() {
        assert_eq!(longest_unique_substring("abcba"), 3);
    }

    #[test]
    fn test_longest_unique_substring_all_unique() {
        assert_eq!(longest_unique_substring("abcdef"), 6);
    }

    #[test]
    fn test_longest_unique_substring_all_same() {
        assert_eq!(longest_unique_substring("aaaa"), 1);
    }

    #[test]
    fn test_longest_unique_substring_empty() {
        assert_eq!(longest_unique_substring(""), 0);
    }

    #[test]
    fn test_longest_unique_substring_single_char() {
        assert_eq!(longest_unique_substring("a"), 1);
    }

    #[test]
    fn test_longest_unique_substring_two_chars() {
        assert_eq!(longest_unique_substring("ab"), 2);
    }

    #[test]
    fn test_longest_unique_substring_repeating_pattern() {
        assert_eq!(longest_unique_substring("abcabcbb"), 3);
    }

    #[test]
    fn test_longest_unique_substring_end_longest() {
        assert_eq!(longest_unique_substring("aabcdef"), 6);
    }

    #[test]
    fn test_longest_unique_substring_middle_longest() {
        assert_eq!(longest_unique_substring("aaabcdefa"), 6);
    }
}
