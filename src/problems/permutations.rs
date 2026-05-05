/// # Permutations (Backtracking)
///
/// Given an array of distinct integers, return all possible permutations.
///
/// Build a path one choice at a time. Track which indexes are already used, or
/// swap elements in-place during recursion.
///
/// **Expected complexity:** O(n * n!) time, O(n) recursion space excluding output

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let _ = nums;
    todo!("implement permutations")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        items.sort();
        items
    }

    #[test]
    fn three_numbers() {
        let result = sorted(permute(vec![1, 2, 3]));
        let expected = sorted(vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn one_number() {
        assert_eq!(permute(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn empty() {
        assert_eq!(permute(vec![]), vec![vec![] as Vec<i32>]);
    }
}
