/// # Subsets (Backtracking)
///
/// Given an integer array `nums` of unique elements, return all possible subsets (the power set).
/// The solution set must not contain duplicate subsets. Return them in any order.
///
/// This is a foundational backtracking problem. At each index you make a binary choice:
/// include the current element or skip it, then recurse on the remainder.
///
/// **Expected complexity:** O(n * 2^n) time, O(n) space (excluding output)
///
/// ## Examples
/// - nums = [1, 2, 3] -> [[], [1], [2], [3], [1,2], [1,3], [2,3], [1,2,3]]
/// - nums = [0] -> [[], [0]]

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    backtrack(0, Vec::new(), &nums, &mut results);
    results
}

fn backtrack(index: i32, current: Vec<i32>, nums: &Vec<i32>, results: &mut Vec<Vec<i32>>) {
    if index == nums.len() as i32 {
        results.push(current);
        return;
    }

    let mut next = current.clone();
    next.push(nums[index as usize]);
    backtrack(index + 1, current, nums, results);
    backtrack(index + 1, next, nums, results);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut vv: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for v in vv.iter_mut() {
            v.sort();
        }
        vv.sort();
        vv
    }

    #[test]
    fn three_elements() {
        let result = sorted(subsets(vec![1, 2, 3]));
        let expected = sorted(vec![
            vec![], vec![1], vec![2], vec![3],
            vec![1, 2], vec![1, 3], vec![2, 3],
            vec![1, 2, 3],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn single_element() {
        let result = sorted(subsets(vec![0]));
        let expected = sorted(vec![vec![], vec![0]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn empty() {
        let result = sorted(subsets(vec![]));
        assert_eq!(result, vec![vec![] as Vec<i32>]);
    }

    #[test]
    fn two_elements() {
        let result = sorted(subsets(vec![5, 10]));
        let expected = sorted(vec![vec![], vec![5], vec![10], vec![5, 10]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn correct_count() {
        // Power set of n elements has 2^n subsets
        let result = subsets(vec![1, 2, 3, 4]);
        assert_eq!(result.len(), 16);
    }
}
