use std::cmp;
/// # Longest Substring Without Repeating Characters
///
/// Given a string `s`, find the length of the longest substring
/// without repeating characters.
///
/// **Expected complexity:** O(n) time, O(min(n, alphabet)) space
///
/// ## Examples
/// - "abcabcbb" → 3 ("abc")
/// - "bbbbb" → 1 ("b")
/// - "pwwkew" → 3 ("wke")
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut map: HashMap<char, usize> = HashMap::new();

    let mut l: usize = 0;
    let mut max: usize = 0;

    for r in 0..chars.len() {
        if let Some(&prev) = map.get(&chars[r]) {
            l = cmp::max(l, prev + 1);
        }
        map.insert(chars[r], r);
        max = cmp::max(max, r - l + 1);
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_abc() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
    }

    #[test]
    fn all_same() {
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
    }

    #[test]
    fn middle_window() {
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
    }

    #[test]
    fn empty() {
        assert_eq!(length_of_longest_substring("".into()), 0);
    }

    #[test]
    fn single_char() {
        assert_eq!(length_of_longest_substring("a".into()), 1);
    }

    #[test]
    fn all_unique() {
        assert_eq!(length_of_longest_substring("abcdef".into()), 6);
    }

    #[test]
    fn repeat_at_end() {
        assert_eq!(length_of_longest_substring("abcda".into()), 4);
    }

    #[test]
    fn spaces_and_symbols() {
        assert_eq!(length_of_longest_substring("a b c a".into()), 3);
    }
}
