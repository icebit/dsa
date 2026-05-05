/// # Unique Paths (Grid Dynamic Programming)
///
/// A robot starts at the top-left of an `m x n` grid and can move only right or
/// down. Return the number of unique paths to the bottom-right.
///
/// Recurrence: `paths[row][col] = paths[row - 1][col] + paths[row][col - 1]`.
///
/// **Expected complexity:** O(m * n) time, O(n) space

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let _ = (m, n);
    todo!("implement unique paths")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_by_one() {
        assert_eq!(unique_paths(1, 1), 1);
    }

    #[test]
    fn three_by_seven() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn three_by_two() {
        assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn square_grid() {
        assert_eq!(unique_paths(3, 3), 6);
    }
}
