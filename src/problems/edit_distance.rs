/// # Edit Distance (2D Dynamic Programming)
///
/// Given two strings, return the minimum number of insertions, deletions, and
/// replacements required to convert `word1` into `word2`.
///
/// Each cell represents the best answer for prefixes `word1[..i]` and
/// `word2[..j]`.
///
/// **Expected complexity:** O(m * n) time, O(m * n) space

pub fn min_distance(word1: String, word2: String) -> i32 {
    let _ = (word1, word2);
    todo!("implement edit distance")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horse_to_ros() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn intention_to_execution() {
        assert_eq!(
            min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }

    #[test]
    fn same_word() {
        assert_eq!(min_distance("same".to_string(), "same".to_string()), 0);
    }

    #[test]
    fn empty_to_word() {
        assert_eq!(min_distance("".to_string(), "abc".to_string()), 3);
    }
}
