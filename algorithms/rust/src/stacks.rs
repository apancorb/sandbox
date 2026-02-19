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

/// Next Largest Number to the Right
///
/// Given an integer array nums, return an output array res where, for each value nums[i],
/// res[i] is the first number to the right that's larger than nums[i]. If no larger number exists
/// to the right of nums[i], set res[i] to -1.
///
/// # Example
///
/// ```text
/// Input: nums = [5, 2, 4, 6, 1]
/// Output: [6, 4, 6, -1, -1]
/// ```
pub fn next_largest_to_right(nums: &[i32]) -> Vec<i32> {
    let mut res = vec![-1; nums.len()];
    let mut stack = Vec::new();

    for i in (0..nums.len()).rev() {
        while let Some(&top) = stack.last() {
            if top <= nums[i] {
                stack.pop();
            } else {
                break;
            }
        }

        if let Some(&top) = stack.last() {
            res[i] = top;
        }

        stack.push(nums[i]);
    }

    res
}

/// Evaluate Expression
///
/// Given a string representing a mathematical expression containing integers, parentheses,
/// addition, and subtraction operators, evaluate and return the result of the expression.
///
/// # Example
///
/// ```text
/// Input: s = "18-(7+(2-4))"
/// Output: 13
/// ```
pub fn evaluate_expression(s: &str) -> i32 {
    let mut res: i32 = 0;
    let mut curr_num: i32 = 0;
    let mut sign: i32 = 1;
    let mut stack: Vec<i32> = Vec::new();

    for c in s.chars() {
        if c.is_ascii_digit() {
            curr_num = curr_num * 10 + c.to_digit(10).unwrap() as i32;
        } else if c == '+' || c == '-' {
            res += curr_num * sign;
            curr_num = 0;
            sign = if c == '+' { 1 } else { -1 };
        } else if c == '(' {
            stack.push(res);
            stack.push(sign);
            res = 0;
            sign = 1;
        } else if c == ')' {
            res += curr_num * sign;
            let prev_sign = stack.pop().unwrap();
            let prev_res =  stack.pop().unwrap();
            res = prev_res + res * prev_sign;
            curr_num = 0;
            sign = 1;
        }
    }

    res + curr_num * sign
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

    // next_largest_to_right tests

    #[test]
    fn test_next_largest_to_right_example() {
        assert_eq!(
            next_largest_to_right(&[5, 2, 4, 6, 1]),
            vec![6, 4, 6, -1, -1]
        );
    }

    #[test]
    fn test_next_largest_to_right_empty() {
        assert_eq!(next_largest_to_right(&[]), vec![]);
    }

    #[test]
    fn test_next_largest_to_right_single() {
        assert_eq!(next_largest_to_right(&[5]), vec![-1]);
    }

    #[test]
    fn test_next_largest_to_right_increasing() {
        assert_eq!(
            next_largest_to_right(&[1, 2, 3, 4, 5]),
            vec![2, 3, 4, 5, -1]
        );
    }

    #[test]
    fn test_next_largest_to_right_decreasing() {
        assert_eq!(
            next_largest_to_right(&[5, 4, 3, 2, 1]),
            vec![-1, -1, -1, -1, -1]
        );
    }

    #[test]
    fn test_next_largest_to_right_all_same() {
        assert_eq!(next_largest_to_right(&[3, 3, 3, 3]), vec![-1, -1, -1, -1]);
    }

    #[test]
    fn test_next_largest_to_right_two_elements() {
        assert_eq!(next_largest_to_right(&[1, 2]), vec![2, -1]);
        assert_eq!(next_largest_to_right(&[2, 1]), vec![-1, -1]);
    }

    #[test]
    fn test_next_largest_to_right_duplicates() {
        assert_eq!(
            next_largest_to_right(&[2, 1, 2, 4, 3]),
            vec![4, 2, 4, -1, -1]
        );
    }

    // evaluate_expression tests

    #[test]
    fn test_evaluate_expression_example() {
        assert_eq!(evaluate_expression("18-(7+(2-4))"), 13);
    }

    #[test]
    fn test_evaluate_expression_simple_addition() {
        assert_eq!(evaluate_expression("1+2+3"), 6);
    }

    #[test]
    fn test_evaluate_expression_simple_subtraction() {
        assert_eq!(evaluate_expression("10-3-2"), 5);
    }

    #[test]
    fn test_evaluate_expression_single_number() {
        assert_eq!(evaluate_expression("42"), 42);
    }

    #[test]
    fn test_evaluate_expression_with_spaces() {
        assert_eq!(evaluate_expression("1 + 2 - 3"), 0);
    }

    #[test]
    fn test_evaluate_expression_nested_parens() {
        assert_eq!(evaluate_expression("((1+2))"), 3);
    }

    #[test]
    fn test_evaluate_expression_negation_by_paren() {
        assert_eq!(evaluate_expression("1-(2+3)"), -4);
    }

    #[test]
    fn test_evaluate_expression_double_negation() {
        assert_eq!(evaluate_expression("1-(-2)"), 3);
    }

    #[test]
    fn test_evaluate_expression_complex() {
        assert_eq!(evaluate_expression("2-(5-6)"), 3);
    }

    #[test]
    fn test_evaluate_expression_leading_minus() {
        assert_eq!(evaluate_expression("-1+2"), 1);
    }
}
