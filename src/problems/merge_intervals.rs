/// # Merge Intervals (Sorting)
///
/// Given a collection of intervals where each interval is a pair `[start, end]`,
/// merge all overlapping intervals and return the result.
///
/// Two intervals overlap if one starts before or when the other ends.
///
/// **Expected complexity:** O(n log n) time, O(n) space
///
/// ## Examples
/// - [[1,3],[2,6],[8,10],[15,18]] -> [[1,6],[8,10],[15,18]]
/// - [[1,4],[4,5]]               -> [[1,5]]
/// - [[1,4],[0,4]]               -> [[0,4]]

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    // [1,3], [2,6], [3, 6]
    // 0 x x x 0 0 0
    // 0 0 x x x x x
    // 0 0 0 x x x x
    //
    // [1, 6], [3, 6]
    // naive: for each interval, find all other intervals in range. if another interval overlaps, merge it into the current one and delete it.
    //
    //

    // implement the naive approach
    let mut intervals = intervals;
    intervals.sort_by_key(|interval| interval[0]);
    let mut merged: Vec<Vec<i32>> = Vec::new();
    for interval in intervals {
        if let Some(last) = merged.last_mut() {
            if last[1] >= interval[0] {
                last[1] = last[1].max(interval[1]);
            } else {
                merged.push(interval);
            }
        } else {
            merged.push(interval);
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_merge() {
        let input = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(merge(input), vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn touching_intervals() {
        let input = vec![vec![1, 4], vec![4, 5]];
        assert_eq!(merge(input), vec![vec![1, 5]]);
    }

    #[test]
    fn single_interval() {
        assert_eq!(merge(vec![vec![1, 1]]), vec![vec![1, 1]]);
    }

    #[test]
    fn already_merged() {
        let input = vec![vec![1, 2], vec![5, 6], vec![9, 10]];
        assert_eq!(merge(input), vec![vec![1, 2], vec![5, 6], vec![9, 10]]);
    }

    #[test]
    fn all_overlap() {
        let input = vec![vec![1, 10], vec![2, 6], vec![3, 5], vec![7, 9]];
        assert_eq!(merge(input), vec![vec![1, 10]]);
    }

    #[test]
    fn unsorted_input() {
        let input = vec![vec![3, 4], vec![1, 2], vec![2, 3]];
        assert_eq!(merge(input), vec![vec![1, 4]]);
    }

    #[test]
    fn nested_intervals() {
        let input = vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7]];
        assert_eq!(merge(input), vec![vec![1, 10]]);
    }
}
