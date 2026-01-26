use std::collections::{HashMap, HashSet};

/// Pair Sum - Unsorted
///
/// Given an array of integers, return the indexes of any two numbers that add up to a target.
/// The order of the indexes in the result doesn't matter. If no pair is found, return an empty
/// array.
///
/// # Example
///
/// ```text
/// Input: nums = [-1, 3, 4, 2], target = 3
/// Output: [0, 2]
/// Explanation: nums[0] + nums[2] = -1 + 4 = 3
/// ```
///
/// # Constraints
///
/// - The same index cannot be used twice in the result
pub fn pair_sum(nums: &[i32], target: i32) -> Vec<usize> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let val = target - num;
        if let Some(&index) = map.get(&val) {
            return vec![index, i];
        }
        map.insert(num, i);
    }

    vec![]
}

/// Verify Sudoku Board
///
/// Given a partially completed 9x9 Sudoku board, determine if the current state of the board
/// adheres to the rules of the game:
///
/// - Each row and column must contain unique numbers between 1 and 9, or be empty (represented as 0).
/// - Each of the nine 3x3 subgrids that compose the grid must contain unique numbers between 1 and 9, or be empty.
///
/// Note: You are asked to determine whether the current state of the board is valid given
/// these rules, not whether the board is solvable.
///
/// # Constraints
///
/// - Assume each integer on the board falls in the range of [0, 9].
pub fn verify_sudoku(board: &[[i32; 9]; 9]) -> bool {
    let mut set_rows = vec![HashSet::<i32>::new(); 9];
    let mut set_cols = vec![HashSet::<i32>::new(); 9];
    let mut set_subgrids = vec![vec![HashSet::<i32>::new(); 3]; 3];

    for i in 0..9 {
        for j in 0..9 {
            let num = board[i][j];
            if num == 0 {
                continue;
            }

            if set_rows[i].contains(&num) {
                return false;
            }

            if set_cols[j].contains(&num) {
                return false;
            }

            let ii: usize = i / 3;
            let jj: usize = j / 3;
            if set_subgrids[ii][jj].contains(&num) {
                return false;
            }

            set_rows[i].insert(num);
            set_cols[j].insert(num);
            set_subgrids[ii][jj].insert(num);
        }
    }

    true
}

/// Zero Striping
///
/// For each zero in an m x n matrix, set its entire row and column to zero in place.
pub fn zero_striping(matrix: &mut [Vec<i32>]) {
    if matrix.len() == 0 {
        return;
    }

    let mut has_zero_first_row = false;
    for i in 0..matrix[0].len() {
        if matrix[0][i] == 0 {
            has_zero_first_row = true;
            break;
        }
    }

    let mut has_zero_first_col = false;
    for i in 0..matrix.len() {
        if matrix[i][0] == 0 {
            has_zero_first_col = true;
            break;
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let val = matrix[i][j];
            if val == 0 {
                matrix[0][j] = 0;
                matrix[i][0] = 0;
            }
        }
    }

    for i in 1..matrix.len() {
        for j in 1..matrix[i].len() {
            if matrix[0][j] == 0 || matrix[i][0] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    if has_zero_first_row {
        for i in 0..matrix[0].len() {
            matrix[0][i] = 0;
        }
    }

    if has_zero_first_col {
        for i in 0..matrix.len() {
            matrix[i][0] = 0;
        }
    }
}

/// Majority Element
///
/// Given an array of size n, return the majority element. The majority element
/// is the element that appears more than n/2 times.
///
/// Uses Boyer-Moore Voting Algorithm: O(n) time, O(1) space.
///
/// # Example 1
///
/// ```text
/// Input: nums = [3, 2, 3]
/// Output: 3
/// ```
///
/// # Example 2
///
/// ```text
/// Input: nums = [2, 2, 1, 1, 1, 2, 2]
/// Output: 2
/// ```
pub fn majority_element(nums: &[i32]) -> i32 {
    let mut maj = nums[0];
    let mut count = 1;

    for i in 1..nums.len() {
        if nums[i] == maj {
            count += 1;
        } else {
            count -= 1;
        }

        if count == 0 {
            maj = nums[i];
            count = 1;
        }
    }

    maj
}

/// Insert Delete GetRandom O(1)
///
/// Implement a data structure that supports insert, remove, and getRandom in O(1) average time.
///
/// - `insert(val)` - Insert if not present. Returns true if inserted, false otherwise.
/// - `remove(val)` - Remove if present. Returns true if removed, false otherwise.
/// - `get_random()` - Return a random element (uniform probability).
///
/// # Example
///
/// ```text
/// let mut set = RandomizedSet::new();
/// set.insert(1);  // true
/// set.remove(2);  // false (not present)
/// set.insert(2);  // true, set = [1, 2]
/// set.get_random(); // 1 or 2
/// set.remove(1);  // true, set = [2]
/// set.insert(2);  // false (already present)
/// ```
pub struct RandomizedSet {
    map: HashMap<i32, usize>, // val -> index in values
    values: Vec<i32>,         // for O(1) random access
}

impl RandomizedSet {
    pub fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            values: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }

        self.values.push(val);
        self.map.insert(val, self.values.len() - 1);
        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.map.get(&val) {
            let last_index = self.values.len() - 1;
            let last_val = self.values[last_index];

            // Swap with last element
            self.values[index] = last_val;
            self.map.insert(last_val, index);

            // Remove last element
            self.values.pop();
            self.map.remove(&val);
            true
        } else {
            false
        }
    }

    pub fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.values.len());
        self.values[random_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_sum_example() {
        let nums = vec![-1, 3, 4, 2];
        let target = 3;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn test_pair_sum_basic() {
        let nums = vec![-5, -2, 3, 4, 6];
        let target = 7;
        let result = pair_sum(&nums, target);
        // Valid pairs: indices where sum = 7: 3+4=7 or -2+6=7 (if exists), etc.
        assert!(result.len() == 2);
        assert_eq!(nums[result[0]] + nums[result[1]], target);
    }

    #[test]
    fn test_pair_sum_duplicates() {
        let nums = vec![1, 1, 1];
        let target = 2;
        let result = pair_sum(&nums, target);
        // Any valid pair where indices are different
        assert!(result.len() == 2);
        assert_ne!(result[0], result[1]);
        assert_eq!(nums[result[0]] + nums[result[1]], target);
    }

    #[test]
    fn test_pair_sum_no_solution() {
        let nums = vec![1, 2, 3];
        let target = 10;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_pair_sum_negative_numbers() {
        let nums = vec![-10, -5, 0, 5, 10];
        let target = 0;
        let result = pair_sum(&nums, target);
        assert!(result.len() == 2);
        assert_eq!(nums[result[0]] + nums[result[1]], target);
    }

    #[test]
    fn test_pair_sum_two_elements() {
        let nums = vec![1, 9];
        let target = 10;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_pair_sum_empty_array() {
        let nums = vec![];
        let target = 5;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_pair_sum_single_element() {
        let nums = vec![5];
        let target = 5;
        let result = pair_sum(&nums, target);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_pair_sum_large_numbers() {
        let nums = vec![-1000000, 0, 1000000];
        let target = 0;
        let result = pair_sum(&nums, target);
        assert!(result.len() == 2);
        assert_eq!(nums[result[0]] + nums[result[1]], target);
    }

    #[test]
    fn test_verify_sudoku_valid_board() {
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        assert!(verify_sudoku(&board));
    }

    #[test]
    fn test_verify_sudoku_invalid_row() {
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 3, 0], // duplicate 3 in row
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        assert!(!verify_sudoku(&board));
    }

    #[test]
    fn test_verify_sudoku_invalid_column() {
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [5, 0, 0, 0, 8, 0, 0, 7, 9], // duplicate 5 in column 0
        ];
        assert!(!verify_sudoku(&board));
    }

    #[test]
    fn test_verify_sudoku_invalid_subgrid() {
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 5, 0, 0, 0, 0, 6, 0], // duplicate 5 in top-left 3x3
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        assert!(!verify_sudoku(&board));
    }

    #[test]
    fn test_verify_sudoku_empty_board() {
        let board = [[0; 9]; 9];
        assert!(verify_sudoku(&board));
    }

    #[test]
    fn test_verify_sudoku_full_valid_board() {
        let board = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        assert!(verify_sudoku(&board));
    }

    #[test]
    fn test_verify_sudoku_single_value() {
        let mut board = [[0; 9]; 9];
        board[0][0] = 5;
        assert!(verify_sudoku(&board));
    }

    #[test]
    fn test_verify_sudoku_duplicate_zeros_ok() {
        // Zeros (empty cells) can repeat
        let board = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert!(verify_sudoku(&board));
    }

    #[test]
    fn test_zero_striping_basic() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 0, 6], vec![7, 8, 9]];
        zero_striping(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 3], vec![0, 0, 0], vec![7, 0, 9],]);
    }

    #[test]
    fn test_zero_striping_multiple_zeros() {
        let mut matrix = vec![vec![0, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
        zero_striping(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0, 0], vec![0, 5, 0], vec![0, 0, 0],]);
    }

    #[test]
    fn test_zero_striping_no_zeros() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        zero_striping(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]);
    }

    #[test]
    fn test_zero_striping_all_zeros() {
        let mut matrix = vec![vec![0, 0], vec![0, 0]];
        zero_striping(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0], vec![0, 0],]);
    }

    #[test]
    fn test_zero_striping_single_element_zero() {
        let mut matrix = vec![vec![0]];
        zero_striping(&mut matrix);
        assert_eq!(matrix, vec![vec![0]]);
    }

    #[test]
    fn test_zero_striping_single_element_nonzero() {
        let mut matrix = vec![vec![5]];
        zero_striping(&mut matrix);
        assert_eq!(matrix, vec![vec![5]]);
    }

    #[test]
    fn test_zero_striping_empty() {
        let mut matrix: Vec<Vec<i32>> = vec![];
        zero_striping(&mut matrix);
        assert_eq!(matrix, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_zero_striping_single_row() {
        let mut matrix = vec![vec![1, 0, 3, 4]];
        zero_striping(&mut matrix);
        assert_eq!(matrix, vec![vec![0, 0, 0, 0]]);
    }

    #[test]
    fn test_zero_striping_single_column() {
        let mut matrix = vec![vec![1], vec![0], vec![3]];
        zero_striping(&mut matrix);
        assert_eq!(matrix, vec![vec![0], vec![0], vec![0],]);
    }

    #[test]
    fn test_zero_striping_rectangular() {
        let mut matrix = vec![vec![1, 2, 3, 4], vec![5, 0, 7, 8], vec![9, 10, 11, 12]];
        zero_striping(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![1, 0, 3, 4], vec![0, 0, 0, 0], vec![9, 0, 11, 12],]
        );
    }

    #[test]
    fn test_majority_element_example1() {
        assert_eq!(majority_element(&[3, 2, 3]), 3);
    }

    #[test]
    fn test_majority_element_example2() {
        assert_eq!(majority_element(&[2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn test_majority_element_single() {
        assert_eq!(majority_element(&[1]), 1);
    }

    #[test]
    fn test_majority_element_two_same() {
        assert_eq!(majority_element(&[5, 5]), 5);
    }

    #[test]
    fn test_majority_element_all_same() {
        assert_eq!(majority_element(&[7, 7, 7, 7, 7]), 7);
    }

    #[test]
    fn test_majority_element_at_end() {
        assert_eq!(majority_element(&[1, 2, 3, 3, 3]), 3);
    }

    #[test]
    fn test_randomized_set_example() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));
        let rand = set.get_random();
        assert!(rand == 1 || rand == 2);
        assert!(set.remove(1));
        assert!(!set.insert(2));
        assert_eq!(set.get_random(), 2);
    }

    #[test]
    fn test_randomized_set_insert_duplicate() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(5));
        assert!(!set.insert(5));
    }

    #[test]
    fn test_randomized_set_remove_nonexistent() {
        let mut set = RandomizedSet::new();
        assert!(!set.remove(10));
    }

    #[test]
    fn test_randomized_set_insert_remove_insert() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(set.remove(1));
        assert!(set.insert(1));
        assert_eq!(set.get_random(), 1);
    }

    #[test]
    fn test_randomized_set_multiple() {
        let mut set = RandomizedSet::new();
        set.insert(10);
        set.insert(20);
        set.insert(30);
        set.remove(20);
        let rand = set.get_random();
        assert!(rand == 10 || rand == 30);
    }
}
