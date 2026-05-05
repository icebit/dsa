/// # Longest Common Subsequence (2D Dynamic Programming)
///
/// Given two strings, return the length of their longest common subsequence.
/// A subsequence keeps relative order but does not need to be contiguous.
///
/// If the current characters match, extend the diagonal answer. Otherwise, take
/// the best of skipping one character from either string.
///
/// **Expected complexity:** O(m * n) time, O(m * n) space
/// **Stretch goal:** optimize to O(min(m, n)) space

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let _ = (text1, text2);
    todo!("implement longest common subsequence")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_example() {
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn identical() {
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn no_common_chars() {
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }

    #[test]
    fn repeated_chars() {
        assert_eq!(
            longest_common_subsequence("bsbininm".to_string(), "jmjkbkjkv".to_string()),
            1
        );
    }
}
