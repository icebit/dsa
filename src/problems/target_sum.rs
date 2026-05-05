/// # Target Sum (Dynamic Programming)
///
/// Given integers `nums` and a `target`, count the number of ways to assign '+'
/// or '-' before each number so the expression evaluates to `target`.
///
/// This can be solved as a sign-assignment DP or converted into a subset-sum
/// counting problem.
///
/// **Expected complexity:** O(n * sum(nums)) time

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let _ = (nums, target);
    todo!("implement target sum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_example() {
        assert_eq!(find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn single_matches_positive() {
        assert_eq!(find_target_sum_ways(vec![1], 1), 1);
    }

    #[test]
    fn single_matches_negative() {
        assert_eq!(find_target_sum_ways(vec![1], -1), 1);
    }

    #[test]
    fn zero_doubles_count() {
        assert_eq!(find_target_sum_ways(vec![0, 0, 1], 1), 4);
    }
}
