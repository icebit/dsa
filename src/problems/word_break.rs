/// # Word Break (Dynamic Programming)
///
/// Given a string `s` and a dictionary of words, return whether `s` can be
/// segmented into a space-separated sequence of dictionary words.
///
/// DP idea: `dp[i]` means `s[..i]` can be segmented. For each reachable i, test
/// every dictionary word or every previous split point.
///
/// **Expected complexity:** O(n^2) time, O(n) space

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let _ = (s, word_dict);
    todo!("implement word break")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(items: &[&str]) -> Vec<String> {
        items.iter().map(|word| word.to_string()).collect()
    }

    #[test]
    fn can_segment() {
        assert!(word_break("leetcode".to_string(), words(&["leet", "code"])));
    }

    #[test]
    fn reuses_words() {
        assert!(word_break("applepenapple".to_string(), words(&["apple", "pen"])));
    }

    #[test]
    fn cannot_segment() {
        assert!(!word_break(
            "catsandog".to_string(),
            words(&["cats", "dog", "sand", "and", "cat"])
        ));
    }

    #[test]
    fn empty_string() {
        assert!(word_break("".to_string(), vec![]));
    }
}
