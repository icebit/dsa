/// # Implement Trie (Prefix Tree)
///
/// Implement a trie with insert, full-word search, and prefix search.
///
/// The default LeetCode API uses:
/// - `Trie::new()`
/// - `insert(word)`
/// - `search(word)`
/// - `starts_with(prefix)`
///
/// **Expected complexity:** O(length) per operation

pub struct Trie {
    _private: (),
}

impl Trie {
    pub fn new() -> Self {
        todo!("implement trie constructor")
    }

    pub fn insert(&mut self, word: String) {
        let _ = word;
        todo!("implement trie insert")
    }

    pub fn search(&self, word: String) -> bool {
        let _ = word;
        todo!("implement trie search")
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let _ = prefix;
        todo!("implement trie starts_with")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_example() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());

        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));

        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }

    #[test]
    fn independent_words() {
        let mut trie = Trie::new();
        trie.insert("car".to_string());
        trie.insert("cat".to_string());

        assert!(trie.search("car".to_string()));
        assert!(trie.search("cat".to_string()));
        assert!(!trie.search("cap".to_string()));
        assert!(trie.starts_with("ca".to_string()));
    }
}
