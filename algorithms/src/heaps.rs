/// K Most Frequent Strings
///
/// Find the k most frequently occurring strings in an array, and return them sorted by
/// frequency in descending order. If two strings have the same frequency, sort them in
/// lexicographical order.
///
/// # Example
///
/// ```text
/// Input: strs = ["go", "coding", "byte", "byte", "go", "interview", "go"], k = 2
/// Output: ["go", "byte"]
/// Explanation: The strings "go" and "byte" appear the most frequently, with frequencies of
/// 3 and 2, respectively.
/// ```
///
/// # Constraints
///
/// - k <= n, where n denotes the length of the array.
pub fn k_most_frequent_strings(strs: &[&str], k: usize) -> Vec<String> {
    todo!("Implement k_most_frequent_strings")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_most_frequent_strings_example() {
        assert_eq!(
            k_most_frequent_strings(
                &["go", "coding", "byte", "byte", "go", "interview", "go"],
                2
            ),
            vec!["go", "byte"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_single() {
        assert_eq!(
            k_most_frequent_strings(&["a", "b", "c", "a", "a"], 1),
            vec!["a"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_all_same_freq() {
        // When frequencies are equal, sort lexicographically
        assert_eq!(
            k_most_frequent_strings(&["c", "b", "a"], 2),
            vec!["a", "b"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_k_equals_n() {
        assert_eq!(
            k_most_frequent_strings(&["x", "y", "z"], 3),
            vec!["x", "y", "z"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_tie_breaker() {
        // "apple" and "banana" both have freq 2, so lexicographical order
        assert_eq!(
            k_most_frequent_strings(&["apple", "banana", "apple", "banana", "cherry"], 2),
            vec!["apple", "banana"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_empty_k_zero() {
        assert_eq!(k_most_frequent_strings(&["a", "b", "c"], 0), Vec::<String>::new());
    }

    #[test]
    fn test_k_most_frequent_strings_all_same() {
        assert_eq!(
            k_most_frequent_strings(&["same", "same", "same"], 1),
            vec!["same"]
        );
    }

    #[test]
    fn test_k_most_frequent_strings_mixed_frequencies() {
        // a: 4, b: 3, c: 2, d: 1
        assert_eq!(
            k_most_frequent_strings(&["a", "a", "a", "a", "b", "b", "b", "c", "c", "d"], 3),
            vec!["a", "b", "c"]
        );
    }
}
