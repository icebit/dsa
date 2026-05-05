/// # Course Schedule (Graph / Topological Sort)
///
/// There are `num_courses` courses labeled from 0 to `num_courses - 1`.
/// Each prerequisite pair `[a, b]` means you must take b before a.
/// Return whether it is possible to finish all courses.
///
/// Solve by detecting a cycle with DFS colors, or by Kahn's algorithm using
/// indegrees.
///
/// **Expected complexity:** O(courses + prerequisites) time

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let _ = (num_courses, prerequisites);
    todo!("implement course schedule")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_possible() {
        assert!(can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn simple_cycle() {
        assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }

    #[test]
    fn disconnected_courses() {
        assert!(can_finish(4, vec![vec![1, 0], vec![2, 1]]));
    }

    #[test]
    fn longer_cycle() {
        assert!(!can_finish(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]]));
    }
}
