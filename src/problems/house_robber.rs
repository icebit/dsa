/// # House Robber (Dynamic Programming)
///
/// Given a row of houses where `nums[i]` is the money in house i, return the
/// maximum amount you can rob without robbing two adjacent houses.
///
/// Recurrence:
/// `best[i] = max(best[i - 1], best[i - 2] + nums[i])`
///
/// **Expected complexity:** O(n) time, O(1) space
///
/// ## Examples
/// - [1, 2, 3, 1] -> 4
/// - [2, 7, 9, 3, 1] -> 12

pub fn rob(nums: Vec<i32>) -> i32 {
    let _ = nums;

    let n = nums.len();
    let mut sol: Vec<i32> = vec![-1; n];
    rob_dp(n - 1, &nums, &mut sol)
}

pub fn rob_dp(i: usize, nums: &Vec<i32>, sol: &mut Vec<i32>) -> i32 {
    if i == 0 {
        return nums[i];
    }

    return std::cmp::max(rob_dp(i - 1, nums, sol), rob_dp(i - 2, nums, sol) + nums[i]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(rob(vec![]), 0);
    }

    #[test]
    fn one_house() {
        assert_eq!(rob(vec![5]), 5);
    }

    #[test]
    fn classic_example() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn skip_adjacent() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn choose_later_houses() {
        assert_eq!(rob(vec![2, 1, 1, 2]), 4);
    }
}
