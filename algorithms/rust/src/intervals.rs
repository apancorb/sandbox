/// Merge Overlapping Intervals
///
/// Merge an array of intervals so there are no overlapping intervals.
///
/// # Examples
///
/// ```text
/// Input: intervals = [[3, 4], [7, 8], [2, 5], [6, 7], [1, 4]]
/// Output: [[1, 5], [6, 8]]
/// ```
///
/// Sort intervals by start time, then iterate through. For each interval,
/// if it overlaps with the last merged interval (current start <= previous
/// end), extend the previous end. Otherwise, add it as a new interval.
///
/// Walkthrough for [[3,4], [7,8], [2,5], [6,7], [1,4]]:
///
/// ```text
/// After sorting: [[1,4], [2,5], [3,4], [6,7], [7,8]]
///
/// merged = [[1,4]]
/// [2,5]: 2 <= 4 → overlap, extend → [[1,5]]
/// [3,4]: 3 <= 5 → overlap, max(5,4)=5 → [[1,5]]
/// [6,7]: 6 > 5 → no overlap → [[1,5], [6,7]]
/// [7,8]: 7 <= 7 → overlap, extend → [[1,5], [6,8]]
///
/// Answer: [[1,5], [6,8]]
/// ```
///
/// # Complexity
///
/// - Time: O(n log n) — dominated by sorting
/// - Space: O(n) — for the result
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

/// Largest Overlap of Intervals
///
/// Given an array of intervals, determine the maximum number of intervals
/// that overlap at any point. Each interval is half-open [start, end),
/// meaning it includes start but excludes end.
///
/// # Examples
///
/// ```text
/// Input: intervals = [[1, 3], [2, 6], [4, 8], [6, 7], [5, 7]]
/// Output: 3
/// ```
///
/// Line Sweep Algorithm: convert each interval to two events (START and
/// END), sort by time, then sweep through counting active intervals. At
/// each start event add one, at each end event subtract one. The maximum
/// count seen during the sweep is the answer.
///
/// Walkthrough for [[1,3], [2,6], [4,8]]:
///
/// ```text
/// Events: [(1,'S'), (2,'S'), (3,'E'), (4,'S'), (6,'E'), (8,'E')]
///
/// Time 1, S: current=1  ← [1,3] starts
/// Time 2, S: current=2  ← [2,6] starts (2 active now!)
/// Time 3, E: current=1  ← [1,3] ends
/// Time 4, S: current=2  ← [4,8] starts
/// Time 6, E: current=1  ← [2,6] ends
/// Time 8, E: current=0  ← [4,8] ends
///
/// Max was 2 (at times 2-3 and 4-6)
/// ```
///
/// At same time, 'E' comes before 'S' (half-open intervals).
/// 'E' < 'S' alphabetically, so this works naturally.
///
/// # Complexity
///
/// - Time: O(n log n) — sorting the events
/// - Space: O(n) — storing events
pub fn max_overlapping(intervals: &[[i32; 2]]) -> usize {
    let mut points = vec![];

    for interval in &intervals[..] {
        points.push((interval[0], 'S'));
        points.push((interval[1], 'E'));
    }

    points.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let mut ans = 0;
    let mut curr = 0;

    for point in points {
        if point.1 == 'S' {
            curr += 1;
        } else {
            curr -= 1;
        }
        ans = ans.max(curr);
    }

    ans
}

/// Identify All Interval Overlaps
///
/// Return all overlaps between two arrays of intervals. Each input array
/// is sorted by start value and contains no overlapping intervals within itself.
///
/// # Examples
///
/// ```text
/// Input: intervals1 = [[1, 4], [5, 6], [9, 10]], intervals2 = [[2, 7], [8, 9]]
/// Output: [[2, 4], [5, 6], [9, 9]]
/// ```
///
/// Use two pointers, one for each list. Compare the current intervals: if
/// they overlap, record the intersection as [max(starts), min(ends)]. Then
/// advance whichever pointer's interval ends first, since that interval
/// can't overlap with anything further in the other list.
///
/// Walkthrough for intervals1=[[1,4],[5,6],[9,10]], intervals2=[[2,7],[8,9]]:
///
/// ```text
/// i=0, j=0: [1,4] vs [2,7] → overlap? 1<=7 and 2<=4 → yes
///            intersection = [max(1,2), min(4,7)] = [2,4]
///            4 <= 7 → advance i
/// i=1, j=0: [5,6] vs [2,7] → overlap? 5<=7 and 2<=6 → yes
///            intersection = [max(5,2), min(6,7)] = [5,6]
///            6 <= 7 → advance i
/// i=2, j=0: [9,10] vs [2,7] → overlap? 9<=7? no
///            7 <= 10 → advance j
/// i=2, j=1: [9,10] vs [8,9] → overlap? 9<=9 and 8<=10 → yes
///            intersection = [max(9,8), min(10,9)] = [9,9]
///            9 <= 10 → advance j
/// j=2: out of bounds → done
///
/// Answer: [[2,4], [5,6], [9,9]]
/// ```
///
/// # Complexity
///
/// - Time: O(n + m) — single pass through both lists
/// - Space: O(k) — where k is number of overlaps
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
    fn test_max_overlapping_example() {
        assert_eq!(
            max_overlapping(&[[1, 3], [2, 6], [4, 8], [6, 7], [5, 7]]),
            3
        );
    }

    #[test]
    fn test_max_overlapping_single() {
        assert_eq!(max_overlapping(&[[1, 5]]), 1);
    }

    #[test]
    fn test_max_overlapping_no_overlap() {
        assert_eq!(max_overlapping(&[[1, 2], [3, 4], [5, 6]]), 1);
    }

    #[test]
    fn test_max_overlapping_all_overlap() {
        assert_eq!(max_overlapping(&[[1, 10], [2, 9], [3, 8], [4, 7]]), 4);
    }

    #[test]
    fn test_max_overlapping_touching_half_open() {
        // Half-open: [1, 3) and [3, 5) don't overlap at point 3
        assert_eq!(max_overlapping(&[[1, 3], [3, 5]]), 1);
    }

    #[test]
    fn test_max_overlapping_same_start() {
        assert_eq!(max_overlapping(&[[1, 5], [1, 3], [1, 4]]), 3);
    }

    #[test]
    fn test_max_overlapping_same_end() {
        assert_eq!(max_overlapping(&[[1, 5], [2, 5], [3, 5]]), 3);
    }

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
