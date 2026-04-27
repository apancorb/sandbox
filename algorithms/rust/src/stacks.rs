/// Valid Parenthesis Expression
///
/// Given a string representing an expression of parentheses containing the
/// characters '(', ')', '[', ']', '{', or '}', determine if the expression
/// forms a valid sequence of parentheses.
///
/// A sequence of parentheses is valid if every opening parenthesis has a
/// corresponding closing parenthesis, and no closing parenthesis appears
/// before its matching opening parenthesis.
///
/// # Examples
///
/// ```text
/// Input: s = "([]{})"
/// Output: true
///
/// Input: s = "([]{)}"
/// Output: false
/// ```
///
/// Push opening brackets onto stack. For closing brackets, check if top
/// of stack matches. If not, or stack empty, invalid.
///
/// Example "([]{})" step by step:
///
/// ```text
/// '(' → push, stack=['(']
/// '[' → push, stack=['(', '[']
/// ']' → pop '[', matches → stack=['(']
/// '{' → push, stack=['(', '{']
/// '}' → pop '{', matches → stack=['(']
/// ')' → pop '(', matches → stack=[]
/// Stack empty → True ✓
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the string
/// - Space: O(n) — stack can hold up to n/2 opening brackets
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
/// Given an integer array nums, return an output array res where, for each
/// value nums[i], res[i] is the first number to the right that's larger than
/// nums[i]. If no larger number exists to the right of nums[i], set res[i] to -1.
///
/// # Examples
///
/// ```text
/// Input: nums = [5, 2, 4, 6, 1]
/// Output: [6, 4, 6, -1, -1]
/// ```
///
/// Iterate from right to left. Maintain a stack of candidates. Pop smaller
/// elements (they can't be the answer for any future element). Top of stack
/// is the answer.
///
/// Example [5, 2, 4, 6, 1]:
///
/// ```text
/// i=4 (1): stack=[], no answer → res[4]=-1, push 1, stack=[1]
/// i=3 (6): pop 1 (1 <= 6), stack=[], no answer → res[3]=-1, push 6, stack=[6]
/// i=2 (4): stack=[6], top=6 > 4 → res[2]=6, push 4, stack=[6, 4]
/// i=1 (2): stack=[6, 4], top=4 > 2 → res[1]=4, push 2, stack=[6, 4, 2]
/// i=0 (5): pop 2 (2 <= 5), pop 4 (4 <= 5), top=6 > 5 → res[0]=6, push 5
/// Result: [6, 4, 6, -1, -1] ✓
/// ```
///
/// # Complexity
///
/// - Time: O(n) — each element pushed/popped at most once
/// - Space: O(n) — stack and result array
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
/// Given a string representing a mathematical expression containing integers,
/// parentheses, addition, and subtraction operators, evaluate and return the
/// result of the expression.
///
/// # Examples
///
/// ```text
/// Input: s = "18-(7+(2-4))"
/// Output: 13
/// ```
///
/// Track current result and sign. When hitting '(', push result and sign
/// onto stack and reset. When hitting ')', pop and combine with outer context.
///
/// Example "18-(7+(2-4))":
///
/// ```text
/// '1' → curr_num=1
/// '8' → curr_num=18
/// '-' → res=18, sign=-1, curr_num=0
/// '(' → push res=18, push sign=-1, stack=[18, -1], res=0, sign=1
/// '7' → curr_num=7
/// '+' → res=7, sign=1, curr_num=0
/// '(' → push res=7, push sign=1, stack=[18, -1, 7, 1], res=0, sign=1
/// '2' → curr_num=2
/// '-' → res=2, sign=-1, curr_num=0
/// '4' → curr_num=4
/// ')' → res=2+4*(-1)=-2, pop sign=1, pop prev=7, res=7+(-2)*1=5
/// ')' → res=5+0*1=5, pop sign=-1, pop prev=18, res=18+5*(-1)=13
/// Answer: 13 ✓
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass through the string
/// - Space: O(n) — stack for nested parentheses
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
