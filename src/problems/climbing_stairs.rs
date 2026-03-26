use std::collections::HashMap;

/// # Climbing Stairs (Dynamic Programming)
///
/// You are climbing a staircase with `n` steps. Each time you can climb 1 or 2 steps.
/// Return the number of distinct ways to reach the top.
///
/// This is a classic DP problem. The recurrence is f(n) = f(n-1) + f(n-2) because
/// you can arrive at step n from either step n-1 (one step) or step n-2 (two steps).
/// A naive recursive solution is O(2^n); with memoization or bottom-up DP it becomes O(n).
///
/// **Expected complexity:** O(n) time, O(1) space (bottom-up with two variables)
///
/// ## Examples
/// - n = 2 -> 2  (1+1 or 2)
/// - n = 3 -> 3  (1+1+1, 1+2, 2+1)

pub fn climb_stairs(n: i32) -> i32 {
    let mut map : HashMap<i32, i32> = HashMap::new();
    return climb_stairs_dp(n, &mut map);
}

fn climb_stairs_dp(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&precalculated) = map.get(&n) {
        return precalculated;
    }

    if n <= 2 {
        return n;
    }

    let result = climb_stairs_dp(n - 1, map) + climb_stairs_dp(n - 2, map);
    map.insert(n, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_step() {
        assert_eq!(climb_stairs(1), 1);
    }

    #[test]
    fn two_steps() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn three_steps() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn five_steps() {
        assert_eq!(climb_stairs(5), 8);
    }

    #[test]
    fn ten_steps() {
        assert_eq!(climb_stairs(10), 89);
    }

    #[test]
    fn larger() {
        // Fibonacci-like: f(20) = 10946
        assert_eq!(climb_stairs(20), 10946);
    }
}
