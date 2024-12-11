use std::collections::HashMap;

fn count_n_iterations(n: usize, start: &Vec<usize>) -> usize {
    start.iter().map(|&a| count_n_iterations_cell(n, a)).sum()
}

fn count_n_iterations_cell(n: usize, start: usize) -> usize {
    // dynamic programming
    let mut buckets: HashMap<usize, usize> = HashMap::new();
    buckets.insert(start, 1);

    for _ in 0..n {
        let mut next_buckets: HashMap<usize, usize> = HashMap::new();

        for (&digits, &count) in &buckets {
            let next = next_cell(digits as u128);
            for &n in &next {
                *next_buckets.entry(n as usize).or_insert(0) += count;
            }
        }

        buckets = next_buckets;
    }

    buckets.values().sum()
}

fn next_cell(a: u128) -> Vec<u128> {
    match a {
        0 => vec![1],
        x if has_even_digits(x) => split_n(x as u64),
        x => vec![multiply_by_2024(x)],
    }
}

fn has_even_digits(n: u128) -> bool {
    let mut n = n.clone();
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits % 2 == 0
}

fn split_n(n: u64) -> Vec<u128> {
    let digits = ((n as f64).log10().floor() as u64) + 1;

    let half = digits / 2;
    let divisor = 10u64.pow(half as u32);

    let a = n / divisor;
    let b = n % divisor;
    vec![a as u128, b as u128]
}

fn multiply_by_2024(x: u128) -> u128 {
    (x << 11) - (x << 4) - (x << 3)
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let input = include_str!("../puzzle.txt");
    let list = parse(input);
    let count = count_n_iterations(25, &list);
    let count_long = count_n_iterations(75, &list);
    println!("result 1: {}", count);
    println!("result 2: {}", count_long);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_multiply_by_2024_correctly() {
        let given = 827364;
        assert_eq!(multiply_by_2024(given), given * 2024);
    }

    #[test]
    fn should_recognise_even_number() {
        let given = 827364;
        assert!(has_even_digits(given));
    }

    #[test]
    fn should_recognise_odd_number() {
        let given = 82736;
        assert!(!has_even_digits(given));
    }

    #[test]
    fn should_split_correct() {
        let given = 1000;
        assert_eq!(split_n(given), vec![10, 0]);
    }

    #[test]
    fn should_count_correctly() {
        let given = vec![0];
        let iterations = 3;
        let expected = vec![20, 24].iter().count();
        let actual = count_n_iterations(iterations, &given);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_count_correctly_example1() {
        let given = vec![125, 17];
        let iterations = 6;
        let expected = 22;
        let actual = count_n_iterations(iterations, &given);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_count_correctly_example1_more() {
        let given = vec![125, 17];
        let iterations = 25;
        let expected = 55312;
        let actual = count_n_iterations(iterations, &given);
        assert_eq!(actual, expected);
    }
}
