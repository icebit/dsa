/// # Minimum Path Sum (Grid Dynamic Programming)
///
/// Given a grid of non-negative numbers, find a path from top-left to
/// bottom-right with the minimum sum. You may only move right or down.
///
/// **Expected complexity:** O(m * n) time, O(1) extra space if mutating the grid

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let _ = grid;
    todo!("implement minimum path sum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_example() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(min_path_sum(grid), 7);
    }

    #[test]
    fn single_row() {
        assert_eq!(min_path_sum(vec![vec![1, 2, 3]]), 6);
    }

    #[test]
    fn single_column() {
        assert_eq!(min_path_sum(vec![vec![1], vec![2], vec![3]]), 6);
    }

    #[test]
    fn one_cell() {
        assert_eq!(min_path_sum(vec![vec![5]]), 5);
    }
}
