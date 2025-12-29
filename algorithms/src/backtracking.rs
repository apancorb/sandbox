use std::collections::HashSet;

/// Find All Permutations
///
/// Return all possible permutations of a given array of unique integers. They can be returned
/// in any order.
///
/// # Example
///
/// ```text
/// Input: nums = [4, 5, 6]
///
/// Output: [[4, 5, 6], [4, 6, 5], [5, 4, 6], [5, 6, 4], [6, 4, 5], [6, 5, 4]]
/// ```
pub fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    fn permutations_helper(
        res: &mut Vec<Vec<i32>>,
        nums: &[i32],
        candidate: &mut Vec<i32>,
        visited: &mut HashSet<i32>,
    ) {
        if candidate.len() == nums.len() {
            res.push(candidate.clone());
            return;
        }

        for &n in nums {
            if !visited.contains(&n) {
                candidate.push(n);
                visited.insert(n);
                permutations_helper(res, nums, candidate, visited);
                candidate.pop();
                visited.remove(&n);
            }
        }
    }

    let mut res = Vec::new();
    let mut candidate = Vec::new();
    let mut visited = HashSet::new();

    permutations_helper(&mut res, nums, &mut candidate, &mut visited);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_example() {
        let mut result = permutations(&[4, 5, 6]);
        result.sort();
        let mut expected = vec![
            vec![4, 5, 6],
            vec![4, 6, 5],
            vec![5, 4, 6],
            vec![5, 6, 4],
            vec![6, 4, 5],
            vec![6, 5, 4],
        ];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permutations_single() {
        assert_eq!(permutations(&[1]), vec![vec![1]]);
    }

    #[test]
    fn test_permutations_two() {
        let mut result = permutations(&[1, 2]);
        result.sort();
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_permutations_empty() {
        assert_eq!(permutations(&[]), vec![vec![]]);
    }

    #[test]
    fn test_permutations_count() {
        // 4! = 24 permutations
        let result = permutations(&[1, 2, 3, 4]);
        assert_eq!(result.len(), 24);
    }
}
