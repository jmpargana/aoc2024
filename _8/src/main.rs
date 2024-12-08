use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Puzzle {
    rows: usize,
    cols: usize,
    pts: HashMap<char, Vec<(i32, i32)>>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let mut cols = 0;

        for (x, line) in input.lines().enumerate() {
            for (y, ch) in line.chars().enumerate() {
                cols = line.chars().count();
                if ch != '.' {
                    nodes
                        .entry(ch)
                        .or_insert(Vec::new())
                        .push((x as i32, y as i32));
                }
            }
        }

        Puzzle {
            pts: nodes,
            rows: input.lines().count(),
            cols,
        }
    }

    fn unique_antinodes(&self) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        for pts in self.pts.values() {
            for pair in pts.iter().combinations(2) {
                let [&a, &b] = pair[..] else { continue };
                result.extend(antinodes(a, b, self.rows, self.cols));
            }
        }
        result.sort();
        result.dedup();
        result
    }

    fn count_unique_antinodes(&self) -> usize {
        self.unique_antinodes().iter().count()
    }
}

fn antinodes(a: (i32, i32), b: (i32, i32), rows: usize, cols: usize) -> Vec<(i32, i32)> {
    let delta = (b.0 - a.0, b.1 - a.1);
    let extended_back = (a.0 - delta.0, a.1 - delta.1);
    let extended_front = (b.0 + delta.0, b.1 + delta.1);
    vec![extended_back, extended_front]
        .into_iter()
        .filter(|&pt| !is_within_bounds(pt, rows, cols))
        .collect()
}

fn is_within_bounds(pt: (i32, i32), rows: usize, cols: usize) -> bool {
    pt.0 < 0 || pt.1 < 0 || pt.0 as usize >= rows || pt.1 as usize >= cols
}

fn main() {
    let map = include_str!("../puzzle.txt");
    let puzzle = Puzzle::new(map);
    let count = puzzle.count_unique_antinodes();
    println!("result 1: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_antinotes() {
        let (a, b) = ((3, 4), (5, 5));
        let expected = vec![(1, 3), (7, 6)];
        let actual = antinodes(a, b, 10, 10);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_find_with_boundaries() {
        let (a, b) = ((1, 3), (3, 4));
        let expected = vec![(5, 5)];
        let actual = antinodes(a, b, 10, 10);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_parse_map() {
        let input = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

        let expected = vec![(1, 3), (7, 6)];
        let actual = Puzzle::new(input).unique_antinodes();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_count_example_1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let expected = 14;
        let actual = Puzzle::new(input).count_unique_antinodes();
        assert_eq!(expected, actual);
    }
}
