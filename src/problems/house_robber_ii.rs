/// # House Robber II (Dynamic Programming)
///
/// Houses are arranged in a circle, so the first and last houses are adjacent.
/// Return the maximum amount you can rob without robbing adjacent houses.
///
/// Hint: solve the linear House Robber problem twice: once excluding the first
/// house, and once excluding the last house.
///
/// **Expected complexity:** O(n) time, O(1) space

pub fn rob(nums: Vec<i32>) -> i32 {
    let _ = nums;
    todo!("implement house robber ii")
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
        assert_eq!(rob(vec![3]), 3);
    }

    #[test]
    fn cannot_take_first_and_last() {
        assert_eq!(rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn standard() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn larger_middle_choice() {
        assert_eq!(rob(vec![1, 2, 3]), 3);
    }
}
