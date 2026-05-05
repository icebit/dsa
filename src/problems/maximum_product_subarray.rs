/// # Maximum Product Subarray (Dynamic Programming)
///
/// Given an integer array, return the largest product of any non-empty
/// contiguous subarray.
///
/// Track both the maximum and minimum product ending at each index because a
/// negative number can turn the smallest product into the largest.
///
/// **Expected complexity:** O(n) time, O(1) space

pub fn max_product(nums: Vec<i32>) -> i32 {
    let _ = nums;
    todo!("implement maximum product subarray")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_run() {
        assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn zero_breaks_subarray() {
        assert_eq!(max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn two_negatives() {
        assert_eq!(max_product(vec![-2, 3, -4]), 24);
    }

    #[test]
    fn single_negative() {
        assert_eq!(max_product(vec![-2]), -2);
    }
}
