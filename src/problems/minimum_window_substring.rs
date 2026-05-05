/// # Minimum Window Substring (Sliding Window)
///
/// Given strings `s` and `t`, return the minimum window substring of `s` that
/// contains every character in `t`, including duplicate counts. Return an empty
/// string if no such window exists.
///
/// Use a growing/shrinking window with character counts.
///
/// **Expected complexity:** O(s.len() + t.len()) time

pub fn min_window(s: String, t: String) -> String {
    let _ = (s, t);
    todo!("implement minimum window substring")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_example() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC");
    }

    #[test]
    fn exact_match() {
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a");
    }

    #[test]
    fn impossible() {
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "");
    }

    #[test]
    fn duplicate_requirement() {
        assert_eq!(min_window("aaab".to_string(), "aab".to_string()), "aab");
    }
}
