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

/// Find All Subsets
///
/// Return all possible subsets of a given set of unique integers. Each subset can be ordered
/// in any way, and the subsets can be returned in any order.
///
/// # Example
///
/// ```text
/// Input: nums = [4, 5, 6]
///
/// Output: [[], [4], [4, 5], [4, 5, 6], [4, 6], [5], [5, 6], [6]]
/// ```
pub fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    fn subsets_helper(nums: &[i32], res: &mut Vec<Vec<i32>>, curr_subset: &mut Vec<i32>, i: usize) {
        if i == nums.len() {
            res.push(curr_subset.clone());
            return;
        }

        curr_subset.push(nums[i]);
        subsets_helper(nums, res, curr_subset, i + 1);
        curr_subset.pop();
        subsets_helper(nums, res, curr_subset, i + 1);
    }

    let mut res = Vec::new();
    let mut curr_subset = Vec::new();
    subsets_helper(nums, &mut res, &mut curr_subset, 0);
    res
}

/// N Queens
///
/// There is a chessboard of size n x n. Your goal is to place n queens on the board such that
/// no two queens attack each other. Return the number of distinct configurations where this
/// is possible.
///
/// # Example
///
/// ```text
/// Input: n = 4
///
/// Output: 2
/// ```
pub fn n_queens(n: usize) -> usize {
    fn n_queens_helper(
        n: usize,
        r: usize,
        res: &mut usize,
        cols: &mut HashSet<usize>,
        diagonals: &mut HashSet<isize>,
        anti_diagonals: &mut HashSet<usize>,
    ) {
        if r == n {
            *res += 1;
            return;
        }

        for c in 0..n {
            let curr_diagonal = (r as isize) - (c as isize);
            let curr_anti_diagonal = r + c;

            if cols.contains(&c)
                || diagonals.contains(&curr_diagonal)
                || anti_diagonals.contains(&curr_anti_diagonal)
            {
                continue;
            }

            cols.insert(c);
            diagonals.insert(curr_diagonal);
            anti_diagonals.insert(curr_anti_diagonal);

            n_queens_helper(n, r + 1, res, cols, diagonals, anti_diagonals);

            cols.remove(&c);
            diagonals.remove(&curr_diagonal);
            anti_diagonals.remove(&curr_anti_diagonal);
        }
    }

    let mut res = 0;
    n_queens_helper(
        n,
        0,
        &mut res,
        &mut HashSet::new(),
        &mut HashSet::new(),
        &mut HashSet::new(),
    );
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

    #[test]
    fn test_subsets_example() {
        let mut result = subsets(&[4, 5, 6]);
        result.iter_mut().for_each(|s| s.sort());
        result.sort();
        let mut expected = vec![
            vec![],
            vec![4],
            vec![4, 5],
            vec![4, 5, 6],
            vec![4, 6],
            vec![5],
            vec![5, 6],
            vec![6],
        ];
        expected.iter_mut().for_each(|s| s.sort());
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subsets_empty() {
        assert_eq!(subsets(&[]), vec![vec![]]);
    }

    #[test]
    fn test_subsets_single() {
        let mut result = subsets(&[1]);
        result.sort();
        assert_eq!(result, vec![vec![], vec![1]]);
    }

    #[test]
    fn test_subsets_two() {
        let mut result = subsets(&[1, 2]);
        result.iter_mut().for_each(|s| s.sort());
        result.sort();
        assert_eq!(result, vec![vec![], vec![1], vec![1, 2], vec![2]]);
    }

    #[test]
    fn test_subsets_count() {
        // 2^4 = 16 subsets
        let result = subsets(&[1, 2, 3, 4]);
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_n_queens_4() {
        assert_eq!(n_queens(4), 2);
    }

    #[test]
    fn test_n_queens_1() {
        assert_eq!(n_queens(1), 1);
    }

    #[test]
    fn test_n_queens_2() {
        assert_eq!(n_queens(2), 0);
    }

    #[test]
    fn test_n_queens_3() {
        assert_eq!(n_queens(3), 0);
    }

    #[test]
    fn test_n_queens_8() {
        assert_eq!(n_queens(8), 92);
    }
}
