//! Searching and Sorting Pattern
//!
//! A collection of searching and sorting algorithm problems.
//!
//! Sections:
//! - Binary Search: problems using binary search on sorted arrays and answer ranges
//! - Sorting: fundamental sorting algorithms and merge operations

// =============================================================================
// Binary Search
// =============================================================================

/// Find the Insertion Index
///
/// Given a sorted array with unique values and a target:
/// - If target exists, return its index
/// - Otherwise, return where it would be inserted to maintain order
///
/// # Examples
///
/// ```text
/// Input: nums = [1, 2, 4, 5, 7, 8, 9], target = 4
/// Output: 2
/// ```
///
/// ```text
/// Input: nums = [1, 2, 4, 5, 7, 8, 9], target = 6
/// Output: 4  (6 would go between 5 and 7)
/// ```
///
/// Standard binary search. When target not found, left pointer ends up
/// at the insertion position.
///
/// Example walkthrough for [1, 2, 4, 5, 7, 8, 9], target=6:
///
/// ```text
/// left=0, right=7, mid=3 → nums[3]=5 < 6 → left=4
/// left=4, right=7, mid=5 → nums[5]=8 >= 6 → right=5
/// left=4, right=5, mid=4 → nums[4]=7 >= 6 → right=4
/// left==right=4 → return 4 (between 5 and 7)
/// ```
///
/// # Complexity
///
/// - Time: O(log n) — binary search
/// - Space: O(1) — only pointers
pub fn find_insertion_index(nums: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = (right + left) / 2;
        if nums[mid] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

/// First and Last Occurrences of a Number
///
/// Given an array sorted in non-decreasing order, return the first and last
/// indexes of a target number. If not found, return [-1, -1].
///
/// # Examples
///
/// ```text
/// Input: nums = [1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], target = 4
/// Output: [3, 5]
/// ```
///
/// Run binary search twice: once to find leftmost occurrence (lower bound),
/// once to find rightmost occurrence (upper bound).
///
/// For find_lower (find FIRST occurrence): when mid lands on target, it could
/// be the first, so right=mid (keep it). Keeps shrinking right until
/// left==right at first target.
///
/// For find_upper (find LAST occurrence): when mid lands on target, it could
/// be the last, so left=mid (keep it). Why +1 bias? Without it, when left=3,
/// right=4: mid = (3+4)//2 = 3, then left=mid=3 → infinite loop! With +1:
/// mid = (3+4)//2 + 1 = 4, then left=mid=4 → left==right, done.
///
/// Example walkthrough for [1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], target=4:
///
/// ```text
/// find_lower: narrows right when nums[mid] >= 4
///     mid=6 → 5 >= 4, right=6 → mid=3 → 4 >= 4, right=3
///     → left==right=3, nums[3]=4 ✓ → first=3
/// find_upper: narrows left when nums[mid] <= 4 (with +1 bias)
///     mid=7 → 6 > 4, right=6 → mid=3 → 4 <= 4, left=3
///     mid=5 → 4 <= 4, left=5 → mid=6 → 5 > 4, right=5
///     → left==right=5, nums[5]=4 ✓ → last=5
/// Result: [3, 5]
/// ```
///
/// # Complexity
///
/// - Time: O(log n) — two binary searches
/// - Space: O(1) — only pointers
pub fn find_first_and_last(nums: &[i32], target: i32) -> [i32; 2] {
    if nums.is_empty() {
        return [-1, -1];
    }

    let find_lower_bound = |nums: &[i32], target: i32| -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (right + left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid - 1;
            } else {
                right = mid;
            }
        }

        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    };
    let find_upper_bound = |nums: &[i32], target: i32| -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = ((right + left) / 2) + 1;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid - 1;
            } else {
                left = mid;
            }
        }

        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    };

    [
        find_lower_bound(nums, target),
        find_upper_bound(nums, target),
    ]
}

/// Cutting Wood
///
/// Given tree heights and amount of wood needed (k), find the highest
/// sawblade height that collects at least k wood. Each tree taller than
/// the blade contributes (tree_height - blade_height) wood.
///
/// # Examples
///
/// ```text
/// Input: heights = [2, 6, 3, 8], k = 7
/// Output: 3  (Cut at height 3: (6-3)+(8-3) = 3+5 = 8 >= 7)
/// ```
///
/// Binary search on the answer. Search for highest blade height that
/// still gives enough wood. This is monotonic: lower blade means more
/// wood, higher blade means less wood.
///
/// Instead of searching for a value in an array, we search for the
/// best answer in a range [0, max_height]. Key insight: this is monotonic!
/// - Lower blade = more wood collected
/// - Higher blade = less wood collected
/// - If blade=3 gives enough, blade=2 also gives enough
/// - If blade=5 doesn't give enough, blade=6 also won't
///
/// We use a +1 bias on mid since we want the highest valid answer.
///
/// Example walkthrough for heights=[2, 6, 3, 8], k=7, search range [0, 8]:
///
/// ```text
/// mid=5: wood = (6-5)+(8-5) = 1+3 = 4 < 7   → right=4
/// mid=3: wood = (6-3)+(8-3) = 3+5 = 8 >= 7  → left=3
/// mid=4: wood = (6-4)+(8-4) = 2+4 = 6 < 7   → right=3
/// left==right=3 → answer is 3
/// ```
///
/// # Complexity
///
/// - Time: O(n log h) — binary search on height, O(n) check each
/// - Space: O(1)
pub fn cutting_wood(heights: &[i32], k: i32) -> usize {
    let has_enough_wood = |heights: &[i32], target_height: i32, k: i32| -> bool {
        let mut wood_collected = 0;

        for &height in heights {
            if height > target_height {
                wood_collected += height - target_height;
            }
        }

        wood_collected >= k
    };

    let mut left = 0;
    let mut right = *heights.iter().max().unwrap() as usize;

    while left < right {
        let mid = ((right + left) / 2) + 1;

        if has_enough_wood(heights, mid as i32, k) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

/// Find Target in a Rotated Sorted Array
///
/// A rotated sorted array is a sorted array where a portion is moved from
/// the beginning to the end. Example: [1,2,3,4,5] -> [3,4,5,1,2]
///
/// Given a rotated sorted array of unique numbers, return the index of target.
/// Return -1 if not found.
///
/// # Examples
///
/// ```text
/// Input: nums = [8, 9, 1, 2, 3, 4, 5, 6, 7], target = 1
/// Output: 2
/// ```
///
/// Key insight: At any mid point, ONE half is always sorted:
///
/// ```text
/// [8, 9, 1, 2, 3, 4, 5, 6, 7]
///       ^mid
/// Left half [8,9,1] is NOT sorted (8 > 1)
/// Right half [2,3,4,5,6,7] IS sorted (2 < 7)
/// ```
///
/// Strategy:
/// 1. Find which half is sorted (compare endpoints)
/// 2. Check if target is in the sorted half (easy range check)
/// 3. If yes, search that half. If no, search the other half.
///
/// # Complexity
///
/// - Time: O(log n) — binary search
/// - Space: O(1)
pub fn find_in_rotated_array(nums: &[i32], target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = (right + left) / 2;

        if nums[mid] == target {
            return mid as i32;
        } else if nums[left] <= nums[mid] {
            if nums[left] <= target && target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    if nums[left] == target {
        left as i32
    } else {
        -1
    }
}

// =============================================================================
// Sorting
// =============================================================================

/// Sort Array (Quicksort)
///
/// Sort an integer array in ascending order using quicksort.
///
/// # Examples
///
/// ```text
/// Input: nums = [6, 8, 4, 2, 7, 3, 1, 5]
/// Output: [1, 2, 3, 4, 5, 6, 7, 8]
/// ```
///
/// Quicksort steps:
/// 1. Pick a pivot (we use last element)
/// 2. Partition: move smaller elements left, larger right
/// 3. Recursively sort left and right partitions
///
/// Partition example for [6,8,4,2,7,3,1,5], pivot=5:
///
/// `lo` marks where the next "small" element should go.
/// Scan left to right; when we find something < pivot, swap it to lo.
///
/// ```text
/// [6, 8, 4, 2, 7, 3, 1, 5]    pivot=5
///  ^lo
///  i=0: 6 >= 5? yes, skip
///
/// [6, 8, 4, 2, 7, 3, 1, 5]
///  ^lo
///  i=1: 8 >= 5? yes, skip
///
/// [6, 8, 4, 2, 7, 3, 1, 5]
///  ^lo
///  i=2: 4 < 5? yes! swap arr[lo] with arr[i]
///
/// [4, 8, 6, 2, 7, 3, 1, 5]    swapped 6↔4
///     ^lo
///  i=3: 2 < 5? yes! swap
///
/// [4, 2, 6, 8, 7, 3, 1, 5]    swapped 8↔2
///        ^lo
///  i=4: 7 >= 5? yes, skip
///  i=5: 3 < 5? yes! swap
///
/// [4, 2, 3, 8, 7, 6, 1, 5]    swapped 6↔3
///           ^lo
///  i=6: 1 < 5? yes! swap
///
/// [4, 2, 3, 1, 7, 6, 8, 5]    swapped 8↔1
///              ^lo
///  Done scanning. Now put pivot at lo:
///
/// [4, 2, 3, 1, 5, 6, 8, 7]    swapped 7↔5
///  ←--<5---→   ↑  ←->=5-→
///            pivot in final position!
///
///  Recurse on left [4,2,3,1] and right [6,8,7]
/// ```
///
/// # Complexity
///
/// - Time: O(n log n) average, O(n²) worst — bad pivots (always min/max)
///   create n levels instead of log(n). Happens with already sorted arrays
///   when using last element as pivot.
/// - Space: O(log n) average (recursion stack), O(n) worst
pub fn sort_array(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }
    quicksort(nums, 0, nums.len() - 1);
}

fn quicksort(nums: &mut [i32], left: usize, right: usize) {
    // Base case: if the subarray has 0 or 1 element, it's already sorted
    if left >= right {
        return;
    }

    // Partition the array and retrieve the pivot index
    let pivot_index = partition(nums, left, right);

    // Recursively sort left and right parts
    if pivot_index > 0 {
        quicksort(nums, left, pivot_index - 1);
    }
    quicksort(nums, pivot_index + 1, right);
}

fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = nums[right];
    let mut lo = left;

    // Move all numbers less than pivot to the left
    for i in left..right {
        if nums[i] < pivot {
            nums.swap(lo, i);
            lo += 1;
        }
    }

    // Swap pivot into its correct position
    nums.swap(lo, right);
    lo
}

/// Merge Sorted Array
///
/// Merge nums2 into nums1. nums1 has length m+n (last n elements are 0s).
/// Modify nums1 in-place.
///
/// # Examples
///
/// ```text
/// Input: nums1 = [1, 2, 3, 0, 0, 0], m = 3, nums2 = [2, 5, 6], n = 3
/// Output: nums1 = [1, 2, 2, 3, 5, 6]
/// ```
///
/// Two pointer merge: compare elements from both arrays,
/// pick the smaller one each time.
///
/// Example walkthrough for nums1=[1,3,5,0,0,0] m=3, nums2=[2,4,6] n=3:
///
/// ```text
/// copy1 = [1,3,5]
/// i=0: 1 <= 2 → take 1
/// i=1: 3 > 2  → take 2
/// i=2: 3 <= 4 → take 3
/// i=3: 5 > 4  → take 4
/// i=4: 5 <= 6 → take 5
/// i=5: done   → take 6
/// Result: [1,2,3,4,5,6]
/// ```
///
/// # Complexity
///
/// - Time: O(m + n)
/// - Space: O(m) — copy of nums1
pub fn merge_sorted_array(nums1: &mut [i32], m: usize, nums2: &[i32], n: usize) {
    let copy_nums1 = nums1[0..m].to_vec();
    let mut p1 = 0;
    let mut p2 = 0;

    for i in 0..(m + n) {
        if p2 >= n || (p1 < m && copy_nums1[p1] <= nums2[p2]) {
            nums1[i] = copy_nums1[p1];
            p1 += 1;
        } else {
            nums1[i] = nums2[p2];
            p2 += 1;
        }
    }
}

/// H-Index
///
/// Given an array where citations[i] is the number of citations for the ith paper,
/// return the h-index: the maximum h such that at least h papers have at least
/// h citations each.
///
/// # Examples
///
/// ```text
/// Input: citations = [3, 0, 6, 1, 5]
/// Output: 3
/// Sorted: [0, 1, 3, 5, 6]
/// 3 papers have >= 3 citations (papers with 3, 5, 6)
/// ```
///
/// Sort ascending then scan. At index i, there are (n - i) papers
/// with at least citations[i] citations. If citations[i] >= (n - i),
/// that's a valid h-index.
///
/// Example walkthrough for [3, 0, 6, 1, 5] → sorted: [0, 1, 3, 5, 6]:
///
/// ```text
/// i=0: citations=0, papers_left=5 → 0 >= 5? no
/// i=1: citations=1, papers_left=4 → 1 >= 4? no
/// i=2: citations=3, papers_left=3 → 3 >= 3? yes! h=3
/// i=3: citations=5, papers_left=2 → 5 >= 2? yes! h=max(3,2)=3
/// i=4: citations=6, papers_left=1 → 6 >= 1? yes! h=max(3,1)=3
/// Answer: 3
/// ```
///
/// # Complexity
///
/// - Time: O(n log n) — sorting
/// - Space: O(n) — sorted copy
pub fn h_index(citations: &[usize]) -> usize {
    let mut sorted = citations.to_vec();
    sorted.sort();

    let n = sorted.len();
    let mut h = 0;

    for i in 0..n {
        let papers_with_at_least = n - i;
        let citation_count = sorted[i];

        if citation_count >= papers_with_at_least {
            h = h.max(papers_with_at_least);
        }
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Binary Search tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_find_insertion_index_found() {
        assert_eq!(find_insertion_index(&[1, 2, 4, 5, 7, 8, 9], 4), 2);
    }

    #[test]
    fn test_find_insertion_index_not_found() {
        assert_eq!(find_insertion_index(&[1, 2, 4, 5, 7, 8, 9], 6), 4);
    }

    #[test]
    fn test_find_insertion_index_insert_at_start() {
        assert_eq!(find_insertion_index(&[2, 4, 6, 8], 1), 0);
    }

    #[test]
    fn test_find_insertion_index_insert_at_end() {
        assert_eq!(find_insertion_index(&[2, 4, 6, 8], 10), 4);
    }

    #[test]
    fn test_find_insertion_index_first_element() {
        assert_eq!(find_insertion_index(&[1, 3, 5, 7], 1), 0);
    }

    #[test]
    fn test_find_insertion_index_last_element() {
        assert_eq!(find_insertion_index(&[1, 3, 5, 7], 7), 3);
    }

    #[test]
    fn test_find_insertion_index_empty_array() {
        assert_eq!(find_insertion_index(&[], 5), 0);
    }

    #[test]
    fn test_find_insertion_index_single_element_found() {
        assert_eq!(find_insertion_index(&[5], 5), 0);
    }

    #[test]
    fn test_find_insertion_index_single_element_insert_before() {
        assert_eq!(find_insertion_index(&[5], 3), 0);
    }

    #[test]
    fn test_find_insertion_index_single_element_insert_after() {
        assert_eq!(find_insertion_index(&[5], 7), 1);
    }

    #[test]
    fn test_find_insertion_index_negative_numbers() {
        assert_eq!(find_insertion_index(&[-10, -5, 0, 5, 10], -3), 2);
    }

    #[test]
    fn test_find_insertion_index_two_elements() {
        assert_eq!(find_insertion_index(&[1, 3], 2), 1);
    }

    #[test]
    fn test_find_first_and_last_example() {
        assert_eq!(
            find_first_and_last(&[1, 2, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 11], 4),
            [3, 5]
        );
    }

    #[test]
    fn test_find_first_and_last_not_found() {
        assert_eq!(find_first_and_last(&[1, 2, 3, 5, 6], 4), [-1, -1]);
    }

    #[test]
    fn test_find_first_and_last_single_occurrence() {
        assert_eq!(find_first_and_last(&[1, 2, 3, 4, 5], 3), [2, 2]);
    }

    #[test]
    fn test_find_first_and_last_all_same() {
        assert_eq!(find_first_and_last(&[4, 4, 4, 4, 4], 4), [0, 4]);
    }

    #[test]
    fn test_find_first_and_last_at_start() {
        assert_eq!(find_first_and_last(&[1, 1, 1, 2, 3, 4], 1), [0, 2]);
    }

    #[test]
    fn test_find_first_and_last_at_end() {
        assert_eq!(find_first_and_last(&[1, 2, 3, 4, 4, 4], 4), [3, 5]);
    }

    #[test]
    fn test_find_first_and_last_empty_array() {
        assert_eq!(find_first_and_last(&[], 4), [-1, -1]);
    }

    #[test]
    fn test_find_first_and_last_single_element_found() {
        assert_eq!(find_first_and_last(&[4], 4), [0, 0]);
    }

    #[test]
    fn test_find_first_and_last_single_element_not_found() {
        assert_eq!(find_first_and_last(&[5], 4), [-1, -1]);
    }

    #[test]
    fn test_find_first_and_last_two_occurrences() {
        assert_eq!(find_first_and_last(&[1, 2, 2, 3], 2), [1, 2]);
    }

    #[test]
    fn test_cutting_wood_example() {
        assert_eq!(cutting_wood(&[2, 6, 3, 8], 7), 3);
    }

    #[test]
    fn test_cutting_wood_exact_cut() {
        assert_eq!(cutting_wood(&[5, 5, 5, 5], 4), 4);
    }

    #[test]
    fn test_cutting_wood_all_same_height() {
        assert_eq!(cutting_wood(&[10, 10, 10], 15), 5);
    }

    #[test]
    fn test_cutting_wood_single_tree() {
        assert_eq!(cutting_wood(&[10], 5), 5);
    }

    #[test]
    fn test_cutting_wood_large_k() {
        // blade=10: 10+5+0+7=22 (not enough), blade=9: 11+6+1+8=26 (enough)
        assert_eq!(cutting_wood(&[20, 15, 10, 17], 24), 9);
    }

    #[test]
    fn test_find_in_rotated_array_example() {
        assert_eq!(find_in_rotated_array(&[8, 9, 1, 2, 3, 4, 5, 6, 7], 1), 2);
    }

    #[test]
    fn test_find_in_rotated_array_not_found() {
        assert_eq!(find_in_rotated_array(&[8, 9, 1, 2, 3, 4, 5, 6, 7], 10), -1);
    }

    #[test]
    fn test_find_in_rotated_array_first_element() {
        assert_eq!(find_in_rotated_array(&[4, 5, 6, 7, 0, 1, 2], 4), 0);
    }

    #[test]
    fn test_find_in_rotated_array_last_element() {
        assert_eq!(find_in_rotated_array(&[4, 5, 6, 7, 0, 1, 2], 2), 6);
    }

    #[test]
    fn test_find_in_rotated_array_pivot_element() {
        assert_eq!(find_in_rotated_array(&[4, 5, 6, 7, 0, 1, 2], 7), 3);
    }

    #[test]
    fn test_find_in_rotated_array_no_rotation() {
        assert_eq!(find_in_rotated_array(&[1, 2, 3, 4, 5], 3), 2);
    }

    #[test]
    fn test_find_in_rotated_array_single_element_found() {
        assert_eq!(find_in_rotated_array(&[5], 5), 0);
    }

    #[test]
    fn test_find_in_rotated_array_single_element_not_found() {
        assert_eq!(find_in_rotated_array(&[5], 3), -1);
    }

    #[test]
    fn test_find_in_rotated_array_two_elements() {
        assert_eq!(find_in_rotated_array(&[2, 1], 1), 1);
    }

    #[test]
    fn test_find_in_rotated_array_target_in_left_half() {
        assert_eq!(find_in_rotated_array(&[6, 7, 8, 1, 2, 3, 4, 5], 8), 2);
    }

    #[test]
    fn test_find_in_rotated_array_target_in_right_half() {
        assert_eq!(find_in_rotated_array(&[6, 7, 8, 1, 2, 3, 4, 5], 3), 5);
    }

    // -------------------------------------------------------------------------
    // Sorting tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_sort_array_example() {
        let mut nums = [6, 8, 4, 2, 7, 3, 1, 5];
        sort_array(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_sort_array_empty() {
        let mut nums: [i32; 0] = [];
        sort_array(&mut nums);
        assert_eq!(nums, []);
    }

    #[test]
    fn test_sort_array_single() {
        let mut nums = [5];
        sort_array(&mut nums);
        assert_eq!(nums, [5]);
    }

    #[test]
    fn test_sort_array_already_sorted() {
        let mut nums = [1, 2, 3, 4, 5];
        sort_array(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_array_reverse() {
        let mut nums = [5, 4, 3, 2, 1];
        sort_array(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_array_duplicates() {
        let mut nums = [3, 1, 2, 1, 3];
        sort_array(&mut nums);
        assert_eq!(nums, [1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_sort_array_negative() {
        let mut nums = [3, -1, 0, -5, 2];
        sort_array(&mut nums);
        assert_eq!(nums, [-5, -1, 0, 2, 3]);
    }

    #[test]
    fn test_merge_sorted_array_example() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let nums2 = vec![2, 5, 6];
        merge_sorted_array(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_merge_sorted_array_nums2_empty() {
        let mut nums1 = vec![1, 2, 3];
        let nums2: Vec<i32> = vec![];
        merge_sorted_array(&mut nums1, 3, &nums2, 0);
        assert_eq!(nums1, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sorted_array_nums1_empty() {
        let mut nums1 = vec![0];
        let nums2 = vec![1];
        merge_sorted_array(&mut nums1, 0, &nums2, 1);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_merge_sorted_array_interleaved() {
        let mut nums1 = vec![1, 3, 5, 0, 0, 0];
        let nums2 = vec![2, 4, 6];
        merge_sorted_array(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sorted_array_nums2_all_smaller() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let nums2 = vec![1, 2, 3];
        merge_sorted_array(&mut nums1, 3, &nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_h_index_example1() {
        assert_eq!(h_index(&[3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn test_h_index_example2() {
        assert_eq!(h_index(&[1, 3, 1]), 1);
    }

    #[test]
    fn test_h_index_all_zeros() {
        assert_eq!(h_index(&[0, 0, 0]), 0);
    }

    #[test]
    fn test_h_index_single_high() {
        assert_eq!(h_index(&[100]), 1);
    }

    #[test]
    fn test_h_index_single_zero() {
        assert_eq!(h_index(&[0]), 0);
    }

    #[test]
    fn test_h_index_all_same() {
        assert_eq!(h_index(&[5, 5, 5, 5, 5]), 5);
    }
}
