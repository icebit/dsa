/// # Decode Ways (Dynamic Programming)
///
/// A message containing digits maps to letters where 'A' -> "1" through
/// 'Z' -> "26". Return the number of ways to decode the whole string.
///
/// Watch the zero cases carefully: "0" is invalid by itself, but "10" and "20"
/// are valid two-digit decodings.
///
/// **Expected complexity:** O(n) time, O(1) space

pub fn num_decodings(s: String) -> i32 {
    let _ = s;
    todo!("implement decode ways")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_branch() {
        assert_eq!(num_decodings("12".to_string()), 2);
    }

    #[test]
    fn multiple_branches() {
        assert_eq!(num_decodings("226".to_string()), 3);
    }

    #[test]
    fn leading_zero_invalid() {
        assert_eq!(num_decodings("06".to_string()), 0);
    }

    #[test]
    fn valid_zero_pair() {
        assert_eq!(num_decodings("10".to_string()), 1);
    }
}
