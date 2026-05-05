/// # Longest Increasing Subsequence (Dynamic Programming)
///
/// Given an integer array `nums`, return the length of the longest **strictly
/// increasing** subsequence. A subsequence is derived by deleting some (possibly
/// zero) elements without changing the order of the remaining ones.
///
/// This problem introduces the classic "best result *ending at* index i" DP pattern:
///   dp[i] = 1 + max(dp[j] for all j < i where nums[j] < nums[i])
/// The answer is max(dp).
///
/// **Expected complexity:** O(n^2) time, O(n) space for the standard DP.
/// **Stretch goal:** O(n log n) using patience sorting + binary search — maintain
/// a `tails` array where `tails[k]` is the smallest possible tail of an increasing
/// subsequence of length k+1, and binary-search the insertion point for each value.
///
/// ## Examples
/// - [10, 9, 2, 5, 3, 7, 101, 18] -> 4   (one LIS: [2, 3, 7, 101])
/// - [0, 1, 0, 3, 2, 3]           -> 4   (e.g. [0, 1, 2, 3])
/// - [7, 7, 7, 7, 7, 7, 7]        -> 1
/// - []                           -> 0

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let _ = nums;
    todo!("implement longest increasing subsequence")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(length_of_lis(vec![]), 0);
    }

    #[test]
    fn single() {
        assert_eq!(length_of_lis(vec![42]), 1);
    }

    #[test]
    fn all_equal() {
        assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn already_sorted() {
        assert_eq!(length_of_lis(vec![1, 2, 3, 4, 5, 6]), 6);
    }

    #[test]
    fn reverse_sorted() {
        assert_eq!(length_of_lis(vec![6, 5, 4, 3, 2, 1]), 1);
    }

    #[test]
    fn classic_example() {
        // LIS: [2, 3, 7, 101] or [2, 3, 7, 18]
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn duplicates_in_run() {
        // Strictly increasing: duplicates do not count.
        // LIS: [0, 1, 2, 3]
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn negatives() {
        // LIS: [-5, -3, 0, 6, 9] (length 5)
        assert_eq!(length_of_lis(vec![-5, 10, -3, 0, 6, -1, 9]), 5);
    }

    #[test]
    fn interleaved() {
        // LIS: [1, 3, 6, 7, 9, 10] -> length 6
        assert_eq!(length_of_lis(vec![1, 3, 2, 6, 4, 7, 5, 9, 8, 10]), 6);
    }

    #[test]
    fn larger() {
        // LIS: [1, 2, 3, 4, 5, 6, 7, 8, 9] -> length 9
        let nums = vec![
            10, 22, 9, 33, 21, 50, 41, 60, 80, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];
        assert_eq!(length_of_lis(nums), 9);
    }
}
