use std::{error::Error, fs::File, io::Read};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::open("puzzle.txt")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf);

    let sum_of_multiples = sum_mult(matched_mul(&buf));
    let sum_of_enabled_multiples = sum_mult_parsed_enabled(&buf);

    println!("result 1: {}", sum_of_multiples);
    println!("result 2: {}", sum_of_enabled_multiples);

    Ok(())
}

fn matched_enabled_mul(s: &str) -> String {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let mut enabled = true;
    let mut cleaned_muls = String::new();

    for cap in re.find_iter(s) {
        let token = cap.as_str();

        match token {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    cleaned_muls.push_str(token);
                }
            }
        }
    }
    cleaned_muls
}

fn sum_mult_parsed_enabled(s: &str) -> u32 {
    sum_mult(matched_mul(&matched_enabled_mul(&s)))
}

fn matched_mul(s: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(s)
        .map(|cap| {
            let a = cap[1].parse::<u32>().unwrap();
            let b = cap[2].parse::<u32>().unwrap();
            (a, b)
        })
        .collect()
}

fn sum_mult(v: Vec<(u32, u32)>) -> u32 {
    v.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_only_valid() {
        let given = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let actual = matched_mul(&given);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_match_context() {
        let given = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = "mul(2,4)mul(8,5)".to_string();
        let actual = matched_enabled_mul(&given);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_calculate_only_enabled() {
        let given = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = 48;
        let actual = sum_mult_parsed_enabled(&given);
        assert_eq!(actual, expected);
    }
}
