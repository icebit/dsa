/// # Two Sum
///
/// Given an array of integers `nums` and an integer `target`, return the indices
/// of the two numbers that add up to `target`.
///
/// - Each input has exactly one solution.
/// - You may not use the same element twice.
/// - Return the indices in any order.
///
/// **Expected complexity:** O(n) time, O(n) space
///
/// ## Examples
/// - nums = [2, 7, 11, 15], target = 9 → [0, 1]
/// - nums = [3, 2, 4], target = 6 → [1, 2]
/// - nums = [3, 3], target = 6 → [0, 1]

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut map : HashMap<i32, usize> = HashMap::new();

    for (index, value) in nums.into_iter().enumerate() {
        if let Some(&i) = map.get(&(target - value)) {
            return vec![i, index];
        }
        map.insert(value, index);
    }

    return vec!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(nums: Vec<i32>, target: i32, expected: &mut Vec<usize>) {
        let mut result = two_sum(nums, target);
        result.sort();
        expected.sort();
        assert_eq!(result, *expected);
    }

    #[test]
    fn basic() {
        check(vec![2, 7, 11, 15], 9, &mut vec![0, 1]);
    }

    #[test]
    fn middle_pair() {
        check(vec![3, 2, 4], 6, &mut vec![1, 2]);
    }

    #[test]
    fn duplicates() {
        check(vec![3, 3], 6, &mut vec![0, 1]);
    }

    #[test]
    fn negatives() {
        check(vec![-1, -2, -3, -4, -5], -8, &mut vec![2, 4]);
    }

    #[test]
    fn large_values() {
        check(vec![1_000_000, 500_000, -500_000, 0], 0, &mut vec![1, 2]);
    }

    #[test]
    fn last_two() {
        check(vec![1, 2, 3, 4, 5, 6], 11, &mut vec![4, 5]);
    }
}
