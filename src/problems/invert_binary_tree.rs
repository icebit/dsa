use std::cell::RefCell;
use std::rc::Rc;

/// # Invert Binary Tree (Tree Recursion)
///
/// Swap every node's left and right child and return the root.
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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let _ = root;
    todo!("implement invert binary tree")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    #[test]
    fn empty_tree() {
        assert_eq!(invert_tree(None), None);
    }

    #[test]
    fn single_node() {
        let root = Some(node(1));
        let inverted = invert_tree(root).unwrap();
        assert_eq!(inverted.borrow().val, 1);
        assert!(inverted.borrow().left.is_none());
        assert!(inverted.borrow().right.is_none());
    }

    #[test]
    fn swaps_children() {
        let root = node(2);
        root.borrow_mut().left = Some(node(1));
        root.borrow_mut().right = Some(node(3));

        let inverted = invert_tree(Some(root)).unwrap();
        assert_eq!(inverted.borrow().left.as_ref().unwrap().borrow().val, 3);
        assert_eq!(inverted.borrow().right.as_ref().unwrap().borrow().val, 1);
    }
}
