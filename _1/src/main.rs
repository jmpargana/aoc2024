use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("puzzle.txt")?;

    let buf = BufReader::new(f);

    let mut locations1 = Vec::new();
    let mut locations2 = Vec::new();

    for line in buf.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|a| a.parse::<i32>().unwrap())
            .collect();
        locations1.push(numbers[0]);
        locations2.push(numbers[1]);
    }

    let sum = sum_diff(&mut locations1, &mut locations2);
    let similarity_score = sum_similarity_score(&mut locations1, &mut locations2);

    println!("result 1 {}", sum);
    println!("result 2 {}", similarity_score);

    Ok(())
}

fn sum_diff(a: &mut [i32], b: &mut [i32]) -> i32 {
    a.sort();
    b.sort();
    a.iter().zip(b).map(|(a, b)| (*a - *b).abs()).sum()
}

fn frequency(a: &[i32]) -> HashMap<i32, i32> {
    a.iter().fold(HashMap::new(), |mut acc, curr| {
        acc.entry(*curr).and_modify(|i| *i += 1).or_insert(1);
        acc
    })
}

fn sum_similarity_score(a: &mut [i32], b: &mut [i32]) -> i32 {
    let freq = frequency(&b);
    a.iter()
        .map(|&i| i * freq.get(&i).cloned().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum() {
        let mut a = vec![3, 4, 2, 1, 3, 3];
        let mut b = vec![4, 3, 5, 3, 9, 3];
        let actual = sum_diff(&mut a, &mut b);
        let expected = 11;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_frequency() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let actual = frequency(&a);
        let expected = HashMap::from([(3, 3), (4, 1), (2, 1), (1, 1)]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_compute_similarities() {
        let mut a = vec![3, 4, 2, 1, 3, 3];
        let mut b = vec![4, 3, 5, 3, 9, 3];
        let actual = sum_similarity_score(&mut a, &mut b);
        let expected = 31;
        assert_eq!(actual, expected);
    }
}
