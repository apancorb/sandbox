/// Valid Parenthesis Expression
///
/// Given a string representing an expression of parentheses containing the characters '(', ')',
/// '[', ']', '{', or '}', determine if the expression forms a valid sequence of parentheses.
///
/// A sequence of parentheses is valid if every opening parenthesis has a corresponding closing
/// parenthesis, and no closing parenthesis appears before its matching opening parenthesis.
///
/// # Examples
///
/// Example 1:
/// ```text
/// Input: s = "([]{})"
/// Output: true
/// ```
///
/// Example 2:
/// ```text
/// Input: s = "([]{)}"
/// Output: false
/// Explanation: The '(' parenthesis is closed before its nested '{' parenthesis is closed.
/// ```
pub fn is_valid_parentheses(s: &str) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_parentheses_example_1() {
        assert!(is_valid_parentheses("([]{})"));
    }

    #[test]
    fn test_is_valid_parentheses_example_2() {
        assert!(!is_valid_parentheses("([]{)}"));
    }

    #[test]
    fn test_is_valid_parentheses_empty() {
        assert!(is_valid_parentheses(""));
    }

    #[test]
    fn test_is_valid_parentheses_single_pair() {
        assert!(is_valid_parentheses("()"));
        assert!(is_valid_parentheses("[]"));
        assert!(is_valid_parentheses("{}"));
    }

    #[test]
    fn test_is_valid_parentheses_nested() {
        assert!(is_valid_parentheses("({[]})"));
    }

    #[test]
    fn test_is_valid_parentheses_sequential() {
        assert!(is_valid_parentheses("()[]{}"));
    }

    #[test]
    fn test_is_valid_parentheses_unmatched_open() {
        assert!(!is_valid_parentheses("("));
        assert!(!is_valid_parentheses("(()"));
    }

    #[test]
    fn test_is_valid_parentheses_unmatched_close() {
        assert!(!is_valid_parentheses(")"));
        assert!(!is_valid_parentheses("())"));
    }

    #[test]
    fn test_is_valid_parentheses_wrong_order() {
        assert!(!is_valid_parentheses(")("));
        assert!(!is_valid_parentheses("([)]"));
    }

    #[test]
    fn test_is_valid_parentheses_mismatched_types() {
        assert!(!is_valid_parentheses("(]"));
        assert!(!is_valid_parentheses("[}"));
        assert!(!is_valid_parentheses("{)"));
    }

    #[test]
    fn test_is_valid_parentheses_complex_valid() {
        assert!(is_valid_parentheses("{[()]}()[{}]"));
    }

    #[test]
    fn test_is_valid_parentheses_complex_invalid() {
        assert!(!is_valid_parentheses("{[(])}"));
    }
}
