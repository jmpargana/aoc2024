use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("puzzle.txt")?;

    let buf = BufReader::new(f);

    let mut levels = Vec::new();

    for line in buf.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|a| a.parse::<i32>().unwrap())
            .collect();
        levels.push(numbers);
    }

    let safe_levels = total_safe_levels(&levels);
    let safe_tolerated_levels = total_safe_tolerated_levels(&levels);

    println!("result 1 {}", safe_levels);
    println!("result 2 {}", safe_tolerated_levels);

    Ok(())
}

fn is_unsafe_jump(a: i32) -> bool {
    a.abs() > 3 || a.abs() < 1
}

fn is_unsafe_change(a: i32, b: i32) -> bool {
    (b < 0 && a > 0) || (b > 0 && a < 0) || a == 0 || b == 0
}

fn is_safe(arr: &[i32]) -> bool {
    let mut prev_diff = None;
    for i in 1..arr.len() {
        let diff = arr[i] - arr[i - 1];

        if is_unsafe_jump(diff) {
            return false;
        }

        if let Some(a) = prev_diff {
            if is_unsafe_change(diff, a) {
                return false;
            }
        }

        prev_diff = Some(diff);
    }
    true
}

fn is_safe_tolerate_1(arr: &[i32]) -> bool {
    if is_safe(arr) {
        return true; // Already safe
    }

    for i in 0..arr.len() {
        let mut modified = Vec::from(arr);
        modified.remove(i); // Remove one level
        if is_safe(&modified) {
            return true; // Safe after tolerance
        }
    }

    false // Unsafe even with tolerance
}

fn total_safe_levels(arr: &Vec<Vec<i32>>) -> i32 {
    arr.iter().filter(|a| is_safe(a)).count() as i32
}

fn total_safe_tolerated_levels(arr: &Vec<Vec<i32>>) -> i32 {
    arr.iter().filter(|a| is_safe_tolerate_1(a)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_safe() {
        let given = vec![7, 6, 4, 2, 1];
        let actual = is_safe(&given);
        assert!(actual);
    }

    #[test]
    fn should_not_be_safe() {
        let given = vec![1, 2, 7, 8, 9];
        let actual = is_safe(&given);
        assert!(!actual);
    }

    #[test]
    fn should_not_be_safe_inc_dec() {
        let given = vec![1, 3, 2, 4, 5];
        let actual = is_safe(&given);
        assert!(!actual);
    }

    #[test]
    fn should_be_safe_inc_dec_tolerated() {
        let given = vec![1, 3, 2, 4, 5];
        let actual = is_safe_tolerate_1(&given);
        assert!(actual);
    }

    #[test]
    fn should_be_safe_inc_dec_tolerated2() {
        let given = vec![8, 6, 4, 4, 1];
        let actual = is_safe_tolerate_1(&given);
        assert!(actual);
    }

    #[test]
    fn should_be_unsafe_inc_dec_tolerated() {
        let given = vec![1, 2, 7, 8, 9];
        let actual = is_safe_tolerate_1(&given);
        assert!(!actual);
    }

    #[test]
    fn should_count_safe_levels() {
        let given = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 3, 6, 7, 9],
            vec![1, 2, 7, 8, 9],
        ];
        let actual = total_safe_levels(&given);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_count_safe_tolerated_levels() {
        let given = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let actual = total_safe_tolerated_levels(&given);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_tolerate_first_elem_arr() {
        let given = vec![7, 1, 2, 3, 4];
        let actual = is_safe_tolerate_1(&given);
        assert!(actual);
    }

    #[test]
    fn should_tolerate_last_elem_arr() {
        let given = vec![1, 2, 3, 4, 90];
        let actual = is_safe_tolerate_1(&given);
        assert!(actual);
    }

    #[test]
    fn should_tolerate_one_zero() {
        let given = vec![1, 1, 2, 3, 4];
        let actual = is_safe_tolerate_1(&given);
        assert!(actual);
    }

    #[test]
    fn should_not_tolerate_two_zero() {
        let given = vec![1, 1, 1, 2, 3];
        let actual = is_safe_tolerate_1(&given);
        assert!(!actual);
    }

    #[test]
    fn should_not_tolerate_two_jumps() {
        let given = vec![1, 5, 9, 10, 11];
        let actual = is_safe_tolerate_1(&given);
        assert!(!actual);
    }

    #[test]
    fn single_element_is_safe() {
        let given = vec![7];
        assert!(is_safe(&given));
    }

    #[test]
    fn two_identical_elements_are_unsafe() {
        let given = vec![4, 4];
        assert!(!is_safe(&given));
    }

    #[test]
    fn two_identical_elements_are_safe_when_tolerated() {
        let given = vec![4, 4];
        assert!(is_safe_tolerate_1(&given));
    }

    #[test]
    fn tolerate_large_jump_at_start() {
        let given = vec![1, 5, 6, 7, 8];
        assert!(is_safe_tolerate_1(&given));
    }

    #[test]
    fn tolerate_fluctuation_by_removing_disruptor() {
        let given = vec![1, 3, 2, 4, 5];
        assert!(is_safe_tolerate_1(&given));
    }

    #[test]
    fn tolerate_remove_middle() {
        let given = vec![1, 3, 2, 4, 5];
        assert!(is_safe_tolerate_1(&given));
    }

    #[test]
    fn tolerate_remove_middle2() {
        let given = vec![1, 3, 2, 6, 7];
        assert!(is_safe_tolerate_1(&given));
    }

    #[test]
    fn tolerate_remove_middle3() {
        let given = vec![1, 5, 1, 6, 7];
        assert!(!is_safe_tolerate_1(&given));
    }

    #[test]
    fn tolerate_middle_disruption() {
        let given = vec![1, 3, 2, 4, 5];
        assert!(is_safe_tolerate_1(&given)); // Remove `2`
    }

    #[test]
    fn tolerate_start_disruption() {
        let given = vec![10, 5, 6, 7, 8];
        assert!(is_safe_tolerate_1(&given)); // Remove `10`
    }

    #[test]
    fn tolerate_end_disruption() {
        let given = vec![1, 2, 3, 10];
        assert!(is_safe_tolerate_1(&given)); // Remove `10`
    }

    #[test]
    fn tolerate_multiple_candidates() {
        let given = vec![1, 4, 2, 5, 6];
        assert!(is_safe_tolerate_1(&given)); // Remove `4` or `2`
    }

    #[test]
    fn cannot_tolerate_multiple_problems() {
        let given = vec![1, 4, 2, 10, 6];
        assert!(!is_safe_tolerate_1(&given)); // Too many problems
    }
}
