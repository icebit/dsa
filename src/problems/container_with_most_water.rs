/// # Container With Most Water
///
/// Given `n` non-negative integers `height` where each represents a vertical
/// line at position `i`, find two lines that together with the x-axis form a
/// container that holds the most water.
///
/// Return the maximum amount of water the container can store.
/// The container cannot be slanted.
///
/// **Expected complexity:** O(n) time, O(1) space
///
/// ## Examples
/// - height = [1,8,6,2,5,4,8,3,7] → 49
/// - height = [1,1] → 1

use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {

    // moving the pointer at the lowest height can only help

    let mut max = 0;
    let mut a : i32 = 0;
    let mut b : i32 = height.len() as i32 - 1;

    while a < b {
        max = cmp::max(max, cmp::min(height[a as usize], height[b as usize]) * (b - a));

        if height[a as usize] < height[b as usize] {
            a += 1;
        } else {
            b -= 1;
        }
    }

    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn two_elements() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }

    #[test]
    fn ascending() {
        assert_eq!(max_area(vec![1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn descending() {
        assert_eq!(max_area(vec![5, 4, 3, 2, 1]), 6);
    }

    #[test]
    fn tall_ends() {
        assert_eq!(max_area(vec![100, 1, 1, 1, 100]), 400);
    }

    #[test]
    fn single_tall() {
        assert_eq!(max_area(vec![1, 1, 1, 1000, 1, 1, 1]), 6);
    }

    #[test]
    fn uniform() {
        assert_eq!(max_area(vec![5, 5, 5, 5, 5]), 20);
    }
}
