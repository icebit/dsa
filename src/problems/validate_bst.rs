use std::cell::RefCell;
use std::rc::Rc;

/// # Validate Binary Search Tree (Tree Recursion)
///
/// Return whether a binary tree is a valid BST. Every node in the left subtree
/// must be strictly less than the current node, and every node in the right
/// subtree must be strictly greater.
///
/// Hint: recurse with lower and upper bounds instead of checking only direct
/// children.
///
/// **Expected complexity:** O(n) time, O(h) recursion stack

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let _ = root;
    todo!("implement validate bst")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    #[test]
    fn empty_is_valid() {
        assert!(is_valid_bst(None));
    }

    #[test]
    fn valid_small_tree() {
        let root = node(2);
        root.borrow_mut().left = Some(node(1));
        root.borrow_mut().right = Some(node(3));
        assert!(is_valid_bst(Some(root)));
    }

    #[test]
    fn invalid_direct_child() {
        let root = node(5);
        root.borrow_mut().left = Some(node(1));
        root.borrow_mut().right = Some(node(4));
        assert!(!is_valid_bst(Some(root)));
    }

    #[test]
    fn invalid_deep_child() {
        let root = node(5);
        let right = node(7);
        right.borrow_mut().left = Some(node(4));
        root.borrow_mut().right = Some(right);
        assert!(!is_valid_bst(Some(root)));
    }
}
