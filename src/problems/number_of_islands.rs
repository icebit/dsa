/// # Number of Islands (Graph Traversal)
///
/// Given a 2D grid of '1' land and '0' water, count the number of islands.
/// An island is connected horizontally or vertically.
///
/// Use DFS or BFS from each unvisited land cell.
///
/// **Expected complexity:** O(rows * cols) time, O(rows * cols) worst-case space

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let _ = grid;
    todo!("implement number of islands")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_island() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn three_islands() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn all_water() {
        assert_eq!(num_islands(vec![vec!['0', '0'], vec!['0', '0']]), 0);
    }
}
