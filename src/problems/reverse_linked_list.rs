/// # Reverse Linked List (Linked List)
///
/// Given the head of a singly linked list, reverse the list and return the new
/// head.
///
/// **Expected complexity:** O(n) time, O(1) space for the iterative solution

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let _ = head;

    // head > 2 > 3 > 4 > None
    // None < head < 2 < 3 < 4
    // prev < current < next

    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        let next = node.next.take();

        node.next = prev;

        prev = Some(node);
        current = next;
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &value in values.iter().rev() {
            let mut node = Box::new(ListNode::new(value));
            node.next = head;
            head = Some(node);
        }
        head
    }

    #[test]
    fn empty() {
        assert_eq!(reverse_list(None), None);
    }

    #[test]
    fn one_node() {
        assert_eq!(reverse_list(list(&[1])), list(&[1]));
    }

    #[test]
    fn many_nodes() {
        assert_eq!(reverse_list(list(&[1, 2, 3, 4, 5])), list(&[5, 4, 3, 2, 1]));
    }
}
