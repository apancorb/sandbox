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

/// Identify All Interval Overlaps
///
/// Return an array of all overlaps between two arrays of intervals: intervals1 and intervals2.
/// Each individual interval array is sorted by start value, and contains no overlapping
/// intervals within itself.
///
/// # Example
///
/// ```text
/// Input: intervals1 = [[1, 4], [5, 6], [9, 10]], intervals2 = [[2, 7], [8, 9]]
/// Output: [[2, 4], [5, 6], [9, 9]]
/// ```
///
/// # Constraints
///
/// - For every index i in intervals1, intervals1[i].start <= intervals1[i].end.
/// - For every index j in intervals2, intervals2[j].start <= intervals2[j].end.
pub fn find_overlaps(intervals1: &[[i32; 2]], intervals2: &[[i32; 2]]) -> Vec<[i32; 2]> {
    let mut i = 0;
    let mut j = 0;
    let mut res = Vec::new();

    while i < intervals1.len() && j < intervals2.len() {
        let a;
        let b;
        if intervals1[i][0] <= intervals2[j][0] {
            a = intervals1[i];
            b = intervals2[j];
        } else {
            a = intervals2[j];
            b = intervals1[i];
        }

        if a[1] >= b[0] {
            res.push([b[0], a[1].min(b[1])]);
        }

        if intervals1[i][1] <= intervals2[j][1] {
            i += 1;
        } else {
            j += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_overlaps_example() {
        assert_eq!(
            find_overlaps(&[[1, 4], [5, 6], [9, 10]], &[[2, 7], [8, 9]]),
            vec![[2, 4], [5, 6], [9, 9]]
        );
    }

    #[test]
    fn test_find_overlaps_no_overlap() {
        assert_eq!(
            find_overlaps(&[[1, 2], [5, 6]], &[[3, 4], [7, 8]]),
            Vec::<[i32; 2]>::new()
        );
    }

    #[test]
    fn test_find_overlaps_full_overlap() {
        assert_eq!(
            find_overlaps(&[[1, 10]], &[[2, 4], [6, 8]]),
            vec![[2, 4], [6, 8]]
        );
    }

    #[test]
    fn test_find_overlaps_single_point() {
        assert_eq!(find_overlaps(&[[1, 3]], &[[3, 5]]), vec![[3, 3]]);
    }

    #[test]
    fn test_find_overlaps_empty_first() {
        assert_eq!(find_overlaps(&[], &[[1, 2]]), Vec::<[i32; 2]>::new());
    }

    #[test]
    fn test_find_overlaps_empty_second() {
        assert_eq!(find_overlaps(&[[1, 2]], &[]), Vec::<[i32; 2]>::new());
    }

    #[test]
    fn test_find_overlaps_multiple() {
        assert_eq!(
            find_overlaps(&[[0, 2], [5, 10], [13, 23]], &[[1, 5], [8, 12], [15, 20]]),
            vec![[1, 2], [5, 5], [8, 10], [15, 20]]
        );
    }

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
