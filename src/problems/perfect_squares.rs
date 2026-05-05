/// # Perfect Squares (Dynamic Programming)
///
/// Given `n`, return the least number of perfect square numbers whose sum is n.
///
/// Similar to Coin Change: the "coins" are 1, 4, 9, 16, ...
///
/// **Expected complexity:** O(n * sqrt(n)) time, O(n) space

pub fn num_squares(n: i32) -> i32 {
    let _ = n;
    todo!("implement perfect squares")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_square() {
        assert_eq!(num_squares(16), 1);
    }

    #[test]
    fn twelve() {
        assert_eq!(num_squares(12), 3);
    }

    #[test]
    fn thirteen() {
        assert_eq!(num_squares(13), 2);
    }

    #[test]
    fn small() {
        assert_eq!(num_squares(1), 1);
    }
}
