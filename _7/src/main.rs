use std::collections::HashSet;

use nom::{
    character::complete::{char, line_ending, space0, space1, u128},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Formula {
    target: u128,
    nums: Vec<u128>,
}

fn main() {
    let (_, formulas) = parse_file(include_str!("../puzzle.txt")).unwrap();
    let total_calibration = total_calibration(&formulas);
    println!("result 1: {}", total_calibration);
}

fn parse_file(input: &str) -> IResult<&str, Vec<Formula>> {
    separated_list1(line_ending, parse_formula)(input)
}

fn parse_formula(input: &str) -> IResult<&str, Formula> {
    map(
        separated_pair(
            u128,
            delimited(space0, char(':'), space1),
            separated_list1(space1, u128),
        ),
        |(target, nums)| Formula { target, nums },
    )(input)
}

fn has_target(target: u128, nums: &[u128]) -> bool {
    let mut dp = vec![HashSet::new(); nums.len()];

    dp[0].insert(nums[0]);

    for i in 1..nums.len() {
        for val in dp[i - 1].clone() {
            if let Some(nv) = val.checked_mul(nums[i]) {
                dp[i].insert(nv);
            }
            dp[i].insert(val + nums[i]);
        }
    }

    dp[nums.len() - 1].contains(&target)
}

fn total_calibration(arr: &[Formula]) -> u128 {
    arr.iter()
        .filter(|Formula { target, nums }| has_target(*target, nums))
        .map(|Formula { target, nums: _ }| target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_target() {
        let target = 3267;
        let nums = vec![81, 40, 27];

        let actual = has_target(target, &nums);
        assert!(actual);
    }

    #[test]
    fn should_not_find_target() {
        let target = 21037;
        let nums = vec![9, 7, 18, 13];

        let actual = has_target(target, &nums);
        assert!(!actual);
    }

    #[test]
    fn should_parse_single_line() {
        let expected = Formula {
            target: 21037,
            nums: vec![9, 7, 18, 13],
        };
        let given = "21037: 9 7 18 13";

        let (_, actual) = parse_formula(given).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_well() {
        let input = "292: 11 6 16 20
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
190: 10 19";
        let (_, actual) = parse_file(input).unwrap();
        assert_eq!(actual.len(), 9);
    }

    #[test]
    fn should_add_part1() {
        let input = "292: 11 6 16 20
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
190: 10 19";

        let (_, actual) = parse_file(input).unwrap();
        let actual = total_calibration(&actual);
        let expected = 3749;

        assert_eq!(actual, expected);
    }
}
