use std::collections::HashSet;

/// Find All Permutations
///
/// Return all permutations of a list of unique integers.
///
/// # Examples
///
/// ```text
/// Input: nums = [4, 5, 6]
/// Output: [[4, 5, 6], [4, 6, 5], [5, 4, 6], [5, 6, 4], [6, 4, 5], [6, 5, 4]]
/// ```
///
/// At each level, try every unused number. Use a visited set to skip
/// numbers already in the current permutation.
///
/// Tree for [4, 5, 6]:
///
/// ```text
///                     []
///           /          |          \
///        [4]          [5]         [6]
///       /   \       /   \       /   \
///    [4,5] [4,6] [5,4] [5,6] [6,4] [6,5]
///      |     |     |     |     |     |
///   [4,5,6] ...   ...   ...   ...  [6,5,4]
/// ```
///
/// # Complexity
///
/// - Time: O(n! * n) — n! permutations, each takes O(n) to copy
/// - Space: O(n) recursion depth + O(n) visited set
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
/// Return all subsets (the power set) of a list of unique integers.
///
/// # Examples
///
/// ```text
/// Input: nums = [4, 5, 6]
/// Output: [[], [4], [4, 5], [4, 5, 6], [4, 6], [5], [5, 6], [6]]
/// ```
///
/// At each index, make a binary choice: include or exclude nums[i].
/// Then recurse to the next index. When we reach the end, save the subset.
///
/// Tree for [4, 5, 6]:
///
/// ```text
///                       []
///                /              \
///           include 4          exclude 4
///            [4]                  []
///          /      \            /      \
///      inc 5    exc 5      inc 5    exc 5
///      [4,5]    [4]        [5]       []
///      /  \    /  \      /  \    /  \
///    +6  -6  +6  -6    +6  -6  +6  -6
/// [4,5,6][4,5][4,6][4] [5,6][5] [6] []
/// ```
///
/// # Complexity
///
/// - Time: O(2^n * n) — 2^n subsets, each up to O(n) to copy
/// - Space: O(n) recursion depth
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
/// Place n queens on an n*n board so no two attack each other.
/// Return the number of valid configurations.
///
/// # Examples
///
/// ```text
/// Input: n = 4
/// Output: 2
/// ```
///
/// Place one queen per row (they can't share a row). For each row,
/// try every column. Skip if column, diagonal, or anti-diagonal
/// is already occupied.
///
/// How diagonals work:
///
/// ```text
/// Diagonal (top-left to bottom-right): r - c is constant
///     (e.g. (0,1) and (1,2) both = -1)
/// Anti-diagonal (top-right to bottom-left): r + c is constant
///     (e.g. (0,1) and (1,0) both = 1)
/// ```
///
/// Example n=4, one solution:
///
/// ```text
/// . Q . .    row 0, col 1
/// . . . Q    row 1, col 3
/// Q . . .    row 2, col 0
/// . . Q .    row 3, col 2
/// ```
///
/// # Complexity
///
/// - Time: O(n!) — pruning reduces branching at each row
/// - Space: O(n) for the three sets + recursion
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
