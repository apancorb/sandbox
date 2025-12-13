/// Merge Overlapping Intervals
///
/// Merge an array of intervals so there are no overlapping intervals, and return the resultant
/// merged intervals.
///
/// # Example
///
/// ```text
/// Input: intervals = [[3, 4], [7, 8], [2, 5], [6, 7], [1, 4]]
/// Output: [[1, 5], [6, 8]]
/// ```
///
/// # Constraints
///
/// - The input contains at least one interval.
/// - For every index i in the array, intervals[i].start <= intervals[i].end.
///
// So for ascending sort (a.cmp(b)):
// - 4.cmp(-5) = Greater → 4 goes after -5 ✓
//
// For descending sort (b.cmp(a)):
// - -5.cmp(4) = Less → 4 goes before -5 ✓
pub fn merge_intervals(intervals: &mut [[i32; 2]]) -> Vec<[i32; 2]> {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut merged = vec![intervals[0]];

    for interval in &intervals[1..] {
        let curr = merged.last_mut().unwrap();
        if interval[0] <= curr[1] {
            curr[1] = interval[1].max(curr[1]);
        } else {
            merged.push(*interval);
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_example() {
        assert_eq!(
            merge_intervals(&mut [[3, 4], [7, 8], [2, 5], [6, 7], [1, 4]]),
            vec![[1, 5], [6, 8]]
        );
    }

    #[test]
    fn test_merge_intervals_no_overlap() {
        assert_eq!(
            merge_intervals(&mut [[1, 2], [4, 5], [7, 8]]),
            vec![[1, 2], [4, 5], [7, 8]]
        );
    }

    #[test]
    fn test_merge_intervals_all_overlap() {
        assert_eq!(merge_intervals(&mut [[1, 5], [2, 6], [3, 7]]), vec![[1, 7]]);
    }

    #[test]
    fn test_merge_intervals_single() {
        assert_eq!(merge_intervals(&mut [[1, 5]]), vec![[1, 5]]);
    }

    #[test]
    fn test_merge_intervals_touching() {
        // [1, 3] and [3, 5] touch at 3, should merge
        assert_eq!(merge_intervals(&mut [[1, 3], [3, 5]]), vec![[1, 5]]);
    }

    #[test]
    fn test_merge_intervals_contained() {
        // [2, 4] is fully contained in [1, 5]
        assert_eq!(merge_intervals(&mut [[1, 5], [2, 4]]), vec![[1, 5]]);
    }

    #[test]
    fn test_merge_intervals_unsorted() {
        assert_eq!(
            merge_intervals(&mut [[5, 6], [1, 2], [3, 4]]),
            vec![[1, 2], [3, 4], [5, 6]]
        );
    }

    #[test]
    fn test_merge_intervals_negative() {
        assert_eq!(
            merge_intervals(&mut [[-5, -2], [-3, 0], [1, 3]]),
            vec![[-5, 0], [1, 3]]
        );
    }
}
