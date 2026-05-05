/// # Partition Equal Subset Sum (0/1 Knapsack)
///
/// Given positive integers, return whether the array can be partitioned into two
/// subsets with equal sum.
///
/// If the total sum is even, this becomes: can any subset sum to `total / 2`?
///
/// **Expected complexity:** O(n * target) time, O(target) space

pub fn can_partition(nums: Vec<i32>) -> bool {
    let _ = nums;
    todo!("implement partition equal subset sum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_partition_example() {
        assert!(can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn odd_total() {
        assert!(!can_partition(vec![1, 2, 3, 5]));
    }

    #[test]
    fn two_equal_numbers() {
        assert!(can_partition(vec![2, 2]));
    }

    #[test]
    fn single_number() {
        assert!(!can_partition(vec![7]));
    }
}
